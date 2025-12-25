use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // 读取 HTML 模板文件
    let html_path = Path::new("templates").join("index.html");
    if html_path.exists() {
        let html_content = fs::read_to_string(&html_path)
            .expect("Failed to read HTML template file");
        
        // 创建一个包含 HTML 内容的常量
        let out_dir = env::var("OUT_DIR").unwrap();
        let html_const_path = Path::new(&out_dir).join("html_constants.rs");
        
        let escaped_html = html_content
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t");
        
        let html_const_content = format!(
            "\"{}\"",
            escaped_html
        );
        
        fs::write(&html_const_path, html_const_content)
            .expect("Failed to write HTML constants file");
        
        // 告诉 Cargo 在模板文件更改时重新运行构建脚本
        println!("cargo:rerun-if-changed=templates/index.html");
    } else {
        eprintln!("Warning: templates/index.html not found, using default HTML");
        
        // 创建默认的 HTML 内容
        let default_html = r#"<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<title>GitHub Proxy</title>
</head>
<body>
<h2>GitHub Downloader</h2>
<form method="get" action="/download">
<input type="url" name="url" size="80" required>
<button type="submit">Download</button>
</form>
</body>
</html>"#;
        
        let out_dir = env::var("OUT_DIR").unwrap();
        let html_const_path = Path::new(&out_dir).join("html_constants.rs");
        
        let escaped_default_html = default_html
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t");
        
        let html_const_content = format!(
            "\"{}\"",
            escaped_default_html
        );
        
        fs::write(&html_const_path, html_const_content)
            .expect("Failed to write HTML constants file");
    }
    
    println!("cargo:rerun-if-changed=build.rs");
}
