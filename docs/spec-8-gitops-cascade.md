# Especificação Normativa — Ticket #8
## Pipeline GitOps de Publicação no Registry Lumen e Propagação em Cascata para Joaju

**Norma de Referência:** GOST v3.0.0 (Perfil Lilya 2.1)  
**Status:** Aprovado para Implementação  
**Data:** 2026-10-20 (Atualizado 2026-11-22)  

---

### 1. Escopo e Objetivos

Esta especificação define o pipeline unificado de integração contínua (CI), publicação no registry autônomo de crates Forgejo (`sparse+https://gambi.2lp.in/api/packages/lumen/cargo/`) e propagação em cascata GitOps para o compilador **Joaju (`joaju`)**:

1. **Recepção de Workflow Dispatch:** Aceita `workflow_dispatch:`, `push` e `pull_request`.
2. **Auto-atualização GitOps de Dependências Upstream:** Quando acionado via `workflow_dispatch`, executa `cargo update -p joaju-csv`. Se houver alteração em `Cargo.lock`, realiza commit e push para a branch `main`.
3. **Validação de Qualidade Rust:** Checagem de formatação determinística (`cargo fmt --check`), clippy zero warnings (`cargo clippy --all-targets -- -D warnings`), suíte completa de testes (`cargo test`) e auditoria GOST (`cccp check --strict`).
4. **Publicação Canônica:** Publica a crate `joaju-table` no registry `gambi` utilizando `${{ secrets.REGISTRY_TOKEN }}`.
5. **Propagação a Jusante (Downstream Cascade):** A publicação de novo pacote emite webhook nativo de `package` para o Gossnab (`https://gossnab.lohn.in/api/v1/webhooks/forgejo`), que dispara `kn cascade --kind cargo` para propagar downstream ao compilador `joaju`.
