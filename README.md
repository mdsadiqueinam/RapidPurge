# RapidPurge

RapidPurge is a desktop utility (built with Tauri, Vue 3, and Rust) for quickly identifying and cleaning large or unnecessary files from your system. It provides a fast, native-like experience with a modern UI while leveraging Rust for performance-critical scanning.

## Features

- Fast file system scanning using a Rust backend
- "Flash Clean" view for quickly spotting large files and folders
- Human-readable size formatting (KB/MB/GB)
- Cross-platform desktop app powered by Tauri
- Vue 3 front-end with hot-reload for rapid development

## Tech Stack

- **Frontend:** Vue 3, Vite
- **Backend:** Rust, Tauri
- **Tooling:** pnpm, ESLint

## Getting Started

### Prerequisites

- Node.js and pnpm installed
- Rust toolchain installed (via `rustup`)

### Install dependencies

```bash
pnpm install
```

### Run in development mode

```bash
pnpm tauri dev
```

### Build for production

```bash
pnpm tauri build
```

## Recommended IDE Setup

- VS Code with the Vue - Official (Volar) extension
- Tauri VS Code extension
- rust-analyzer

## License

This project is licensed under the **Elastic License 2.0 (ELv2)**. See the `LICENSE` file for the full license text and terms, including limitations on providing the software as a hosted or managed service.
