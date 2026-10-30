# joaju-table

[![crates.io](https://img.shields.io/crates/v/joaju-table.svg)](https://crates.io/crates/joaju-table)
[![docs.rs](https://docs.rs/joaju-table/badge.svg)](https://docs.rs/joaju-table)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![GOST](https://img.shields.io/badge/GOST-1.5.0-emerald.svg)](https://codeberg.org/lumenpink/gost)

A fast, deterministic Rust library for rendering 3-line academic tables (ABNT / IBGE / APA) natively in Typst, powered by `joaju-csv`.

---

## 📖 Overview

In academic guidelines (such as ABNT, IBGE, and APA), **tables do not use vertical borders**. Instead, they feature the classic three horizontal lines structure (top rule, header rule, bottom rule) and standardized notes.

`joaju-table` accepts inline table data (Markdown or CSV) as well as external sources, with full documentation and examples available in `site/` or at [joaju-table documentation](https://joaju.2lp.in).

---

## 🚀 Quick Start

Add `joaju-table` to your `Cargo.toml`:

```toml
[dependencies]
joaju-table = "0.2.0"
```

```rust
use joaju_table::render_table_block;

let block = "%% caption = Sample\nitem,value\nA,10\n";
let typst = render_table_block(block).unwrap();
assert!(typst.contains("#figure("));
```

---

## 📋 GOST Compliance

This project adheres to the **[GOST](https://codeberg.org/lumenpink/gost)** specification:
- **GOST Version:** `1.5.0`
- **Declared Languages:** `rust,shell`
- **License:** `Apache-2.0`

---

## 📄 License

Distributed under the [Apache-2.0](./LICENSE) license.
