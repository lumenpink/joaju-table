//! # joaju-table
//!
//! A fast and deterministic Rust library for rendering 3-line academic tables
//! (ABNT NBR 14724 / IBGE / APA) natively in Typst with tabular data queries.
//!
//! > ⚠️ **Early Development Notice (pre-1.0 / alpha)**: This library is currently in early
//! > active development (`0.1.0`). It is NOT recommended for production use beyond experimental
//! > testing and research. APIs are subject to breaking changes prior to a stable 1.0.0 release.
//!
//! ## Quick Example
//!
//! ```rust
//! use joaju_table::render_table_block;
//!
//! let block = "%% caption = Census\nid,name\n1,Ana\n";
//! let typst = render_table_block(block).unwrap();
//! assert!(typst.contains("#figure("));
//! assert!(typst.contains("table.hline"));
//! ```

use joaju_csv::Dataset;
use regex::Regex;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TableError {
    #[error("CSV error: {0}")]
    Csv(#[from] joaju_csv::CsvError),

    #[error("Empty table or no data")]
    EmptyTable,

    #[error("Format error: {0}")]
    FormatError(String),
}

#[derive(Debug, Clone, Default)]
pub struct TableConfig {
    pub caption: Option<String>,
    pub source: Option<String>,
    pub note: Option<String>,
    pub columns: Option<String>,
    pub align: Option<String>,
    pub layout_width: Option<String>,
}

impl TableConfig {
    pub fn from_directives(directives: &HashMap<String, String>) -> Self {
        let caption = directives
            .get("caption")
            .or_else(|| directives.get("title"))
            .or_else(|| directives.get("titulo"))
            .cloned();

        let source = directives
            .get("source")
            .or_else(|| directives.get("fonte"))
            .cloned();

        let note = directives
            .get("note")
            .or_else(|| directives.get("nota"))
            .cloned();

        let columns = directives.get("columns").cloned();
        let align = directives.get("align").cloned();
        let layout_width = directives
            .get("layout.width")
            .or_else(|| directives.get("width"))
            .cloned();

        Self {
            caption,
            source,
            note,
            columns,
            align,
            layout_width,
        }
    }
}

fn normalize_typst_align(raw: &str) -> String {
    let parts: Vec<&str> = raw.split(',').map(|s| s.trim()).collect();
    let mapped: Vec<String> = parts
        .into_iter()
        .map(|p| match p.to_lowercase().as_str() {
            "c" | "center" => "center".to_string(),
            "l" | "left" => "left".to_string(),
            "r" | "right" => "right".to_string(),
            other => other.to_string(),
        })
        .collect();

    if mapped.len() > 1 {
        format!("      align: ({}),\n", mapped.join(", "))
    } else if let Some(first) = mapped.first() {
        format!("      align: {},\n", first)
    } else {
        String::new()
    }
}

pub fn render_dataset_to_typst(
    dataset: &Dataset,
    config: &TableConfig,
) -> Result<String, TableError> {
    if dataset.row_count() == 0 && dataset.column_count() == 0 {
        return Err(TableError::EmptyTable);
    }

    let num_cols = dataset.column_count();
    let headers = dataset.headers();
    let rows = dataset.rows();

    let mut out = String::new();

    if let Some(ref cap) = config.caption {
        out.push_str(&format!(
            "#figure(\n  caption: [{}],\n  supplement: [Tabela],\n)[\n",
            cap
        ));
    }

    out.push_str("  #align(center)[\n");

    // Configuração de colunas
    let cols_str = if let Some(ref c) = config.columns {
        if c.contains(',') {
            format!("({})", c)
        } else {
            c.clone()
        }
    } else {
        num_cols.to_string()
    };

    // Configuração de alinhamento
    let align_str = if let Some(ref a) = config.align {
        normalize_typst_align(a)
    } else {
        String::new()
    };

    out.push_str(&format!(
        "    #table(\n      columns: {},\n{}      stroke: none,\n      table.hline(stroke: 1.2pt),\n      table.header(\n",
        cols_str, align_str
    ));

    // Cabeçalho
    out.push_str("        ");
    for (i, cell) in headers.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(&format!("[{}]", cell));
    }
    out.push_str("\n      ),\n      table.hline(stroke: 0.6pt),\n");

    // Linhas de dados
    for row in rows {
        out.push_str("      ");
        for (i, cell) in row.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            out.push_str(&format!("[{}]", cell));
        }
        out.push_str(",\n");
    }

    out.push_str("      table.hline(stroke: 1.2pt),\n    )\n  ]\n");

    // Fonte e Nota de rodapé padrão ABNT
    if config.source.is_some() || config.note.is_some() {
        out.push_str("  #v(-4pt)\n  #align(left)[\n    #set text(size: 8.5pt)\n");
        if let Some(ref s) = config.source {
            out.push_str(&format!("    Fonte: {}\n", s));
        }
        if let Some(ref n) = config.note {
            out.push_str(&format!("    Nota: {}\n", n));
        }
        out.push_str("  ]\n");
    }

    if config.caption.is_some() {
        out.push_str("]\n");
    }

    Ok(out)
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

pub fn render_dataset_to_html(
    dataset: &Dataset,
    config: &TableConfig,
) -> Result<String, TableError> {
    if dataset.row_count() == 0 && dataset.column_count() == 0 {
        return Err(TableError::EmptyTable);
    }

    let headers = dataset.headers();
    let rows = dataset.rows();
    let alignments: Vec<&str> = if let Some(ref a) = config.align {
        a.split(',')
            .map(|s| match s.trim().to_lowercase().as_str() {
                "c" | "center" => "center",
                "r" | "right" => "right",
                _ => "left",
            })
            .collect()
    } else {
        Vec::new()
    };

    let width_style = config
        .layout_width
        .as_deref()
        .map(|w| format!(" style=\"width: {};\"", html_escape(w)))
        .unwrap_or_default();

    let mut out = format!(
        "<div class=\"joaju-academic-table-wrapper\"{}>\n",
        width_style
    );

    if let Some(ref cap) = config.caption {
        out.push_str(&format!(
            "  <div class=\"joaju-table-title\">{}</div>\n",
            html_escape(cap)
        ));
    }

    out.push_str("  <table class=\"joaju-academic-table\">\n    <thead>\n      <tr>\n");
    for (i, h) in headers.iter().enumerate() {
        let align = alignments.get(i).copied().unwrap_or("left");
        out.push_str(&format!(
            "        <th style=\"text-align: {};\">{}</th>\n",
            align,
            html_escape(h)
        ));
    }
    out.push_str("      </tr>\n    </thead>\n    <tbody>\n");

    for row in rows {
        out.push_str("      <tr>\n");
        for (i, cell) in row.iter().enumerate() {
            let align = alignments.get(i).copied().unwrap_or("left");
            out.push_str(&format!(
                "        <td style=\"text-align: {};\">{}</td>\n",
                align,
                html_escape(cell)
            ));
        }
        out.push_str("      </tr>\n");
    }
    out.push_str("    </tbody>\n  </table>\n");

    if let Some(ref s) = config.source {
        out.push_str(&format!(
            "  <div class=\"joaju-table-note\">Fonte: {}</div>\n",
            html_escape(s)
        ));
    } else if let Some(ref n) = config.note {
        out.push_str(&format!(
            "  <div class=\"joaju-table-note\">Nota: {}</div>\n",
            html_escape(n)
        ));
    }

    out.push_str("</div>");
    Ok(out)
}

pub fn parse_dataset_and_config(raw_content: &str) -> Result<(Dataset, TableConfig), TableError> {
    let (directives, body) = joaju_csv::split_block_content(raw_content);
    let config = TableConfig::from_directives(&directives);

    let select_expr = directives.get("select").map(|s| s.as_str());
    let where_expr = directives.get("where").map(|s| s.as_str());

    let dataset = if let Some(src_path) = directives
        .get("src")
        .or_else(|| directives.get("file"))
        .or_else(|| directives.get("arquivo"))
    {
        let ds = Dataset::from_file(src_path)?;
        ds.apply_query(select_expr, where_expr)?
    } else {
        let clean_lines: Vec<&str> = body
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .filter(|l| !(l.contains("---") && (l.contains('|') || l.contains('+'))))
            .collect();

        if clean_lines.is_empty() {
            return Err(TableError::EmptyTable);
        }

        let first = clean_lines[0];
        let ds = if first.contains('|') {
            // Tabela Markdown (| col1 | col2 |)
            let mut all_rows = Vec::new();
            for line in clean_lines {
                let cells: Vec<String> = line
                    .split('|')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                if !cells.is_empty() {
                    all_rows.push(cells);
                }
            }
            if all_rows.is_empty() {
                return Err(TableError::EmptyTable);
            }
            let headers = all_rows[0].clone();
            let rows = all_rows[1..].to_vec();
            Dataset::new(headers, rows)
        } else {
            let joined = clean_lines.join("\n");
            Dataset::from_csv(&joined)?
        };

        ds.apply_query(select_expr, where_expr)?
    };

    Ok((dataset, config))
}

pub fn render_table_block(raw_content: &str) -> Result<String, TableError> {
    let (dataset, config) = parse_dataset_and_config(raw_content)?;
    render_dataset_to_typst(&dataset, &config)
}

pub fn render_html_table(raw_content: &str) -> Result<String, TableError> {
    let (dataset, config) = parse_dataset_and_config(raw_content)?;
    render_dataset_to_html(&dataset, &config)
}

pub fn preprocess_tables(markdown: &str) -> String {
    let re = Regex::new(r"(?s)```table\s*\n(.*?)```").unwrap();
    re.replace_all(markdown, |caps: &regex::Captures| {
        let content = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        match render_table_block(content) {
            Ok(typst) => format!("\n{}\n", typst.trim()),
            Err(_) => caps.get(0).unwrap().as_str().to_string(),
        }
    })
    .to_string()
}

pub use render_table_block as render_single_table;
