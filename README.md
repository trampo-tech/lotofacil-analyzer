# Lotofácil Analyzer

## Visão Geral

O Lotofácil Analyzer é uma aplicação desenvolvida em Rust para análise combinatória e geração de soluções otimizadas para o jogo Lotofácil, uma loteria brasileira. O objetivo principal é gerar, analisar e cobrir subconjuntos de combinações de 15 números (S15) a partir de subconjuntos menores (S14, S13, S12, S11), buscando estratégias que minimizem o número de apostas necessárias para cobrir todas as possibilidades relevantes.

## Funcionalidades

- **Geração de combinações**: Gera todos os subconjuntos possíveis de 11 a 15 números dentre os 25 disponíveis, salvando-os em arquivos CSV.
- **Cobertura de subconjuntos**: Para cada S15, busca cobrir todas as combinações S14, S13, S12 e S11, utilizando algoritmos que selecionam subconjuntos de S15 capazes de cobrir o máximo possível de subconjuntos menores.
- **Análise de custo**: Calcula o custo financeiro total das apostas geradas, considerando o valor unitário de cada jogo.
- **Visualização de resultados**: Exibe o número de combinações geradas para cada abordagem e exercício.
- **Limpeza de diretórios**: Remove arquivos de saída para facilitar novas execuções.

## Conceitos-Chave

- **Cobertura de Subconjuntos**: O conceito central é selecionar subconjuntos de 15 números (S15) que cubram, por meio de suas combinações, todos os subconjuntos de tamanho menor (S14, S13, S12, S11). Isso é feito removendo 1, 2, 3 ou 4 números de cada S15 para formar os subconjuntos menores.
- **Bitmask**: Para eficiência, cada combinação é representada como um bitmask (`u32`), onde cada bit indica a presença de um número. Isso permite operações rápidas de comparação e armazenamento em `HashSet`.
- **Seed Aleatória/Reprodutibilidade**: O uso de seeds permite reprodutibilidade dos experimentos e também a geração de soluções diferentes para análise comparativa.

## Approaches para Redução de Custo Computacional

- **Uso de Bitmask**: Representar combinações como bitmask reduz o uso de memória e acelera operações de inclusão/remoção em conjuntos.
- **Algoritmos Gulosos Adaptativos**: Os algoritmos de cobertura utilizam estratégias gulosas, selecionando S15 que cobrem o maior número de subconjuntos ainda não cobertos. Limiares adaptativos são usados para balancear entre qualidade da cobertura e performance.
- **Shuffling e Randomização**: As combinações são embaralhadas para evitar viés e permitir diferentes execuções com resultados variados.

## Estrutura dos Exercícios

- **Exercício 1**: Geração de todas as combinações possíveis de 11 a 15 números.
- **Exercício 2**: Seleção de S15 para cobrir todas as S14.
- **Exercício 3**: Seleção de S15 para cobrir todas as S13.
- **Exercício 4**: Seleção de S15 para cobrir todas as S12.
- **Exercício 5**: Seleção de S15 para cobrir todas as S11.
- **Exercício 6**: Execução sequencial dos exercícios 2 a 5 para análise de tempo.
- **Exercício 7**: Cálculo do custo financeiro das apostas geradas.
- **Mostrar Resultados**: Exibe o número de combinações S15 geradas em cada abordagem.
- **Limpar**: Limpa a pasta de saída para novas execuções.


## Como Executar

1. Instale o Rust e as dependências.
2. Compile o projeto com `cargo build --release`.
3. Execute com `cargo run --release` e siga o menu interativo.

## Observações

- O tempo de execução pode ser elevado para exercícios que envolvem grandes volumes de combinações.
- Recomenda-se executar em máquinas com boa quantidade de memória RAM e múltiplos núcleos de CPU.
- O uso de seeds permite reprodutibilidade e análise de diferentes soluções.

---

Desenvolvido para fins de estudo e análise combinatória do Lotofácil.