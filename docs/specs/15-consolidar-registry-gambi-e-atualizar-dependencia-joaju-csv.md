# Especificação Técnica - Ticket #15
## Consolidar Registry gambi e Atualizar Dependência joaju-csv

### Contexto
O ecossistema canônico local e remoto utiliza a convenção `[registries.gambi]` para o endpoint sparse do pacote Cargo (`sparse+https://gambi.2lp.in/api/packages/lumen/cargo/`). Declarar simultaneamente `[registries.lumen]` e `[registries.gambi]` com a mesma URL causa erro imediato no Cargo. Em `joaju-table`, a dependência de `joaju-csv` estava declarada apontando para `registry = "lumen"`, gerando falha de resolução quando apenas `gambi` está configurado.

### Objetivos
1. Consolidar o registry do Cargo exclusivamente em `gambi` em `.forgejo/workflows/ci.yml`.
2. Alinhar `Cargo.toml` para `joaju-csv = { version = "0.1.0", registry = "gambi" }`.
3. Ajustar `tests/audit_gitops_cascade.sh` para aceitar `cargo publish --registry (lumen|gambi)` e `tests/version_spec.rs` para asserir a consolidação.
4. Adicionar fallback do daemon Kombinat para `172.17.0.1:4040`.

### Conclusão e Laudo de Fechamento (Fase 9)
Implementação completada com êxito. Todos os testes unitários, testes de integração de workflow e validações passaram sem ressalvas.
