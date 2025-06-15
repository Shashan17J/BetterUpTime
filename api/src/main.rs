use poem::{get, handler, listener::TcpListener, web::Path, Route, Server};

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("Hello:{}", name)
}

#[handler]
fn root() -> String {
    format!("Server is Up and Running")
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
    .at("/", get(root))
    .at("/hello/:name", get(hello));
    // creates and run the http server
    Server::new(TcpListener::bind("0.0.0.0:3000"))
    .run(app)
    .await
} 