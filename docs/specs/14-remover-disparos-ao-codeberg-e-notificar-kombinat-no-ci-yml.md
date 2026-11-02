# Spec 14: Remover Disparos ao Codeberg e Notificar Kombinat no ci.yml

## 1. Contexto e Motivação
A esteira do `joaju-table` continha steps com chamadas para o Codeberg para autocomit e disparo downstream. O daemon `kombinat` (`http://127.0.0.1:4040/api/cascade/dispatch`) assume toda a orquestração de dependências no ecossistema `joaju`.

Além disso:
- Exportar `pub const VERSION: &str` no `src/lib.rs`.
- Purgar termos imperialistas da esteira.

## 2. Requisitos Técnicos
- Remover steps legados que interagem com o Codeberg.
- Inserir chamada HTTP POST para o Kombinat (`POST http://127.0.0.1:4040/api/cascade/dispatch`).
- Atualizar script `tests/audit_gitops_cascade.sh` para auditar a chamada ao Kombinat.
- Exportar `pub const VERSION: &str = env!("CARGO_PKG_VERSION");` em `src/lib.rs`.

## 3. Plano de Testes (DD-TDD)
- Fase 4: Teste comprovadamente falho verificando `VERSION` e endpoint do Kombinat no workflow.
- Fase 5: Implementação funcional concluída.
- Fase 8: Testes verdes, clippy limpo, `cccp check --strict` com 100% de conformidade.

## 4. Status do Ciclo DD-TDD
- **Status:** Concluído com sucesso (Fases 1 a 9).
