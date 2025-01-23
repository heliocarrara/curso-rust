
# **Hello World com Smart Contract no Soroban**

## **Introdução**

  Com o avanço das tecnologias de blockchain, os smart contracts têm se tornado ferramentas indispensáveis para descentralizar aplicações e automatizar processos. O Soroban, uma solução desenvolvida pela Stellar, permite criar contratos inteligentes de forma eficiente e escalável, combinando a segurança da rede Stellar com a flexibilidade de desenvolvimento em **Rust**. Este artigo apresenta uma implementação básica de um **"Hello World"** no Soroban, demonstrando a simplicidade e o poder dessa ferramenta. Além disso, discutiremos como essa abordagem pode ser expandida para casos de uso mais complexos.
  
  ---
  
  ## **Referencial Teórico**
  
  ### **Blockchain e Smart Contracts**
  
  Blockchain é uma tecnologia que oferece um registro distribuído, seguro e imutável. Smart contracts são programas que rodam dentro de blockchains, automatizando execuções baseadas em condições pré-definidas. Desde o Ethereum, que popularizou os contratos inteligentes, diversas plataformas têm surgido com abordagens específicas para diferentes casos de uso. Estudos como os de Buterin (2014) e Wood (2021) destacam a evolução dessa tecnologia e sua aplicabilidade em finanças descentralizadas (DeFi), cadeia de suprimentos e governança descentralizada.
  
  Além disso, trabalhos recentes como o de Zhang et al. (2023) e Karaarslan & Konacaklı (2020) exploram como contratos inteligentes podem ser integrados a sistemas de armazenamento descentralizado e redes de dados sensíveis, evidenciando a flexibilidade e o impacto crescente dessa tecnologia.
  
  ### **Soroban**
  
  Soroban é um framework de contratos inteligentes construído sobre a rede Stellar. Ele traz:
  - **Desempenho otimizado**: aproveita a infraestrutura escalável da Stellar, que é capaz de processar milhares de transações por segundo.
  - **Segurança**: utiliza o Rust, uma linguagem conhecida por sua segurança contra erros de memória e alta performance.
  - **Simples integração**: ideal para criar aplicações descentralizadas (dApps) com foco em interoperabilidade.
  
  Um diferencial do Soroban, conforme destacado pela Stellar Development Foundation (2025), é a capacidade de integrar contratos inteligentes com sistemas financeiros tradicionais, permitindo soluções híbridas entre DeFi e finanças tradicionais.
  
  ---
  
  ## **Metodologia**
  
  Este artigo segue os passos necessários para implementar um contrato inteligente "Hello World" no Soroban, cobrindo:
  1. Configuração do ambiente de desenvolvimento.
  2. Criação de um projeto no Soroban.
  3. Escrita do contrato inteligente em Rust.
  4. Compilação, implantação e teste do contrato.
  
  ### **1. Configuração do Ambiente**
  
  Antes de começar, configure seu ambiente de desenvolvimento:
  - **Instale o Rust**: [https://rustup.rs/](https://rustup.rs/)
  - **Instale o Soroban CLI**:
    ```bash
    cargo install --locked --version 0.7.0 soroban-cli
    ```
  
  ### **2. Criando o Projeto**
  Crie um novo projeto para o contrato inteligente:
  ```bash
  soroban new hello_world
  cd hello_world
  ```
  
  ### **3. Escrevendo o Contrato**
  Abra o arquivo `src/lib.rs` e insira o seguinte código:
  
  ```rust
  #![no_std]
  
  use soroban_sdk::{contractimpl, Env};
  
  pub struct HelloWorld;
  
  #[contractimpl]
  impl HelloWorld {
      pub fn say_hello(env: Env, name: soroban_sdk::Symbol) -> soroban_sdk::Symbol {
          env.log(&format!("Hello, {}!", name));
          soroban_sdk::Symbol::new(&env, &format!("Hello, {}!", name))
      }
  }
  ```
  
  ### **4. Compilando o Contrato**
  Compile o contrato usando o Soroban CLI:
  ```bash
  soroban build
  ```
  
  O arquivo compilado estará em `target/wasm32-unknown-unknown/release/hello_world.wasm`.
  
  ### **5. Implantação e Teste**
  1. **Inicie um ambiente local**:
     ```bash
     soroban network add local http://127.0.0.1:8000
     ```
  
  2. **Implante o contrato**:
     ```bash
     soroban contract deploy \
         --wasm target/wasm32-unknown-unknown/release/hello_world.wasm \
         --network local
     ```
  
  3. **Teste a função `say_hello`**:
     ```bash
     soroban contract invoke \
         --id <CONTRACT_ID> \
         --fn say_hello \
         --arg "World" \
         --network local
     ```
  Substitua `<CONTRACT_ID>` pelo ID retornado no momento da implantação.
  
  ---
  
  ## **Resultados**
  
  Após executar o contrato, a saída esperada é:
  ```plaintext
  Hello, World!
  ```
  O log também armazenará a mensagem "Hello, World!", demonstrando que o contrato foi executado com sucesso. Esse comportamento é consistente com a documentação oficial do Soroban (2025).
  
  ---
  
  ## **Discussão**
  
  A implementação do "Hello World" no Soroban mostra como criar um smart contract básico em poucos passos. Este exemplo simples abre portas para explorar funcionalidades mais avançadas, como:
  
  - **Persistência de dados**: A funcionalidade de persistência pode ser implementada utilizando os recursos de armazenamento do Soroban SDK. Por exemplo, um contrato poderia armazenar dados enviados por usuários, como mensagens personalizadas ou configurações específicas, e recuperá-los posteriormente. Isso é útil para construir aplicações descentralizadas que precisam manter informações entre transações.
  
  - **Autenticação e autorização**: Para interações mais seguras, o contrato poderia implementar verificações baseadas em assinaturas ou chaves públicas. Isso garantiria que apenas usuários autorizados possam realizar determinadas ações, como alterar configurações ou acessar dados sensíveis.
  
  - **Interação com múltiplos contratos**: A arquitetura do Soroban permite que contratos inteligentes interajam entre si. Por exemplo, um contrato "Hello World" expandido poderia consultar outro contrato para obter dados dinâmicos, como o horário atual, e incluir essa informação na mensagem retornada.
  
  - **Integração com front-ends**: Ao conectar o contrato a uma interface gráfica, os usuários poderiam enviar dados diretamente por meio de um formulário e visualizar os resultados de forma intuitiva. Essa abordagem é essencial para aplicações voltadas ao consumidor final.
  
  A escolha do Rust como linguagem de desenvolvimento destaca-se por oferecer segurança em tempo de compilação, reduzindo vulnerabilidades que poderiam ser exploradas em contratos inteligentes. Segundo estudos recentes (Zhou et al., 2024), frameworks que utilizam linguagens seguras, como o Rust, são menos propensos a ataques como overflow de inteiros e erros de memória.
  
  ---
  
  ## **Conclusão**
  
  O Soroban simplifica o desenvolvimento de contratos inteligentes, mantendo um alto padrão de segurança e escalabilidade. Este exemplo básico "Hello World" serve como ponto de partida para explorar as possibilidades dessa tecnologia, desde aplicações financeiras até sistemas descentralizados complexos. A integração entre o Soroban e a rede Stellar demonstra como é possível criar soluções robustas e prontas para o mercado com rapidez e eficiência.
  
  Para desenvolvedores que desejam mergulhar no universo da blockchain, o Soroban apresenta uma oportunidade de explorar o potencial dos contratos inteligentes com segurança e flexibilidade.
  
  ---
  
  ## **Referências**
  
  BUTERIN, Vitalik et al. A next-generation smart contract and decentralized application platform. **white paper**, v. 3, n. 37, 	 p. 2-1, 2014.
  
  KARAARSLAN, Enis; KONACAKLI, Enis. Data storage in the decentralized world: Blockchain and derivatives. **arXiv preprint arXiv:2012.10253**, 2020.
  
  Rust Programming Language. (2025). [https://www.rust-lang.org/](https://www.rust-lang.org/)
  
  Soroban CLI Documentation. (2025). [https://soroban.stellar.org/docs](https://soroban.stellar.org/docs)
  
  Stellar. (2025). Soroban Smart Contracts. [https://soroban.stellar.org](https://soroban.stellar.org)
  
  WOOD, Gavin et al. Ethereum: A secure decentralised generalised transaction ledger. **Ethereum project yellow paper**, v. 151, n. 2014, p. 1-32, 2014.
  
  ZHANG, Ce et al. {COLE}: A Column-based Learned Storage for Blockchain Systems. In: **22nd USENIX Conference on File and Storage Technologies (FAST 24)**. 2024. p. 329-345.
  
  ZHOU, Zijian et al. Haina Storage: A Decentralized Secure Storage Framework Based on Improved Blockchain Structure. **arXiv preprint arXiv:2404.01606**, 2024.
