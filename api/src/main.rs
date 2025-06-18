use poem::{get, handler, listener::TcpListener, post, web::{Json, Path}, Route, Server};
use store::Store;
pub mod request_inputs;
pub mod request_outputs;

use request_inputs::CreateWebsiteInput;
use request_outputs::CreateWebsiteOutput;

#[handler]
fn root() -> String {
    format!("Server is Up and Running")
}

#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    // let s = Store{};
    let id = s.create_website();
    let response = CreateWebsiteOutput {
        id: id
    };
    Json(response)
}

#[handler]
fn get_website(Path(id): Path<String>) -> String {
    format!("")
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
    .at("/", get(root))
    .at("/website", post(create_website));
    // creates and run the http server
    Server::new(TcpListener::bind("0.0.0.0:3000"))
    .run(app)
    .await
}