use tide::http::{mime, StatusCode};
use tide::Body;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let mut app = tide::new();

    app.at("/").get(hello);
    app.at("/hello_html").get(hello_html);
    app.at("/html_template/:name").get(hello_html_from_template);
    app.at("/*").get(not_found);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn hello(_req: tide::Request<()>) -> Result<String, tide::Error> {
    Ok("hello".to_string())
}

async fn hello_html(_req: tide::Request<()>) -> Result<tide::Response, tide::Error> {
    let mut res = tide::Response::new(StatusCode::Ok);
    let body = Body::from_file("hello.html").await?;
    res.set_body(body);
    Ok(res)
}

async fn not_found(_req: tide::Request<()>) -> Result<tide::Response, tide::Error> {
    let mut res = tide::Response::new(StatusCode::NotFound);
    let body = Body::from_string("Not Found".to_string());
    res.set_body(body);
    Ok(res)
}

async fn hello_html_from_template(req: tide::Request<()>) -> Result<tide::Response, tide::Error> {
    let mut res = tide::Response::new(StatusCode::Ok);

    // 'tide::Server.at(*/:name)'の形でルーティングしている場合に、:nameの値を取得する
    // 'tide::Server.at(*/:name)' when routing in the form Get the value of :name.
    // :nameがある場合、&str.parse::<String> (= &str.to_string())は必ず成功する。
    // When ':name' is exist, '&str.parse::<String>' ('= &str. to_string()') always succeed.
    let name = req.param::<String>("name")?;

    // Endpointにerrorが伝播するとResponse::new(StatusCode::InternalServerError)が返る
    // When the error is propagated to Endpoint, Response::new( StatusCode::InternalServerError) is returned.
    let html_body = generate_html(name).unwrap();
    let body = Body::from_string(html_body);
    res.set_body(body);
    res.set_content_type(mime::HTML);
    Ok(res)
}

// htmlをテンプレートエンジンで生成します。
// Generate html with the template engine.
// この関数を利用するために、'serde' and 'tinytemplate'クレートが必要です。
// In order to use this function, the 'serde' and 'tinytemplate' crates are added to 'extern crate'.
use serde::Serialize;
use std::error::Error;
use tinytemplate::TinyTemplate;

#[derive(Serialize)]
struct Context {
    name: String,
}

const HTML_TEMPLATE: &'static str = "<!DOCTYPE html>
    <html lang=\"en\">
        <head>
            <meta charset=\"UTF-8\">
            <title>Hello!</title>
        </head>
        <body>
            <h1>Hello!</h1>
            <p>Hi {name} from Rust</p>
        </body>
    </html>";

pub fn generate_html(name: String) -> Result<String, Box<dyn Error>> {
    let mut template = TinyTemplate::new();
    template.add_template("hello_rust_template", HTML_TEMPLATE)?;

    let context = Context { name };

    let rendered = template.render("hello_rust_template", &context)?;
    Ok(rendered)
}
