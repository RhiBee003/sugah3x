# SugaH3x

Girl-owned alt clothing shop — Rust (Axum) backend + static storefront.

## Run on your computer

**Needs:** [Rust](https://rustup.rs/) (stable)

```bash
git clone https://github.com/RhiBee003/sugah3x.git
cd sugah3x
cargo run --release
```

Open [http://127.0.0.1:3000](http://127.0.0.1:3000)

Optional port:

```bash
PORT=8080 cargo run --release
```

## Deploy on Render

This is a **Rust web service**, not a static site and not Node.

1. In [Render](https://dashboard.render.com/), create a **Web Service** (not Static Site).
2. Connect `RhiBee003/sugah3x`, branch `main`.
3. Set:

| Field | Value |
| --- | --- |
| Language | **Rust** |
| Build Command | `cargo build --release --locked` |
| Start Command | `./target/release/sugarhex-rs` |
| Health Check Path | `/health` |

Or open [Deploy to Render](https://render.com/deploy?repo=https://github.com/RhiBee003/sugah3x) and apply the `render.yaml` Blueprint.

The first build takes a few minutes. If the last deploy failed with `npm` / `package.json` errors, the service was created as Node — change Language to **Rust** and use the commands above, then **Manual Deploy**.

## What’s included

- Full-bleed hero (cemetery editorial)
- Shop grid, look strip, bag drawer
- Product API: `GET /api/products`
- Health: `GET /health`

## Stickers

Print-ready brand stickers live in `static/stickers/`:

- `wordmark.png`
- `cemetery-soft.png`
- `hex-bow.png`
- `pretty-poison.png`
- `girl-owned.png`
- `hex-kitty.png`
- `pack-sheet.png` — all six on one sheet

Transparent PNGs with white die-cut borders — ready for Sticker Mule / Canva print.

## Swap images

Drop replacements in `static/images/`:

- `hero.jpg` — main photo
- `look.jpg`, `story.jpg` — section crops
- product photos in `static/images/`

Logos live in `static/logos/`.
