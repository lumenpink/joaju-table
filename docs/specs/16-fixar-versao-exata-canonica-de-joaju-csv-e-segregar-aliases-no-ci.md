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
- Fase 6–8: Validação com `cargo test`, `cargo clippy` e `cccp check --strict`.
