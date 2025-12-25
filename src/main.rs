use axum::{
    Router,
    extract::{Path, Query, State},
    http::{StatusCode, Uri, header},
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
};
use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper_util::{
    client::legacy::{Client, connect::HttpConnector},
    rt::TokioExecutor,
};
use serde::Deserialize;
use std::sync::Arc;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::{Level, debug, info};
use std::env;

use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};

type AppState = Client<HttpsConnector<HttpConnector>, Full<Bytes>>;

const MAX_REDIRECTS: usize = 10;

#[derive(Deserialize)]
struct DownloadQuery {
    url: String,
}

#[derive(Deserialize)]
struct DirectPath {
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载 .env 文件
    dotenv::dotenv().ok();
    
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let https = HttpsConnectorBuilder::new()
        .with_provider_and_webpki_roots(Arc::new(rustls::crypto::ring::default_provider()))?
        .https_or_http()
        .enable_http1()
        .enable_http2()
        .build();

    let client: AppState = Client::builder(TokioExecutor::new()).build(https);

    let app = Router::new()
        .route("/", get(index))
        .route("/download", get(download_handler))
        .route("/gh/*url", get(proxy_direct))
        .with_state(client)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::DEBUG)),
        );

    // 从环境变量读取端口，默认 3000
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);
    
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Listening on http://{}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn index() -> Html<&'static str> {
    Html(include!(concat!(env!("OUT_DIR"), "/html_constants.rs")))
}

//
// GET /download?url=...
//
async fn download_handler(Query(q): Query<DownloadQuery>) -> Response {
    let input = q.url.trim();

    if input.is_empty() {
        return (StatusCode::BAD_REQUEST, "Empty URL").into_response();
    }

    Redirect::temporary(&format!("/gh/{}", input)).into_response()
}

//
// GitHub 直连代理（支持重定向）
//
async fn proxy_direct(
    Path(DirectPath { url }): Path<DirectPath>,
    State(client): State<AppState>,
) -> Response {
    let mut current = if url.starts_with("http://") || url.starts_with("https://") {
        url
    } else {
        format!("https://{}", url)
    };

    // 超过限制则报错
    for _ in 0..MAX_REDIRECTS {
        debug!("Proxy → {}", current);

        let uri: Uri = match current.parse() {
            Ok(u) => u,
            Err(_) => return (StatusCode::BAD_REQUEST, "Invalid URL").into_response(),
        };

        //
        // SSRF 防护（GitHub allowlist）
        //
        let allowed_hosts = [
            "github.com",
            "raw.githubusercontent.com",
            "codeload.github.com",
            "release-assets.githubusercontent.com",
        ];

        match uri.host() {
            Some(host) if allowed_hosts.iter().any(|h| host.ends_with(h)) => {}
            _ => return (StatusCode::FORBIDDEN, "Host not allowed").into_response(),
        }

        let req = hyper::Request::builder()
            .uri(uri.clone())
            .header(
                header::USER_AGENT,
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64)",
            )
            .header(header::ACCEPT, "*/*")
            .body(Full::new(Bytes::new()))
            .unwrap();

        let resp: hyper::Response<Incoming> = match client.request(req).await {
            Ok(r) => r,
            Err(_) => return (StatusCode::BAD_GATEWAY, "Upstream error").into_response(),
        };

        let status = resp.status();

        // 检测 301 / 302 / 303 / 307 / 308
        if matches!(
            status,
            StatusCode::MOVED_PERMANENTLY
                | StatusCode::FOUND
                | StatusCode::SEE_OTHER
                | StatusCode::TEMPORARY_REDIRECT
                | StatusCode::PERMANENT_REDIRECT
        ) {
            if let Some(loc) = resp.headers().get(header::LOCATION) {
                let loc = match loc.to_str() {
                    Ok(v) => v,
                    Err(_) => {
                        return (StatusCode::BAD_GATEWAY, "Bad Location header").into_response();
                    }
                };

                // 处理相对 Location
                current = match resolve_location(&current, loc) {
                    Ok(u) => u,
                    Err(_) => {
                        return (StatusCode::BAD_GATEWAY, "Invalid redirect URL").into_response();
                    }
                };

                debug!("Redirect → {}", current);
                continue;
            } else {
                return (StatusCode::BAD_GATEWAY, "Redirect without Location").into_response();
            }
        }

        // 非重定向，直接返回（200 / 403 / 404 等）
        let (parts, body) = resp.into_parts();
        return hyper::Response::from_parts(parts, body).into_response();
    }

    (StatusCode::BAD_GATEWAY, "Too many redirects").into_response()
}

//
// 处理相对 / 绝对 Location
//
fn resolve_location(base: &str, location: &str) -> Result<String, ()> {
    let base = url::Url::parse(base).map_err(|_| ())?;
    let new = base.join(location).map_err(|_| ())?;
    Ok(new.to_string())
}
