use axum::{routing::get, Json, Router};
use serde::Serialize;
use std::net::SocketAddr;
use tower_http::services::{ServeDir, ServeFile};
use tracing_subscriber::FmtSubscriber;

#[derive(Serialize, Clone)]
struct Product {
    id: &'static str,
    name: &'static str,
    price: u32,
    category: &'static str,
    image: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    hover_image: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<&'static str>,
}

/// Product photos live in `static/images/`.
/// Send replacements named like `p1.jpg` … `p8.jpg` (and optional `p1-hover.jpg`).
fn catalog() -> Vec<Product> {
    vec![
        Product {
            id: "blood-bow-mini",
            name: "Blood Bow Mini",
            price: 78,
            category: "Dresses",
            image: "/static/images/p1.jpg",
            hover_image: Some("/static/images/p1-hover.jpg"),
            tag: Some("Just in"),
        },
        Product {
            id: "ribcage-corset",
            name: "Ribcage Corset",
            price: 64,
            category: "Tops",
            image: "/static/images/p2.jpg",
            hover_image: Some("/static/images/p2-hover.jpg"),
            tag: None,
        },
        Product {
            id: "spiderweb-skirt",
            name: "Spiderweb Skirt",
            price: 72,
            category: "Bottoms",
            image: "/static/images/p3.jpg",
            hover_image: Some("/static/images/p3-hover.jpg"),
            tag: Some("Favorite"),
        },
        Product {
            id: "funeral-velvet",
            name: "Funeral Velvet Coat",
            price: 128,
            category: "Outerwear",
            image: "/static/images/p4.jpg",
            hover_image: Some("/static/images/p4-hover.jpg"),
            tag: None,
        },
        Product {
            id: "hex-girl-tee",
            name: "Hex Girl Tee",
            price: 42,
            category: "Tops",
            image: "/static/images/p5.jpg",
            hover_image: Some("/static/images/p5-hover.jpg"),
            tag: Some("Just in"),
        },
        Product {
            id: "cemetery-set",
            name: "Cemetery Picnic Set",
            price: 118,
            category: "Sets",
            image: "/static/images/p6.jpg",
            hover_image: Some("/static/images/p6-hover.jpg"),
            tag: None,
        },
        Product {
            id: "graveyard-platforms",
            name: "Graveyard Platforms",
            price: 148,
            category: "Shoes",
            image: "/static/images/p7.jpg",
            hover_image: Some("/static/images/p7-hover.jpg"),
            tag: None,
        },
        Product {
            id: "rotten-sugar-hoodie",
            name: "Rotten Sugar Hoodie",
            price: 88,
            category: "Tops",
            image: "/static/images/p8.jpg",
            hover_image: Some("/static/images/p8-hover.jpg"),
            tag: None,
        },
    ]
}

async fn products() -> Json<Vec<Product>> {
    Json(catalog())
}

async fn health() -> &'static str {
    "sugah3x ok"
}

#[tokio::main]
async fn main() {
    FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .init();

    let app = Router::new()
        .route("/api/products", get(products))
        .route("/health", get(health))
        .route_service("/", ServeFile::new("static/index.html"))
        .nest_service("/static", ServeDir::new("static"));

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("SugaH3x rust shop listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");
    axum::serve(listener, app).await.expect("serve");
}
