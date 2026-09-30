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

## What’s included

- Full-bleed hero (cemetery editorial)
- Shop grid, look strip, bag drawer
- Product API: `GET /api/products`
- Health: `GET /health`

## Swap images

Drop replacements in `static/images/`:

- `hero.jpg` — main photo
- `look.jpg`, `story.jpg` — section crops
- `p1.jpg` … `p8.jpg` (+ optional `p*-hover.jpg`)

Logos live in `static/logos/`.
