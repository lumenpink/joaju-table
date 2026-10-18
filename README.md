# joaju-table

[![crates.io](https://img.shields.io/crates/v/joaju-table.svg)](https://crates.io/crates/joaju-table)
[![docs.rs](https://docs.rs/joaju-table/badge.svg)](https://docs.rs/joaju-table)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![GOST](https://img.shields.io/badge/GOST-1.1.0-emerald.svg)](https://codeberg.org/lumenpink/gost)

Gerador soberano em Rust de tabelas acadêmicas no padrão de 3 linhas (ABNT NBR 14724 / Normas de Apresentação Tabular do IBGE / APA) renderizadas nativamente em **Typst**, com suporte a consultas SSOT dinâmicas.

> ⚠️ **Early Development Notice (pre-1.0 / alpha)**  
> This library is currently in early active development (`0.1.0`). It is NOT recommended for production use beyond experimental testing and research. APIs are subject to breaking changes prior to a stable 1.0.0 release.

---

## 📖 Visão Geral

Nas normas acadêmicas brasileiras (ABNT e IBGE) e internacionais (APA), **tabelas não possuem bordas verticais**. Elas são caracterizadas pela clássica estrutura de "3 linhas horizontais":
1. Linha superior espessa (`1.2pt`)
2. Linha separadora do cabeçalho (`0.6pt`)
3. Linha inferior de fechamento dos dados (`1.2pt`)
4. Legenda / Título no topo e fonte / notas padronizadas no rodapé com alinhamento à esquerda.

`joaju-table` aceita dados inline (Markdown ou CSV) ou fontes externas através de integração com o motor `joaju-csv`, permitindo filtrar (`%%where`) e projetar (`%%select`) os dados da tabela em tempo de compilação.

---

## 🚀 Instalação

Adicione `joaju-table` ao seu `Cargo.toml`:

```toml
[dependencies]
joaju-table = "0.1.0"
```

Ou instale a CLI:

```bash
cargo install joaju-table
```

---

## 🛠️ Exemplo de Uso (Rust API)

### 1. Renderizando um bloco com diretivas

```rust
use joaju_table::render_table_block;

let block = r#"
%% caption = Distribuição Salarial por Departamento
%% source = Departamento de Recursos Humanos (2026)
%% note = Valores expressos em salários mínimos.
departamento,funcionarios,media_salarial
Engenharia,42,8.5
Design,15,6.2
Atendimento,28,4.1
"#;

let typst_code = render_table_block(block).unwrap();
println!("{}", typst_code);
assert!(typst_code.contains("#figure("));
assert!(typst_code.contains("table.hline(stroke: 1.2pt)"));
assert!(typst_code.contains("Fonte: Departamento de Recursos Humanos"));
```

### 2. Pré-processamento de Documentos Markdown com blocos ````table`

```rust
use joaju_table::preprocess_tables;

let doc = r#"
# Relatório Anual

Abaixo apresentamos os resultados apurados:

```table
%% caption = Amostra de Teste
%% source = Laboratório Central
parametro,valor
pH,7.2
Temperatura,24.5
```
"#;

let processed = preprocess_tables(doc);
assert!(processed.contains("#figure("));
```

---

## 💻 Uso via CLI

```bash
# Converter arquivo com bloco table para Typst
joaju-table render tabela.csv --caption "Dados Coletados" --source "IBGE"

# Pré-processar arquivo markdown completo
joaju-table preprocess documento.md -o documento_typst.md
```

---

## 📋 Conformidade GOST

Este projeto adota a norma **[GOST](https://codeberg.org/lumenpink/gost)** (Gosplan Standard for Technical Software).
- **Versão GOST:** `1.1.0`
- **Linguagens declaradas:** `rust,shell`
- **Licença:** `Apache-2.0`

```bash
# Executar auditoria de qualidade estrita
cccp check --strict
```

---

## 📄 Licença

Distribuído sob a licença [Apache-2.0](./LICENSE).
