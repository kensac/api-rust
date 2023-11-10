pub mod database;
#[allow(warnings)]
pub mod prisma;
pub mod routes;
pub mod utils;

use axum::routing::get;
use axum::{Json, Router};
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            utils::health_check,
            routes::hackathons::create_hackathon,
            routes::hackathons::get_hackathon,
            routes::hackathons::get_hackathon_by_id,
            routes::hackathons::delete_hackathon_by_id,
            routes::hackathons::set_active_hackathon
        ),
        components(schemas(routes::hackathons::CreateHackathonEntity))
    )]
    struct ApiDoc;

    let sponsor_routes = routes::sponsors::sponsor_get_router().await;
    let hackathon_routes = routes::hackathons::hackathon_get_router().await;

    let app = Router::new()
        .route("/", get(utils::hello_world))
        .route("/health", get(utils::health_check))
        .nest("/sponsor", sponsor_routes)
        .nest("/hackathon", hackathon_routes)
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .route("/test", get(server_side_auth));
    //.fallback(get(utils::handle_404));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[axum::debug_handler]
pub async fn server_side_auth() -> Json<()> {
    dotenv::dotenv().ok();

    let appwrite_secret = std::env::var("APPWRITE_SECRET").unwrap();
    let appwrite_id = std::env::var("APPWRITE_ID").unwrap();

    let client = reqwest::Client::new();

    let url = format!(
        "https://cloud.appwrite.io/v1/databases/{}/collections/{}/documents",
        "test", "sample"
    );
    let response = client
        .get(&url)
        .header("X-Appwrite-Project", appwrite_id)
        .header("X-Appwrite-Key", appwrite_secret)
        .send()
        .await;

    println!("{:?}", response.unwrap().text().await.unwrap());
    Json(())
}
