# WhatsApp Desktop Ultimate - Project Documentation

Project ini adalah desktop wrapper untuk WhatsApp Web yang dibangun menggunakan **Tauri v2** dan **Rust**. Sistem ini dikonfigurasi khusus untuk menembus batasan CORS, mencegah error sinkronisasi Service Worker (IndexedDB), mendukung kustomisasi CSS/JS injection, dan mendukung otomatisasi build via GitHub Actions tanpa membebani perangkat lokal.

---

## 1. Struktur Direktori Project

Pastikan struktur file di dalam repositori GitHub Anda tersusun seperti ini:
```text
├── .github/
│   └── workflows/
│       └── build.yml          # GitHub Actions Automated CI/CD Pipeline
├── src/
│   ├── inject.css             # File Kustomisasi Tampilan/Tema WhatsApp
│   └── inject.js              # File Kustomisasi Script/Otomatisasi JavaScript
├── src-tauri/
│   ├── src/
│   │   └── main.rs            # Logika Aplikasi Rust & Handler Asset Injection
│   ├── Cargo.toml             # Dependensi Rust & Fitur DevTools
│   └── tauri.conf.json        # Flag Chromium Engine (Anti-CORS) & Aturan Window
└── package.json               # Metadata Node.js & Tooling Build
