# 🧰 Trait Image Exporter & WASM Previewer

This is a lightweight tool to help artists and developers generate trait metadata for generative art projects.
It includes:

* 🦀 A **Rust CLI tool** to traverse trait folders and generate a JSON file
* 🌐 A **WASM-powered HTML previewer** to visually combine and preview the trait layers

---

## 🗂️ Folder Structure

Organize your traits like this:

```
traits/
├── background/
│   ├── sky.1.svg
│   └── sunset.0.5.png
├── head/
│   ├── robot.7.svg
│   └── alien.svg
└── cap/
    ├── gold.0.2.png
    └── red.svg
```

* Folder names are used as trait categories (`background`, `head`, `cap`, etc.)
* File names optionally include rarity (e.g. `gold.0.2.png` = 20%)
* All common image formats are supported (`.svg`, `.png`, `.jpg`, `.jpeg`, `.webp`)

---

## ⚙️ Step 1: Build the Rust CLI

### 1. Install Rust (if not installed)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Clone and build the tool

```bash
git clone https://github.com/your-username/trait-tool.git
cd trait-tool
cargo build --release
```

### 3. Run the tool

```bash
./target/release/trait-tool \
  --path ./traits \
  --output ./traits.json \
  --format base64   # or 'raw' for inline svg
```

---

## 🌐 Step 2: Preview with WASM

### 1. Install dependencies

```bash
cargo install wasm-pack
rustup target add wasm32-unknown-unknown
```

### 2. Build the WASM module

```bash
cd previewer
wasm-pack build --target web --out-dir ./pkg
```

### 3. Open the previewer

You can simply open `index.html` in your browser:

```bash
open index.html  # macOS
xdg-open index.html  # Linux
start index.html  # Windows
```

### 4. Use the previewer

* Upload `traits.json`
* Click **Randomize** to generate random combinations visually

---

## 📦 Output Format Example (`traits.json`)

```json
{
  "background": [
    {
      "name": "Sky",
      "filename": "sky.1.svg",
      "data": "<svg>...</svg>",
      "rarity": 1.0,
      "order": 1
    }
  ],
  "head": [
    {
      "name": "Robot",
      "filename": "robot.7.svg",
      "data": "data:image/svg+xml;base64,...",
      "rarity": 0.7,
      "order": 1
    }
  ]
}
```

---

## 🧠 Notes

* File names like `name.0.3.png` will set rarity to `0.3`. If no rarity is included, it defaults to `1.0`.
* Trait names are auto-formatted from filenames (e.g. `gold_hat.0.5.svg` → `Gold Hat`)
* You can change layering order by modifying folder reading logic or trait ordering.

---

## 🤝 Contributing

PRs welcome! Built with ❤️ using Rust + WebAssembly.

---

## 🪪 License

MIT
