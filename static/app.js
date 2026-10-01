const state = {
  products: [],
  filter: "All",
  cart: [],
  selectedSizes: {},
  cartOpen: false,
  menuOpen: false,
};

const categories = ["All", "Dresses", "Tops", "Bottoms", "Sets", "Lingerie", "Outerwear"];
const sizes = ["XS", "S", "M", "L"];

const $ = (sel, root = document) => root.querySelector(sel);
const $$ = (sel, root = document) => [...root.querySelectorAll(sel)];

function money(n) {
  return `$${n}`;
}

function cartCount() {
  return state.cart.reduce((sum, item) => sum + item.quantity, 0);
}

function cartSubtotal() {
  return state.cart.reduce((sum, item) => sum + item.price * item.quantity, 0);
}

function openCart() {
  state.cartOpen = true;
  document.body.classList.add("cart-open");
  renderCart();
}

function closeCart() {
  state.cartOpen = false;
  document.body.classList.remove("cart-open");
  renderCart();
}

function openSoonAlert(kind = "soon") {
  const alert = $(".soon-alert");
  const title = $("#soon-title");
  const copy = $("#soon-copy");
  if (!alert || !title || !copy) return;

  if (kind === "stock") {
    title.textContent = "Everything is out of stock";
    copy.textContent = "Coming soon!";
  } else {
    title.textContent = "Not currently accepting orders";
    copy.textContent = "Coming soon!";
  }

  alert.hidden = false;
  document.body.classList.add("soon-open");
  $(".soon-alert-ok")?.focus();
}

function closeSoonAlert() {
  const alert = $(".soon-alert");
  if (!alert) return;
  alert.hidden = true;
  document.body.classList.remove("soon-open");
  try {
    sessionStorage.setItem("sugah3x-soon-seen", "1");
  } catch {
    /* ignore */
  }
}

function wireSoonAlert() {
  const alert = $(".soon-alert");
  if (!alert) return;

  $(".soon-alert-ok")?.addEventListener("click", closeSoonAlert);
  alert.addEventListener("click", (e) => {
    if (e.target === alert) closeSoonAlert();
  });
  window.addEventListener("keydown", (e) => {
    if (e.key === "Escape" && !alert.hidden) closeSoonAlert();
  });

  let seen = false;
  try {
    seen = sessionStorage.getItem("sugah3x-soon-seen") === "1";
  } catch {
    seen = false;
  }
  if (!seen) {
    window.setTimeout(openSoonAlert, 480);
  }
}

function sizeFor(productId) {
  return state.selectedSizes[productId] || "M";
}

function addToCart(product, size) {
  const pick = size || sizeFor(product.id);
  const existing = state.cart.find((item) => item.id === product.id && item.size === pick);
  if (existing) existing.quantity += 1;
  else state.cart.push({ ...product, size: pick, quantity: 1 });
  openCart();
  renderNav();
}

function removeFromCart(id, size) {
  state.cart = state.cart.filter((item) => !(item.id === id && item.size === size));
  renderCart();
  renderNav();
}

function updateQty(id, size, quantity) {
  if (quantity <= 0) return removeFromCart(id, size);
  const item = state.cart.find((i) => i.id === id && i.size === size);
  if (item) item.quantity = quantity;
  renderCart();
  renderNav();
}

function renderNav() {
  const countEl = $(".nav-cart-count");
  if (countEl) countEl.textContent = String(cartCount());
}

function renderFilters() {
  const toolbar = $(".shop-toolbar");
  if (!toolbar) return;
  toolbar.innerHTML = categories
    .map(
      (cat) => `
      <button class="filter-btn${state.filter === cat ? " active" : ""}" data-filter="${cat}" type="button">
        ${cat}
      </button>`
    )
    .join("");

  toolbar.onclick = (e) => {
    const btn = e.target.closest("[data-filter]");
    if (!btn) return;
    state.filter = btn.dataset.filter;
    renderFilters();
    renderProducts();
  };
}

function filteredProducts() {
  if (state.filter === "All") return state.products;
  return state.products.filter((p) => p.category === state.filter);
}

function renderProducts() {
  const grid = $(".product-grid");
  if (!grid) return;
  const items = filteredProducts();
  grid.innerHTML = items
    .map(
      (p) => `
    <article class="product${p.packshot ? " is-packshot-card" : ""}" data-reveal>
      <div class="product-media${p.packshot ? " is-packshot" : ""}${p.id === "savage-sweats" ? " is-bow" : ""}${["savage-tee", "bow-cargo-pants", "lace-hem-romper"].includes(p.id) ? " is-grave" : ""}" data-tilt>
        ${p.tag ? `<span class="product-tag">${p.tag}</span>` : ""}
        <img src="${p.image}" alt="${p.name}" loading="lazy" />
        ${
          p.hover_image
            ? `<img class="hover" src="${p.hover_image}" alt="" aria-hidden="true" loading="lazy" />`
            : ""
        }
        <button class="product-add" type="button" data-add="${p.id}">Add to bag</button>
      </div>
      <div class="product-meta">
        <div class="product-meta-row">
          <div>
            <h3 class="product-name">${p.name}</h3>
            <p class="product-category">${p.category}</p>
          </div>
          <p class="product-price">${money(p.price)}</p>
        </div>
        <div class="product-sizes" role="group" aria-label="Size for ${p.name}">
          ${sizes
            .map(
              (s) => `
            <button type="button" class="size-btn${sizeFor(p.id) === s ? " active" : ""}" data-size="${s}" data-product="${p.id}">${s}</button>`
            )
            .join("")}
        </div>
      </div>
    </article>`
    )
    .join("");

  grid.onclick = (e) => {
    const sizeBtn = e.target.closest("[data-size][data-product]");
    if (sizeBtn) {
      state.selectedSizes[sizeBtn.dataset.product] = sizeBtn.dataset.size;
      const group = sizeBtn.closest(".product-sizes");
      group?.querySelectorAll(".size-btn").forEach((btn) => {
        btn.classList.toggle("active", btn.dataset.size === sizeBtn.dataset.size);
      });
      return;
    }
    const btn = e.target.closest("[data-add]");
    if (!btn) return;
    const product = state.products.find((p) => p.id === btn.dataset.add);
    if (product) addToCart(product, sizeFor(product.id));
  };

  observeReveal();
  wireTilt($$("[data-tilt]", grid));
}

function renderCart() {
  const drawer = $(".cart-drawer");
  const backdrop = $(".cart-backdrop");
  if (!drawer || !backdrop) return;

  drawer.classList.toggle("open", state.cartOpen);
  backdrop.classList.toggle("open", state.cartOpen);
  drawer.setAttribute("aria-hidden", state.cartOpen ? "false" : "true");

  const body = $(".cart-body");
  if (!state.cart.length) {
    body.innerHTML = `<p class="cart-empty">Bag's empty — go make a mess in the shop.</p>`;
  } else {
    body.innerHTML = state.cart
      .map(
        (item) => `
      <div class="cart-item">
        <img src="${item.image}" alt="${item.name}" />
        <div>
          <div class="cart-item-top">
            <h3>${item.name}</h3>
            <button type="button" data-remove="${item.id}" data-size="${item.size}">Remove</button>
          </div>
          <p class="cart-item-meta">${money(item.price)} · ${item.size}</p>
          <div class="qty">
            <button type="button" data-qty="${item.id}" data-size="${item.size}" data-delta="-1" aria-label="Decrease">−</button>
            <span>${item.quantity}</span>
            <button type="button" data-qty="${item.id}" data-size="${item.size}" data-delta="1" aria-label="Increase">+</button>
          </div>
        </div>
      </div>`
      )
      .join("");
  }

  $(".cart-subtotal strong").textContent = money(cartSubtotal());
  const checkout = $(".cart-checkout");
  checkout.disabled = false;
  checkout.setAttribute("aria-disabled", "true");

  body.onclick = (e) => {
    const remove = e.target.closest("[data-remove]");
    if (remove) return removeFromCart(remove.dataset.remove, remove.dataset.size);
    const qty = e.target.closest("[data-qty]");
    if (!qty) return;
    const item = state.cart.find((i) => i.id === qty.dataset.qty && i.size === qty.dataset.size);
    if (!item) return;
    updateQty(item.id, item.size, item.quantity + Number(qty.dataset.delta));
  };
}

function wireCursor() {
  if (window.matchMedia("(hover: none), (pointer: coarse)").matches) return;
  if (window.matchMedia("(prefers-reduced-motion: reduce)").matches) return;

  const root = $(".cursor");
  const ring = $(".cursor-ring");
  const dot = $(".cursor-dot");
  if (!root || !ring || !dot) return;

  let x = window.innerWidth / 2;
  let y = window.innerHeight / 2;
  let rx = x;
  let ry = y;

  window.addEventListener(
    "pointermove",
    (e) => {
      x = e.clientX;
      y = e.clientY;
      dot.style.transform = `translate(${x}px, ${y}px) translate(-50%, -50%)`;
    },
    { passive: true }
  );

  window.addEventListener("pointerdown", () => document.body.classList.add("is-pressing"));
  window.addEventListener("pointerup", () => document.body.classList.remove("is-pressing"));

  const hoverables = "a, button, input, .product-media, .film-frame, .size-btn, [data-magnetic]";
  document.addEventListener("pointerover", (e) => {
    if (e.target.closest(hoverables)) document.body.classList.add("is-hovering");
  });
  document.addEventListener("pointerout", (e) => {
    if (e.target.closest(hoverables)) document.body.classList.remove("is-hovering");
  });

  const tick = () => {
    rx += (x - rx) * 0.18;
    ry += (y - ry) * 0.18;
    ring.style.transform = `translate(${rx}px, ${ry}px) translate(-50%, -50%)`;
    requestAnimationFrame(tick);
  };
  tick();
}

function wireMagnetic() {
  if (window.matchMedia("(hover: none), (pointer: coarse)").matches) return;
  $$("[data-magnetic]").forEach((el) => {
    el.addEventListener("pointermove", (e) => {
      const r = el.getBoundingClientRect();
      const dx = e.clientX - (r.left + r.width / 2);
      const dy = e.clientY - (r.top + r.height / 2);
      el.style.transform = `translate(${dx * 0.18}px, ${dy * 0.22}px)`;
    });
    el.addEventListener("pointerleave", () => {
      el.style.transform = "";
    });
  });
}

function wireTilt(nodes) {
  if (window.matchMedia("(hover: none), (pointer: coarse)").matches) return;
  nodes.forEach((el) => {
    if (el.classList.contains("is-packshot")) return;
    el.addEventListener("pointermove", (e) => {
      const r = el.getBoundingClientRect();
      const px = (e.clientX - r.left) / r.width - 0.5;
      const py = (e.clientY - r.top) / r.height - 0.5;
      el.style.transform = `rotateX(${(-py * 7).toFixed(2)}deg) rotateY(${(px * 8).toFixed(2)}deg)`;
    });
    el.addEventListener("pointerleave", () => {
      el.style.transform = "";
    });
  });
}

function wireParallax() {
  if (window.matchMedia("(hover: none), (pointer: coarse)").matches) return;
  const media = $("[data-parallax]");
  if (!media) return;
  const img = media.querySelector("img");
  if (!img) return;

  const onScroll = () => {
    const y = Math.min(window.scrollY, window.innerHeight);
    img.style.transform = `scale(1.08) translate3d(0, ${y * 0.18}px, 0)`;
  };
  window.addEventListener("scroll", onScroll, { passive: true });
  onScroll();
}

function wireScrollThread() {
  const bar = $(".scroll-thread i");
  if (!bar) return;
  const onScroll = () => {
    const max = document.documentElement.scrollHeight - window.innerHeight;
    const p = max > 0 ? (window.scrollY / max) * 100 : 0;
    bar.style.height = `${p}%`;
  };
  window.addEventListener("scroll", onScroll, { passive: true });
  onScroll();
}

function wireFilmDrag() {
  const track = $(".film-track");
  if (!track) return;
  let down = false;
  let startX = 0;
  let scrollLeft = 0;

  track.addEventListener("pointerdown", (e) => {
    if (e.target.closest("a, button")) return;
    down = true;
    startX = e.clientX;
    scrollLeft = track.scrollLeft;
    track.setPointerCapture(e.pointerId);
  });
  track.addEventListener("pointermove", (e) => {
    if (!down) return;
    track.scrollLeft = scrollLeft - (e.clientX - startX);
  });
  track.addEventListener("pointerup", () => {
    down = false;
  });
  track.addEventListener("pointercancel", () => {
    down = false;
  });
}

function observeReveal() {
  const items = $$("[data-reveal]");
  if (!items.length) return;
  if (!("IntersectionObserver" in window)) {
    items.forEach((el) => el.classList.add("is-in"));
    return;
  }
  const io = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        if (!entry.isIntersecting) return;
        entry.target.classList.add("is-in");
        io.unobserve(entry.target);
      });
    },
    { threshold: 0.18, rootMargin: "0px 0px -8% 0px" }
  );
  items.forEach((el) => io.observe(el));
}

function wireUi() {
  const nav = $(".nav");
  const onScroll = () => nav.classList.toggle("scrolled", window.scrollY > 20);
  window.addEventListener("scroll", onScroll, { passive: true });
  onScroll();

  $(".nav-cart").addEventListener("click", openCart);
  $(".cart-close").addEventListener("click", closeCart);
  $(".cart-backdrop").addEventListener("click", closeCart);
  $(".cart-checkout").addEventListener("click", (e) => {
    e.preventDefault();
    e.stopPropagation();
    closeCart();
    openSoonAlert("stock");
  });

  const menuBtn = $(".nav-menu-btn");
  const mobileNav = $(".mobile-nav");
  menuBtn.addEventListener("click", () => {
    state.menuOpen = !state.menuOpen;
    mobileNav.classList.toggle("open", state.menuOpen);
    nav.classList.toggle("scrolled", state.menuOpen || window.scrollY > 20);
  });

  $$(".mobile-nav a").forEach((a) =>
    a.addEventListener("click", () => {
      state.menuOpen = false;
      mobileNav.classList.remove("open");
    })
  );

  const CONTACT_EMAIL = "sugah3x@gmail.com";
  const form = $(".newsletter-form");
  form?.addEventListener("submit", (e) => {
    e.preventDefault();
    const email = form.querySelector("#email")?.value?.trim();
    if (!email) return;
    const subject = encodeURIComponent("SugaH3x list");
    const body = encodeURIComponent(`New signup\n\nFrom: ${email}\n`);
    window.location.href = `mailto:${CONTACT_EMAIL}?subject=${subject}&body=${body}`;
    form.outerHTML = `<p>Opening mail to ${CONTACT_EMAIL}…</p>`;
  });

  wireSoonAlert();
  wireCursor();
  wireMagnetic();
  wireParallax();
  wireScrollThread();
  wireFilmDrag();
  wireTilt($$("[data-tilt]"));
}

async function boot() {
  wireUi();
  renderNav();
  renderFilters();
  renderCart();

  try {
    const res = await fetch("/api/products");
    state.products = await res.json();
  } catch {
    state.products = [];
  }
  renderProducts();
}

boot();
