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
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    packshot: bool,
}

/// Product photos live in `static/images/`.
fn catalog() -> Vec<Product> {
    vec![
        Product {
            id: "dont-be-nice-crop",
            name: "Don't Be Nice Crop Tee",
            price: 48,
            category: "Tops",
            image: "/static/images/dont-be-nice-front.jpg",
            hover_image: Some("/static/images/dont-be-nice-back.jpg"),
            tag: Some("Just in"),
            packshot: true,
        },
        Product {
            id: "savage-sweats",
            name: "Savage Sweats",
            price: 68,
            category: "Bottoms",
            image: "/static/images/savage-sweats.png",
            hover_image: None,
            tag: Some("Just in"),
            packshot: true,
        },
        Product {
            id: "grave-picnic-set",
            name: "Grave Picnic Set",
            price: 128,
            category: "Sets",
            image: "/static/images/grave-picnic.jpg",
            hover_image: None,
            tag: Some("Just in"),
            packshot: false,
        },
        Product {
            id: "blush-corset",
            name: "Blush Lace Corset",
            price: 88,
            category: "Tops",
            image: "/static/images/blush-corset.jpg",
            hover_image: None,
            tag: Some("Just in"),
            packshot: false,
        },
        Product {
            id: "bow-baby-dress",
            name: "Bow Baby Dress",
            price: 98,
            category: "Dresses",
            image: "/static/images/bow-baby-dress.jpg",
            hover_image: None,
            tag: Some("Just in"),
            packshot: false,
        },
        Product {
            id: "chrome-cargo-set",
            name: "Chrome Cargo Set",
            price: 118,
            category: "Sets",
            image: "/static/images/chrome-cargo.jpg",
            hover_image: None,
            tag: None,
            packshot: false,
        },
        Product {
            id: "static-hex-set",
            name: "Static Hex Set",
            price: 108,
            category: "Sets",
            image: "/static/images/static-hex.jpg",
            hover_image: None,
            tag: None,
            packshot: false,
        },
        Product {
            id: "bow-bite-set",
            name: "Bow & Bite Set",
            price: 54,
            category: "Lingerie",
            image: "/static/images/bow-bite.jpg",
            hover_image: None,
            tag: None,
            packshot: false,
        },
        Product {
            id: "widow-robe",
            name: "Widow Lace Robe",
            price: 78,
            category: "Outerwear",
            image: "/static/images/widow-robe.jpg",
            hover_image: None,
            tag: None,
            packshot: false,
        },
        Product {
            id: "black-widow-robe",
            name: "Black Widow Robe",
            price: 88,
            category: "Outerwear",
            image: "/static/images/black-widow-robe.jpg",
            hover_image: None,
            tag: Some("Just in"),
            packshot: false,
        },
        Product {
            id: "blush-slip",
            name: "Blush Slip Robe",
            price: 62,
            category: "Lingerie",
            image: "/static/images/blush-slip.jpg",
            hover_image: None,
            tag: Some("Just in"),
            packshot: false,
        },
        Product {
            id: "pink-hex-cargo",
            name: "Pink Hex Cargo Set",
            price: 118,
            category: "Sets",
            image: "/static/images/pink-hex-cargo.jpg",
            hover_image: None,
            tag: Some("Just in"),
            packshot: false,
        },
        Product {
            id: "patent-ritual-set",
            name: "Patent Ritual Set",
            price: 148,
            category: "Sets",
            image: "/static/images/patent-ritual.jpg",
            hover_image: None,
            tag: None,
            packshot: false,
        },
        Product {
            id: "shoulder-bow-dress",
            name: "Shoulder Bow Dress",
            price: 108,
            category: "Dresses",
            image: "/static/images/shoulder-bow-dress.jpg",
            hover_image: None,
            tag: None,
            packshot: false,
        },
        Product {
            id: "dusty-bow-picnic",
            name: "Dusty Bow Picnic Set",
            price: 128,
            category: "Sets",
            image: "/static/images/dusty-bow-picnic.jpg",
            hover_image: None,
            tag: None,
            packshot: false,
        },
        Product {
            id: "velvet-bow-corset",
            name: "Velvet Bow Corset",
            price: 92,
            category: "Tops",
            image: "/static/images/velvet-bow-corset.jpg",
            hover_image: None,
            tag: None,
            packshot: false,
        },
        Product {
            id: "black-bow-baby",
            name: "Black Bow Baby",
            price: 98,
            category: "Dresses",
            image: "/static/images/black-bow-baby.jpg",
            hover_image: None,
            tag: None,
            packshot: false,
        },
        Product {
            id: "cross-heart-set",
            name: "Cross Heart Set",
            price: 58,
            category: "Lingerie",
            image: "/static/images/cross-heart-set.jpg",
            hover_image: None,
            tag: None,
            packshot: false,
        },
        Product {
            id: "pink-ribbon-corset",
            name: "Pink Ribbon Corset",
            price: 88,
            category: "Tops",
            image: "/static/images/pink-ribbon-corset.jpg",
            hover_image: None,
            tag: None,
            packshot: false,
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
