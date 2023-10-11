pub mod database;
#[allow(warnings)]
pub mod prisma;
pub mod routes;
pub mod utils;

use axum::routing::{get, post};
use axum::{Router, Json};

use prisma::PrismaClient;

use routes::user_routes::{create_user_route, get_first_user_route, get_user_by_id_route};

#[derive(Clone)]
pub struct AppState {
    client: PrismaClient,
}

#[tokio::main]
async fn main() {
    // let client = PrismaClient::_builder()
    //     .build()
    //     .await
    //     .expect("Didn't connect to database");
    //let state = AppState { client: client };
    let app = Router::new()
        // .route("/users/", post(create_user_route).get(get_first_user_route))
        // .route("/users/:user_id", get(get_user_by_id_route))
        .route("/", get(utils::hello_world))
        .route("/test", get(server_side_auth))
        //.with_state(state)
        .fallback(get(utils::handler_404));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[axum::debug_handler]
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

}
