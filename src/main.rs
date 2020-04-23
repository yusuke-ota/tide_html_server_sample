use tide;
use async_std::fs::File;
use async_std::io::BufReader;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let mut app = tide::new();
    app.at("/").get(|_| async move {"hello"});
    app.at("/hello").get( |_| async move {
        let file = File::open("hello.html").await.unwrap();
        let reader = BufReader::new(file);
        tide::Response::new(200)
            .body(reader)
            .set_mime(mime::TEXT_HTML)
    });
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
