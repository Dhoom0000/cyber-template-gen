# Cyber Template Gen

A CLI tool that generates a standardized directory tree for storing cybersecurity project data. Quickly scaffold organized structures for evidence, reports, findings, configurations, and other security artifacts.

## Installation

Build from source with Rust and Cargo installed:

```bash
cargo install --path .
```

Or build manually:

```bash
cargo build --release
./target/release/cyber-template-gen new --name example
```

## Usage

```bash
cyber-template-gen new --name project-name
```

Creates a new directory with all subdirectories and files defined in `Template.toml`. Uses `cyber-template` as the default directory name if `--name` is not provided.

By default, the generated structure includes base files (.gitignore, README.md, Makefile.toml, diagram.drawio, note.md), an assets folder with subdirectories for captures, images, and logs, a config directory with environment and tooling files, an evidence folder for hash lists, a results folder with scan outputs, a scripts directory for Rust and shell scripts, and a reporting folder for documentation.

## Template Format

Modify `Template.toml` to customize the generated structure. TOML sections define directory hierarchies:

```toml
files = ["file1.md", "file2.txt"]

[section]
files = ["file.txt"]

[section.subsection]
files = ["nested-file.txt"]
```

- `files` at the top level creates files in the root directory
- Each TOML section `[name]` creates a directory
- Nested sections create subdirectories with their own files