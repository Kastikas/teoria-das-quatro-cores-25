# Diretrizes do Agente para o Projeto quatro_cores

## 1. Verificação Contínua e Rigor Algébrico
- **Sempre rodar `cargo test`**: Após qualquer alteração em arquivos de código (`src/*.rs`, testes ou verificadores C), execute `cargo test` obrigatoriamente antes de concluir a resposta ou avançar para a próxima etapa.
- **Proibição de Atalhos**: Nunca relaxe, comente ou afrouxe uma validação matemática para fazer configurações passarem nos testes. Se uma configuração falhar nas regras estritas, o comportamento correto é rejeitar a configuração com erro explícito.
- **Preservação de Desempenho**: Mantenha o perfil de teste otimizado (`opt-level = 3` em `Cargo.toml`) para permitir que a validação do catálogo canônico de 629 configurações execute com viabilidade temporal.
