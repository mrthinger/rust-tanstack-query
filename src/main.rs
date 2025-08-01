// Replace some of the `axum::` types with `aide::axum::` ones.
use aide::{
    axum::{
        ApiRouter, IntoApiResponse,
        routing::{get, post},
    },
    openapi::{Info, OpenApi},
};
use axum::{Extension, Json};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(JsonSchema, Debug, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(JsonSchema, Debug, Serialize, Deserialize)]
pub struct Trade {
    side: Side,
    levels: Vec<f64>,
}

#[derive(JsonSchema, Debug, Serialize)]
pub struct TradeRef {
    side: Side,
    levels: [f64],
}


// We'll need to derive `JsonSchema` for
// all types that appear in the api documentation.
#[derive(Deserialize, JsonSchema)]
struct User {
    name: String,
}

async fn hello_user(Json(user): Json<User>) -> impl IntoApiResponse {
    Json(format!("hello {}", user.name))
}

async fn get_trade() -> impl IntoApiResponse {
    let trade_ref: &TradeRef = todo!();
    Json(trade_ref)
}

// Note that this clones the document on each request.
// To be more efficient, we could wrap it into an Arc,
// or even store it as a serialized string.
async fn serve_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}

#[tokio::main]
async fn main() {
    let app = ApiRouter::new()
        // Change `route` to `api_route` for the route
        // we'd like to expose in the documentation.
        .api_route("/hello", post(hello_user))
        .api_route("/trade", get(get_trade))
        // We'll serve our generated document here.
        .route("/api.json", get(serve_api));

    let mut api = OpenApi {
        info: Info {
            description: Some("an example API".to_string()),
            ..Info::default()
        },
        ..OpenApi::default()
    };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(
        listener,
        app
            // Generate the documentation.
            .finish_api(&mut api)
            // Expose the documentation to the handlers.
            .layer(Extension(api))
            .into_make_service(),
    )
    .await
    .unwrap();
}
