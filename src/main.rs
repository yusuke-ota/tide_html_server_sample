use async_std::fs::File;
use async_std::io::BufReader;
use http_types::StatusCode;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let mut app = tide::new();
    // 同じコード / same code
    // app.at("/").get(|_| async move{ Ok("hello".to_string())});
    app.at("/").get(hello);

    // 同じコード / same code
    // app.at("/hello").get( |_| async move {
    //     let file = File::open("hello.html").await.unwrap();
    //     let reader = BufReader::new(file);
    //     let res = tide::Response::new(StatusCode::Ok)
    //         .body(reader)
    //         .set_mime(mime::TEXT_HTML);
    //     Ok(res)
    // });
    app.at("/hello").get(hello_html);

    app.at("/*").get(not_found);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn hello(_req: tide::Request<()>) -> Result<String, tide::Error> {
    Ok("hello".to_string())
}

async fn hello_html(_req: tide::Request<()>) -> Result<tide::Response, tide::Error>{
    let file = File::open("hello.html").await.unwrap();
    let reader = BufReader::new(file);
    let res = tide::Response::new(StatusCode::Ok)
        .body(reader)
        // MIMEについてはこちらのサイト参照About MIME, Read here.
        // https://developer.mozilla.org/ja/docs/Web/HTTP/Basics_of_HTTP/MIME_types
        // About MIME, Read here.
        // https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types
        .set_mime(mime::TEXT_HTML);
    Ok(res)
}

async fn not_found(_req: tide::Request<()>) -> Result<tide::Response, tide::Error>{
    let res = tide::Response::new(StatusCode::NotFound)
        .body_string("Not Found".to_string());
    Ok(res)
}