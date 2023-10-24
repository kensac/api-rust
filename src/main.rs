pub mod database;
#[allow(warnings)]
pub mod prisma;
pub mod routes;
pub mod utils;

use axum::routing::get;
use axum::Router;


#[tokio::main]
async fn main() {

    let sponsor_routes = routes::sponsor::sponosor_get_router().await;

    let app = Router::new()
        .route("/", get(utils::hello_world))
        .nest("/sponsor", sponsor_routes)
        // .route("/test", get(server_side_auth))
        .fallback(get(utils::handler_404));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/* #[axum::debug_handler]
pub async fn server_side_auth() -> Json<()>{

    dotenv::dotenv().ok();

    let appwrite_secret = std::env::var("APPWRITE_SECRET").unwrap();
    let appwrite_id = std::env::var("APPWRITE_ID").unwrap();

    let client = reqwest::Client::new();

    let url = format!(
        "https://cloud.appwrite.io/v1/databases/{}/collections/{}/documents","test","sample"
    );
    let response = client
        .get(&url)
        .header("X-Appwrite-Project", appwrite_id)
        .header("X-Appwrite-Key", appwrite_secret)
        .send().await;

    println!("{:?}", response.unwrap().text().await.unwrap());
    Json(())

} */
