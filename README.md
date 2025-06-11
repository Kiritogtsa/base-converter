# 🧮 Base Converter

Este projeto foi criado com o objetivo de aprender os fundamentos da linguagem **Rust**, com foco especial em:

- Aprender o uso de **`struct`** e **`trait`**
- Organizar código modularmente
- Entender a teoria dos sistemas numéricos e suas conversões
- Implementar manualmente as conversões entre bases numéricas

Atualmente, o projeto suporta conversões entre as seguintes bases numéricas:

- Binário ↔ Decimal  
- Decimal ↔ Hexadecimal  
- Binário ↔ Hexadecimal  

> A base **octal** foi descartada para simplificação do projeto.

---

## 🚀 Objetivos

- Explorar conceitos básicos e avançados de Rust, como estruturas e traits.
- Criar um conversor de bases numéricas que não utilize funções nativas prontas para conversão, para fixar os algoritmos.
- Entender o funcionamento dos sistemas binário, decimal e hexadecimal na prática.
- Desenvolver habilidades em tratamento de erros e validação de entradas.
- Preparar terreno para projetos futuros envolvendo manipulação mais complexa de números e sistemas.

---

## 🛠️ Funcionalidades atuais

- Conversão de números decimais para binário e hexadecimal
- Conversão de números binários para decimal e hexadecimal
- Conversão de números hexadecimais para decimal e binário
- Validação rigorosa das entradas com uso de expressões regulares
- Interface CLI simples para interação via terminal

---

## ✨ Features planejadas

Após concluir um projeto paralelo (implementação de um servidor HTTP do zero em Rust, com gerenciamento próprio de threads), as seguintes melhorias serão adicionadas:

- 🔧 **CLI aprimorada** usando a biblioteca [`console`](https://crates.io/crates/console) para melhorar a experiência do terminal com cores e formatação.
- ➗ **Implementação de frações** para suportar números racionais (frações) nas bases decimal e hexadecimal, representando números reais de forma precisa sem uso extensivo de ponto flutuante.  
  > Nota: frações **não serão implementadas para base binária**, pois sua representação é pouco prática neste formato.
- 📐 **Representação dos conjuntos numéricos**: inclusão dos conceitos de números naturais (ℕ), inteiros (ℤ) e racionais (ℚ) para validar e organizar as conversões.
  
---

## 🚀 Como usar

Execute o projeto com:

```bash
cargo run

