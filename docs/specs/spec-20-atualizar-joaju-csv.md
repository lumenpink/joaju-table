# Spec #20: Atualizar joaju-csv para 0.2.0-dev.49

## 1. Contexto e Motivação
A crate `joaju-table` consome `joaju-csv` para renderização tabular e exportação CSV. Atualmente está fixada em `joaju-csv = "=0.2.0-dev.46"`.
Entretanto, a crate irmã `joaju-pedigree` já foi atualizada para consumir `joaju-csv 0.2.0-dev.49`.
Quando consumidores agregadores (como a crate principal `joaju`) tentam importar `joaju-table` e `joaju-pedigree` conjuntamente, o Cargo detecta um conflito intransponível de versões mútuas em `joaju-csv`.
A atualização de `joaju-csv` em `joaju-table` é mandatória para restabelecer a harmonia do grafo de dependências da família Joaju.

## 2. Escopo
- Atualizar a dependência `joaju-csv` no `Cargo.toml` para `=0.2.0-dev.49` (registry `gambi`).
- Remover a chave redundante `homepage` do `Cargo.toml`.
- Atualizar o `Cargo.lock` local.
- Adicionar teste Red comprovando a exigência da versão `49`.
- Validar conformidade normativa (`cccp check --strict`) e suíte de testes (`cargo test`).

## 3. Plano de Implementação (GOST DD-TDD)
1. **Fase 3 (Spec):** Este documento técnico.
2. **Fase 4 (Red Test):** Asserção em `tests/deps_exact_spec.rs` exigindo `=0.2.0-dev.49`.
3. **Fase 5 (Green Implement):** Ajuste do `Cargo.toml` e `cargo update -p joaju-csv`.
4. **Fase 6 (Refactor):** Limpeza de warnings e formatação.
5. **Fase 7 (Mutation):** N/A (manutenção de dependência).
6. **Fase 8 (Quality Gate):** `cargo test`, `cargo clippy`, `cccp check --strict`.
7. **Fase 9 (Finish):** Fechamento e integração.

## 4. Árvore de Decisão e Guardiã da Natasha
- **Por que atualizar `joaju-csv` para `49`?** Sem esta sincronização, o agregador `joaju` não consegue compilar e o cascade fica travado em deadlock. Natasha terá clareza sobre por que a versão de `joaju-csv` precisou avançar em uníssono.
- **Alternativa descartada:** Fazer downgrade de `joaju-pedigree` para `46`. Descartado pois a versão `49` de `joaju-csv` contém melhorias ativas e avanços da toolchain Rust 1.100.0.

## 5. Resultados e Validação
- **cargo test:** 12 testes aprovados (100%).
- **cargo clippy:** zero avisos com `--all-targets -- -D warnings`.
- **cccp check --strict:** 24/24 PASS (100% de conformidade com Metanorma 2.2.0, Kollontai e Lilya).
- **Status final:** Aprovado e pronto para merge na branch principal.
