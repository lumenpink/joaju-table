use std::fs;
use table_rs::render_table_block;

#[test]
fn test_render_basic_inline_markdown_table() {
    let input = r#"%% title=Tabela 1 – Dados Demográficos
%% source=IBGE (2026).
| ID | Nome | Cidade |
|---|---|---|
| 1 | Alice | Florianópolis |
| 2 | Bob | Curitiba |
"#;
    let typst = render_table_block(input).expect("falha ao renderizar tabela");

    assert!(typst.contains("caption: [Tabela 1 – Dados Demográficos]"));
    assert!(typst.contains("supplement: [Tabela]"));
    assert!(typst.contains("table.hline(stroke: 1.2pt)"));
    assert!(typst.contains("table.header("));
    assert!(typst.contains("[ID], [Nome], [Cidade]"));
    assert!(typst.contains("table.hline(stroke: 0.6pt)"));
    assert!(typst.contains("[1], [Alice], [Florianópolis]"));
    assert!(typst.contains("[2], [Bob], [Curitiba]"));
    assert!(typst.contains("Fonte: IBGE (2026)."));
}

#[test]
fn test_render_table_with_ssot_query() {
    let csv_content = "id,fid,mid,sex,geracao,status\n1,0,0,1,1,ativo\n2,0,0,2,1,ativo\n3,1,2,2,2,inativo\n4,1,2,1,2,ativo\n";
    let tmp_dir = std::env::temp_dir();
    let tmp_file = tmp_dir.join("test_ssot_fam.csv");
    fs::write(&tmp_file, csv_content).expect("falha ao criar arquivo temporário");

    let input = format!(
        "%% src={}\n%% title=Tabela 2 – Membros Ativos da Família\n%% note=Dados preliminares.\n%% select=id, fid as pai, mid as mae, sex as sexo\n%% where=status == ativo and geracao <= 2\n",
        tmp_file.display()
    );

    let typst = render_table_block(&input).expect("falha ao renderizar tabela com SSOT");

    assert!(typst.contains("caption: [Tabela 2 – Membros Ativos da Família]"));
    assert!(typst.contains("[id], [pai], [mae], [sexo]"));
    assert!(typst.contains("[1], [0], [0], [1]"));
    assert!(typst.contains("[2], [0], [0], [2]"));
    assert!(typst.contains("[4], [1], [2], [1]"));
    // Membro 3 era inativo, não deve estar presente
    assert!(!typst.contains("[3], [1], [2], [2]"));
    assert!(typst.contains("Nota: Dados preliminares."));

    let _ = fs::remove_file(tmp_file);
}

#[test]
fn test_render_table_with_custom_columns_and_align() {
    let input = r#"%% columns=1fr, 2fr
%% align=center, left
| Código | Descrição |
| 101 | Análise de Heredograma |
"#;
    let typst = render_table_block(input).expect("falha ao renderizar tabela");
    assert!(typst.contains("columns: (1fr, 2fr)"));
    assert!(typst.contains("align: (center, left)"));
}

#[test]
fn test_preprocess_multiple_tables_in_markdown() {
    let md = r#"# Seção de Resultados

```table
%% title=Primeira Tabela
| A | B |
| 1 | 2 |
```

Texto intermediário.

```table
%% title=Segunda Tabela
| X | Y |
| 9 | 8 |
```
"#;
    let processed = table_rs::preprocess_tables(md);
    assert!(processed.contains("caption: [Primeira Tabela]"));
    assert!(processed.contains("caption: [Segunda Tabela]"));
    assert!(processed.contains("Texto intermediário."));
}
