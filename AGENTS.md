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

## Styling guide (flat / TUI-inspired)

**Direction:** Flat, terminal-adjacent UI (Neovim-like: structured, calm, little chrome). **Base background:** space gray **`#343D46`** (`--color-bg` in `src/app.css`). **Theme:** muted, slightly cyberpunk — cool grays, restrained teal/blue accents; avoid loud gradients or rainbow accents.

**Practice (Svelte 5 + Tauri 2):**

- Keep **design tokens** (colors, spacing, typography, radii, motion) in **`src/app.css`** on `:root` as **`var(--...)`**. Components should consume these variables; avoid hard-coded hex in scattered rules.
- Use **component `<style>`** in `.svelte` files for layout and pieces specific to that component. Reserve **`:global(...)`** for rare exceptions so names stay easy to follow.
- Keep **selectors and class names simple** (`layout`, `toolbar`, `row`, etc.) so edits are obvious.
- **Motion:** short, purposeful transitions on hovers, presses, and focus (e.g. 80–160ms, `ease`); respect **`prefers-reduced-motion`** by narrowing or disabling animation (see existing pattern in `app.css`).

When adding UI, extend the existing token set before introducing one-off colors or durations.
