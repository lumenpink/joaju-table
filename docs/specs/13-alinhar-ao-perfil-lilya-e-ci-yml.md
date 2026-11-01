# Spec: Ticket #13 - Alinhar ao Perfil Lilya e ci.yml

## 1. Contexto e Motivação
Padronização do repositório `joaju-table` com as normas GOST v2.0 e o perfil `lilya` (`https://lilya.2lp.in/profile.md`), adotando extensão canônica `.forgejo/workflows/ci.yml` e expurgando terminologias obsoletas.

## 2. Requisitos Técnicos
- Renomeação de `.forgejo/workflows/ci.yaml` para `.forgejo/workflows/ci.yml`.
- Atualização do script de auditoria GitOps (`tests/audit_gitops_cascade.sh`) para validar `ci.yml`.
- Substituição de termos imperialistas por vocabulário canônico da engenharia.
- Conformidade total com `cccp check --strict`.

## 3. Critérios de Aceitação
- Testes de CI e auditoria GitOps executados com sucesso.
- 100% de conformidade com `cccp check --strict`.
