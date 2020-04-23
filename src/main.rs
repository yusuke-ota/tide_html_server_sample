use tide;
use async_std::fs::File;
use async_std::io::BufReader;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let mut app = tide::new();
    // same code
    // app.at("/").get(|_| async move{ "hello".to_string()};
    app.at("/").get(hello);

    // same code
    // app.at("/hello").get( |_| async move {
    //     let file = File::open("hello.html").await.unwrap();
    //     let reader = BufReader::new(file);
    //     tide::Response::new(200)
    //         .body(reader)
    //         .set_mime(mime::TEXT_HTML)
    // });
    app.at("/hello").get(hello_html);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn hello(_req: tide::Request<()>) -> String {
    "hello".to_string()
}

async fn hello_html(_req: tide::Request<()>) -> tide::Response{
    let file = File::open("hello.html").await.unwrap();
    let reader = BufReader::new(file);
    tide::Response::new(200)
        .body(reader)
        //　MIMEについてはこちらのサイト参照
        // https://developer.mozilla.org/ja/docs/Web/HTTP/Basics_of_HTTP/MIME_types
        .set_mime(mime::TEXT_HTML)
}