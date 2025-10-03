# rust-ai-trading-bot

**An intelligent trading bot using machine learning in Rust to predict market movements.**

---

## 🇧🇷 Descrição em Português

`rust-ai-trading-bot` é um bot de trading que utiliza um modelo de aprendizado de máquina (Decision Tree) para prever sinais de compra e venda com base em features de mercado. Construído inteiramente em Rust, este projeto demonstra a integração de bibliotecas de ML como `linfa` em um ambiente de alta performance.

Este é o quarto de uma série de cinco repositórios focados em trading, mercado financeiro e IA, destacando o uso de Rust para criar sistemas de trading inteligentes e autônomos.

### Funcionalidades

- **Modelo de Machine Learning:** Utiliza um modelo de Decision Tree da biblioteca `linfa` para classificação de sinais.
- **Treinamento e Predição:** Funções para treinar o modelo com dados históricos e fazer predições em novos dados.
- **Integração com Polars:** Usa o `polars` para manipulação e preparação de dados de forma eficiente.
- **Estrutura Modular:** O código é organizado em crates para `core`, `ml`, `data` e `utils`, promovendo um design limpo e modular.

---

## 🇺🇸 English Description

`rust-ai-trading-bot` is a trading bot that uses a machine learning model (Decision Tree) to predict buy and sell signals based on market features. Built entirely in Rust, this project demonstrates the integration of ML libraries like `linfa` in a high-performance environment.

This is the fourth in a series of five repositories focused on trading, the financial market, and AI, highlighting the use of Rust to create intelligent and autonomous trading systems.

### Features

- **Machine Learning Model:** Uses a Decision Tree model from the `linfa` library for signal classification.
- **Training and Prediction:** Functions to train the model with historical data and make predictions on new data.
- **Polars Integration:** Uses `polars` for efficient data manipulation and preparation.
- **Modular Structure:** The code is organized into crates for `core`, `ml`, `data`, and `utils`, promoting a clean and modular design.

---

## 🚀 Quick Start

### Pré-requisitos

- Rust (https://www.rust-lang.org/tools/install)
- Git

### Instalação

1. Clone o repositório:
```bash
git clone https://github.com/your-username/rust-ai-trading-bot.git
cd rust-ai-trading-bot
```

2. Compile e execute o exemplo:
```bash
cargo run --example ai_trading_bot
```

### Exemplo de Saída

O exemplo irá carregar os dados de sinais de mercado, treinar um modelo de Decision Tree e imprimir as predições.

```
Predictions:
shape: (10,)
Series: 'predictions' [u32]
[
	1
	0
	1
	0
	1
	0
	1
	0
	1
	0
]
```

---

## 🏛️ Arquitetura

O bot é construído em torno de um fluxo de trabalho de ML, desde a carga dos dados até a predição.

![Arquitetura do Bot de IA](https://i.imgur.com/O8c7f9d.png)

### Crates

- `ratb-core`: Orquestra o fluxo de treinamento e predição.
- `ratb-data`: Responsável por carregar e preparar os dados.
- `ratb-ml`: Contém a lógica de treinamento e predição do modelo de ML.
- `ratb-utils`: Funções utilitárias.

---

## 🛣️ Roadmap

- [ ] Implementar mais modelos de ML (ex: Random Forest, Gradient Boosting, Redes Neurais).
- [ ] Adicionar um backtesting engine para avaliar a performance do bot.
- [ ] Integração com uma corretora real para execução de ordens (paper trading/live trading).
- [ ] Desenvolver um sistema de feature engineering mais avançado.
- [ ] Criar uma API para interagir com o bot e obter predições.

---

## 🤝 Contribuição

Contribuições são bem-vindas! Sinta-se à vontade para abrir uma issue ou enviar um pull request.

---

## 📜 Licença

Este projeto está licenciado sob a licença MIT.

---

## 👨‍💻 Autor

**Gabriel Demetrios Lafis**

*   Cientista de Dados | Analista de Dados | BI/BA
*   Formado em Análise e Desenvolvimento de Sistemas, Gestão da Tecnologia da Informação e Segurança Cibernética.

