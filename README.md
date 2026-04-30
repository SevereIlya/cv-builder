# CV Builder

A robust Rust-based utility for generating PDF resumes from Markdown files and LaTeX templates.

This project replaces brittle shell/batch scripts with a compiled, safe, and user-friendly executable. It manages the pipeline of converting a Markdown CV into a professionally typeset PDF using Pandoc and LuaLaTeX, complete with native GUI file selection and structured logging.

## Key Features

* **Pre-flight Dependency Checking:** Automatically verifies the presence of `pandoc` and `lualatex` in the system environment before execution, preventing cryptic mid-compilation crashes.
* **Native OS Dialogs:** Eliminates hardcoded paths and CLI arguments. Utilizes native file dialogs to select the input `.md` file, the `.tex` template, and the output destination.
* **Structured Logging:** Implements comprehensive logging across all working stages to simplify debugging and track the compilation state.
* **Robust Error Handling:** Safely captures and formats process execution errors, including surfacing `stdout` and `stderr` directly from the Pandoc/LaTeX engines.

## Prerequisites

Before running the tool, ensure the following dependencies are installed and available in your system's `PATH`:

1. [Pandoc](https://pandoc.org/installing.html) - The universal markup converter.
2. [MiKTeX](https://miktex.org/download) or TeX Live - Specifically, the `lualatex` engine is required to process the template.
   > **Note for MiKTeX users:** It is highly recommended to enable *Install missing packages on-the-fly* during installation to avoid missing font or package errors during PDF generation.

## Build and Installation

To compile the project from source, you need to have [Rust and Cargo](https://rustup.rs/) installed on your system.

Clone the repository and build the optimized release version:

```bash
git clone https://github.com/YOUR_GITHUB_USERNAME/cv-builder.git
cd cv-builder
cargo build --release
```

The compiled binary will be located at `target/release/cv_builder.exe` (on Windows). This is a standalone executable that can be moved and executed from any directory.

## Technology Stack

* **Rust** - Core language.
* **[rfd](https://crates.io/crates/rfd)** - Cross-platform native file dialogs without the overhead of a full GUI framework.
* **[tracing](https://crates.io/crates/tracing)** & **[tracing-subscriber](https://crates.io/crates/tracing-subscriber)** - Application-level structured diagnostic logging.
* **[anyhow](https://crates.io/crates/anyhow)** - Idiomatic and flexible error handling.
