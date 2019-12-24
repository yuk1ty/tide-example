#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let mut app = tide::new();
    app.at("/hc")
        .get(|_| async move { tide::Response::new(200) });
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
