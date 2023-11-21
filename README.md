# CRUD de Cadastro de Frutas com SQLite
Bem-vindo ao Desafio Rust de Construção de um CRUD para Cadastro de Frutas com Banco de Dados SQLite. Este desafio tem como objetivo ajudá-lo a praticar e aprimorar suas habilidades em Rust, desenvolvendo um aplicativo simples para gerenciar um cadastro de frutas.

## Objetivo do Desafio
O objetivo deste desafio é construir um aplicativo Rust que permita a realização das operações CRUD (Criar, Ler, Atualizar, Deletar) em um cadastro de frutas, utilizando um banco de dados SQLite para armazenamento persistente.

## Requisitos do Desafio
1. Cadastro de Frutas: Implemente a funcionalidade para adicionar novas frutas ao sistema, incluindo nome, tipo e outras informações relevantes.
2. Listagem de Frutas: Desenvolva uma funcionalidade que permita listar todas as frutas cadastradas.
3. Atualização de Frutas: Crie a capacidade de atualizar as informações de uma fruta existente no sistema.
4. Remoção de Frutas: Implemente a funcionalidade para excluir uma fruta do cadastro.
5. Banco de Dados SQLite: Utilize o SQLite como banco de dados para armazenar as informações das frutas de forma persistente.
6. Separação de Responsabilidades por Arquivos: É obrigatório organizar o código de forma que as responsabilidades sejam separadas em diferentes arquivos. Considere uma estrutura modular que facilite a manutenção e a compreensão do código.

## Sugestão de estrutura de pastas

```bash
- src
  - main.rs
  - lib.rs
  - models
    - fruit.rs
  - db
    - mod.rs
    - connection.rs
    - queries.rs
  - controllers
    - fruit_controller.rs
```

#### Explicação das pastas:

    src: Esta é a pasta principal do seu código fonte.
        - main.rs: O ponto de entrada da sua aplicação, onde a função main é definida.
        - lib.rs: Pode conter o código principal da biblioteca se você estiver desenvolvendo um crate (biblioteca Rust).

    models: Esta pasta pode conter os arquivos de definição de estrutura de dados, como a representação de uma fruta.
        - fruit.rs: Definição da estrutura de dados para uma fruta.

    db: Esta pasta pode conter os módulos relacionados ao banco de dados SQLite.
        - mod.rs: Arquivo de módulo principal para o módulo db.
        - connection.rs: Código para gerenciar a conexão com o banco de dados.
        - queries.rs: Definições de consultas SQL ou funções relacionadas ao banco de dados.

    controllers: Esta pasta pode conter os controladores responsáveis por manipular os dados antes de serem enviados para o banco de dados ou apresentados ao usuário.
        - fruit_controller.rs: Implementação de funções para realizar operações CRUD específicas para frutas.

## Recursos Úteis
 - Documentação do <a href="https://docs.rs/sqlite/0.28.0/sqlite/" target="_blank">SQLite em Rust</a>
 - <a href="https://doc.rust-lang.org/book/" target="_target">The Rust Programming Language</a>
 - <a href="https://crates.io/" target="_target">Crates.io</a> - Repositório de pacotes Rust