# Spec 16: Fixar Versão Exata Canônica de joaju-csv e Segregar Aliases no CI

## 1. Contexto e Motivação
Para evitar que alterações ou instabilidades no submódulo `joaju-csv` propaguem quebras acidentais na esteira de `joaju-table`, a dependência declarada no `Cargo.toml` deve ser exata e imutável (`=0.2.0-dev.44`), delegando ao Kombinat a coordenação controlada de atualizações.

Adicionalmente, os artefatos de publicação do canal de desenvolvimento devem segregar seus aliases sob `latest-dev` e `X.Y.Z-dev`, preservando as tags limpas e `latest` exclusivamente para releases estáveis.

## 2. Requisitos Técnicos
- Configurar no `Cargo.toml`:
  ```toml
  joaju-csv = { version = "=0.2.0-dev.44", registry = "gambi" }
  ```
- No `.forgejo/workflows/ci.yml`, segregar aliases para artefatos dev:
  - Canal `nightly`/`dev`: `${BASE_VERSION}-dev` e `latest-dev`.
  - Canal estável: `${BASE_VERSION}` e `latest`.
- Criar teste automatizado `tests/deps_exact_spec.rs` validando que a dependência `joaju-csv` utiliza o prefixo de versão exata `=`.

## 3. Plano de Testes (DD-TDD)
- Fase 4: Criar teste `tests/deps_exact_spec.rs` comprovando falha antes da fixação exata.
- Fase 5: Atualizar `Cargo.toml` e `.forgejo/workflows/ci.yml`.
- Fase 6: Refactor e auditoria de estilo (`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`).
- Fase 7: Análise de mutação e verificação de robustez do teste.
- Fase 8: Quality Gate e auditoria estrita (`cccp check --strict`).

## 4. Registro de Execução e Fechamento
- Fase 3 (Spec): Especificação técnica e normativa elaborada.
- Fase 4 (Red Test): Criado teste `tests/deps_exact_spec.rs` comprovando ausência da regra.
- Fase 5 (Green Implement): `Cargo.toml` fixado em `=0.2.0-dev.44` e `.forgejo/workflows/ci.yml` atualizado para segregar aliases `latest-dev`.
- Fase 6 (Refactor): Fmt e clippy validados com zero advertências.
- Fase 7 (Mutation Test): Validada a sensibilidade do teste contra mutações de operador de versão (remoção do prefixo `=`).
- Fase 8 (Quality Gate): Executados `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --test deps_exact_spec` e `cccp check --strict` com 100% de conformidade.
- Fase 9 (Finish & Close): Ciclo concluído com fixação imutável de `=0.2.0-dev.44`, segregação de aliases no CI para `latest-dev` e validação com 100% de aprovação.
