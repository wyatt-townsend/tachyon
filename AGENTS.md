# Agent notes — tachyon

## Project layout (high level)

| Path | Role |
|------|------|
| `src/` | SvelteKit 2 + Svelte 5 UI: `routes/` (pages, layout), `lib/components/` (reusable UI), `app.css` (global design tokens), `app.html` (document shell). |
| `src-tauri/` | Tauri 2 desktop shell: Rust in `src-tauri/src/` (`main.rs`, `lib.rs`, commands/modules like `search.rs`), `tauri.conf.json`, `capabilities/`, `icons/`. Build artifacts live under `target/` (ignore for edits). |
| `static/` | Static files served by SvelteKit as-is. |

## Run the app (development)

The app is run with **`npm run tauri dev`** in Tauri-oriented setups. In this repo, **`npm run tauri:dev`** does the same (Vite + desktop window); with plain npm and the generic `tauri` script, that corresponds to **`npm run tauri -- dev`**.

## Mentorship

The maintainer is **learning Rust and Svelte**. When changing or reviewing code, explain Rust and Svelte concepts briefly and prefer small, readable steps over clever abstractions or complex solutions.

## Styling

**Fluent on space gray:** Windows 11–inspired polish on base **`#343D46`** (`--color-bg` in [`src/app.css`](src/app.css)) — **Segoe UI** stack, centered column layout, soft elevation, and **acrylic-style** panels (translucent fills + `backdrop-filter`; on desktop, **dark Mica** on Windows / under-window vibrancy on macOS with a transparent Tauri window). Keep motion short and smooth; respect **`prefers-reduced-motion`** in `app.css`.

**Practice:** Tokens stay on `:root` as `var(--...)`; use component `<style>` for local layout; simple class names; extend tokens before one-off colors.
