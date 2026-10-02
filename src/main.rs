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
            id: "savage-tee",
            name: "Savage Tee",
            price: 48,
            category: "Tops",
            image: "/static/images/savage-tee-front.jpg",
            hover_image: Some("/static/images/savage-tee-back.jpg"),
            tag: Some("Just in"),
            packshot: true,
        },
        Product {
            id: "crafts-tee",
            name: "Girls Just Love Crafts Tee",
            price: 48,
            category: "Tops",
            image: "/static/images/crafts-tee-front.jpg",
            hover_image: Some("/static/images/crafts-tee-back.jpg"),
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
            id: "bow-cargo-pants",
            name: "Bow Cargo Pants",
            price: 78,
            category: "Bottoms",
            image: "/static/images/bow-cargo-pants.jpg",
            hover_image: None,
            tag: Some("Just in"),
            packshot: true,
        },
        Product {
            id: "lace-hem-romper",
            name: "Lace Hem Romper",
            price: 72,
            category: "Dresses",
            image: "/static/images/lace-hem-romper.jpg",
            hover_image: None,
            tag: Some("Just in"),
            packshot: true,
        },
        Product {
            id: "patent-rave-set",
            name: "Patent Rave Set",
            price: 148,
            category: "Rave",
            image: "/static/images/patent-rave-set.jpg",
            hover_image: None,
            tag: Some("Just in"),
            packshot: true,
        },
        Product {
            id: "vinyl-bow-rave",
            name: "Vinyl Bow Rave Set",
            price: 128,
            category: "Rave",
            image: "/static/images/vinyl-bow-rave.jpg",
            hover_image: None,
            tag: Some("Just in"),
            packshot: true,
        },
        Product {
            id: "afters-robe-rave",
            name: "Afters Robe Set",
            price: 138,
            category: "Rave",
            image: "/static/images/afters-robe-rave.jpg",
            hover_image: None,
            tag: Some("Just in"),
            packshot: true,
        },
        Product {
            id: "wrap-mini-rave",
            name: "Wrap Mini Rave Set",
            price: 88,
            category: "Rave",
            image: "/static/images/wrap-mini-rave.jpg",
            hover_image: None,
            tag: Some("Just in"),
            packshot: true,
        },
        Product {
            id: "blush-afters-set",
            name: "Blush Afters Set",
            price: 98,
            category: "Lingerie",
            image: "/static/images/blush-afters-set.jpg",
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
        Product {
            id: "pink-maryjane-platforms",
            name: "Pink Mary Jane Platforms",
            price: 128,
            category: "Shoes",
            image: "/static/images/shoe-pink-maryjane.jpg",
            hover_image: None,
            tag: Some("Just in"),
            packshot: false,
        },
        Product {
            id: "patent-chunk-platforms",
            name: "Patent Chunk Platforms",
            price: 148,
            category: "Shoes",
            image: "/static/images/shoe-patent-platforms.jpg",
            hover_image: None,
            tag: None,
            packshot: false,
        },
        Product {
            id: "ribbon-ballet-flats",
            name: "Ribbon Ballet Flats",
            price: 88,
            category: "Shoes",
            image: "/static/images/shoe-ribbon-ballet.jpg",
            hover_image: None,
            tag: None,
            packshot: false,
        },
        Product {
            id: "strappy-pink-hardware",
            name: "Strappy Pink Hardware",
            price: 118,
            category: "Shoes",
            image: "/static/images/shoe-strappy-heels.jpg",
            hover_image: None,
            tag: None,
            packshot: false,
        },
        Product {
            id: "blush-platform-boots",
            name: "Blush Platform Boots",
            price: 168,
            category: "Shoes",
            image: "/static/images/shoe-pink-demonia.jpg",
            hover_image: None,
            tag: Some("Just in"),
            packshot: false,
        },
        Product {
            id: "sticker-pack",
            name: "Sticker Pack",
            price: 12,
            category: "Stickers",
            image: "/static/stickers/pack-sheet.png",
            hover_image: Some("/static/stickers/wordmark.png"),
            tag: Some("New"),
            packshot: true,
        },
        Product {
            id: "sticker-pretty-poison",
            name: "Poison Sticker",
            price: 4,
            category: "Stickers",
            image: "/static/stickers/pretty-poison.png",
            hover_image: None,
            tag: None,
            packshot: true,
        },
        Product {
            id: "sticker-hex-bow",
            name: "Bow Sticker",
            price: 4,
            category: "Stickers",
            image: "/static/stickers/hex-bow.png",
            hover_image: None,
            tag: None,
            packshot: true,
        },
        Product {
            id: "sticker-hex-kitty",
            name: "Cat Sticker",
            price: 4,
            category: "Stickers",
            image: "/static/stickers/hex-kitty.png",
            hover_image: None,
            tag: None,
            packshot: true,
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
