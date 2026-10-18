# joaju-table

[![crates.io](https://img.shields.io/crates/v/joaju-table.svg)](https://crates.io/crates/joaju-table)
[![docs.rs](https://docs.rs/joaju-table/badge.svg)](https://docs.rs/joaju-table)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![GOST](https://img.shields.io/badge/GOST-1.1.0-emerald.svg)](https://codeberg.org/lumenpink/gost)

A fast, deterministic Rust library for rendering 3-line academic tables (ABNT NBR 14724 / IBGE / APA) natively in **Typst**, featuring dynamic tabular queries powered by `joaju-csv`.

> ⚠️ **Early Development Notice (pre-1.0 / alpha)**  
> This library is currently in early active development (`0.1.0`). It is NOT recommended for production use beyond experimental testing and research. APIs are subject to breaking changes prior to a stable 1.0.0 release.

---

## 📖 Overview

In academic guidelines (such as ABNT, IBGE, and APA), **tables do not use vertical borders**. Instead, they feature the classic "three horizontal lines" structure:
1. Top horizontal rule (`1.2pt`)
2. Header separator rule (`0.6pt`)
3. Bottom closing rule (`1.2pt`)
4. Title / Caption on top and standardized source / notes at the bottom with left alignment.

`joaju-table` accepts inline table data (Markdown or CSV) as well as external tabular data sources through integration with `joaju-csv`, allowing compile-time row filtering (`%%where`) and column projections (`%%select`).

---

## 🚀 Installation

Add `joaju-table` to your `Cargo.toml`:

```toml
[dependencies]
joaju-table = "0.1.0"
```

Or install the CLI tool:

```bash
cargo install joaju-table
```

---

## 🛠️ Usage Example (Rust API)

### 1. Rendering a Block with Directives

```rust
use joaju_table::render_table_block;

let block = r#"
%% caption = Salary Distribution by Department
%% source = Department of Human Resources (2026)
%% note = Values expressed in minimum wage units.
department,employees,average_salary
Engineering,42,8.5
Design,15,6.2
Support,28,4.1
"#;

let typst_code = render_table_block(block).unwrap();
println!("{}", typst_code);
assert!(typst_code.contains("#figure("));
assert!(typst_code.contains("table.hline(stroke: 1.2pt)"));
assert!(typst_code.contains("Fonte: Department of Human Resources"));
```

### 2. Preprocessing Markdown Documents Containing ````table` Blocks

```rust
use joaju_table::preprocess_tables;

let doc = r#"
# Annual Report

The preliminary results are summarized below:

```table
%% caption = Test Sample
%% source = Central Laboratory
parameter,value
pH,7.2
Temperature,24.5
```
"#;

let processed = preprocess_tables(doc);
assert!(processed.contains("#figure("));
```

---

## 💻 CLI Usage

```bash
# Render a table block file to Typst
joaju-table --input table.csv

# Preprocess a full Markdown document
joaju-table --input document.md --markdown
```

---

## 📋 GOST Compliance

This project adheres to the **[GOST](https://codeberg.org/lumenpink/gost)** specification (Gosplan Standard for Technical Software).
- **GOST Version:** `1.1.0`
- **Declared Languages:** `rust,shell`
- **License:** `Apache-2.0`

```bash
# Run strict compliance verification
cccp check --strict
```

---

## 📄 License

Distributed under the [Apache-2.0](./LICENSE) license.
