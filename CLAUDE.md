# Celah

Desktop app (Tauri v2) for bibliometric analysis and research gap finding.
Frontend: React + TypeScript + Vite in `src/`. Backend: Rust in `src-tauri/`, SQLite storage.

## Rules

1. No emoji anywhere in code: source, comments, UI strings, logs.
2. Keep files short. Aim for under 150 lines per file; split into modules or components.
3. Security first:
   - SQL only through parameterized queries (`params![]`), never string formatting.
   - Validate every value that crosses the `invoke()` boundary in Rust (length, range, charset).
   - Outbound HTTP only to the allowlisted research APIs, HTTPS only, with timeouts.
   - Keep the Tauri CSP strict and capabilities minimal; no shell or fs plugin unless required.
   - API keys go to the OS keychain, never to files, logs or the database.
   - Never render untrusted HTML (`dangerouslySetInnerHTML` is not allowed).
4. UI must match the design canvas and be responsive (check widths 1440, 1100, 800).
   Design: https://claude.ai/artifact/Mz3WF8u91P1i5963NBEoGT

## Commands

- `npm run dev` - Vite dev server only (browser preview, backend calls unavailable)
- `npm run tauri dev` - full desktop app
- `npm run check` - TypeScript type check
- `cargo check --manifest-path src-tauri/Cargo.toml` - Rust check
