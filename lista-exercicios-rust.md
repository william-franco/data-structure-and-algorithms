# Lista de Exercícios de Rust — Algoritmos e Estruturas de Dados

Lista de exercícios de Rust organizada em duas categorias, **algorithms** e **data_structure**, cada uma dividida em tópicos com 10 exercícios cada — totalizando 100 exercícios.

## Regras

- O código completo de cada questão deve ficar somente em um único arquivo, sem depender de outros arquivos.
- Utilizar apenas a biblioteca padrão do Rust (`std`), sem crates externos.

## Estrutura de Projeto

```
rust-algoritmos-estrutura-dados/
├── algorithms/
│   ├── sorting/
│   │   ├── questao_01.rs
│   │   ├── questao_02.rs
│   │   ├── questao_03.rs
│   │   ├── questao_04.rs
│   │   ├── questao_05.rs
│   │   ├── questao_06.rs
│   │   ├── questao_07.rs
│   │   ├── questao_08.rs
│   │   ├── questao_09.rs
│   │   └── questao_10.rs
│   ├── searching/
│   │   ├── questao_01.rs
│   │   ├── questao_02.rs
│   │   ├── questao_03.rs
│   │   ├── questao_04.rs
│   │   ├── questao_05.rs
│   │   ├── questao_06.rs
│   │   ├── questao_07.rs
│   │   ├── questao_08.rs
│   │   ├── questao_09.rs
│   │   └── questao_10.rs
│   ├── recursion-and-backtracking/
│   │   ├── questao_01.rs
│   │   ├── questao_02.rs
│   │   ├── questao_03.rs
│   │   ├── questao_04.rs
│   │   ├── questao_05.rs
│   │   ├── questao_06.rs
│   │   ├── questao_07.rs
│   │   ├── questao_08.rs
│   │   ├── questao_09.rs
│   │   └── questao_10.rs
│   └── dynamic-programming/
│       ├── questao_01.rs
│       ├── questao_02.rs
│       ├── questao_03.rs
│       ├── questao_04.rs
│       ├── questao_05.rs
│       ├── questao_06.rs
│       ├── questao_07.rs
│       ├── questao_08.rs
│       ├── questao_09.rs
│       └── questao_10.rs
└── data_structure/
    ├── arrays-and-strings/
    │   ├── questao_01.rs
    │   ├── questao_02.rs
    │   ├── questao_03.rs
    │   ├── questao_04.rs
    │   ├── questao_05.rs
    │   ├── questao_06.rs
    │   ├── questao_07.rs
    │   ├── questao_08.rs
    │   ├── questao_09.rs
    │   └── questao_10.rs
    ├── linked-lists/
    │   ├── questao_01.rs
    │   ├── questao_02.rs
    │   ├── questao_03.rs
    │   ├── questao_04.rs
    │   ├── questao_05.rs
    │   ├── questao_06.rs
    │   ├── questao_07.rs
    │   ├── questao_08.rs
    │   ├── questao_09.rs
    │   └── questao_10.rs
    ├── stacks-and-queues/
    │   ├── questao_01.rs
    │   ├── questao_02.rs
    │   ├── questao_03.rs
    │   ├── questao_04.rs
    │   ├── questao_05.rs
    │   ├── questao_06.rs
    │   ├── questao_07.rs
    │   ├── questao_08.rs
    │   ├── questao_09.rs
    │   └── questao_10.rs
    ├── trees/
    │   ├── questao_01.rs
    │   ├── questao_02.rs
    │   ├── questao_03.rs
    │   ├── questao_04.rs
    │   ├── questao_05.rs
    │   ├── questao_06.rs
    │   ├── questao_07.rs
    │   ├── questao_08.rs
    │   ├── questao_09.rs
    │   └── questao_10.rs
    ├── graphs/
    │   ├── questao_01.rs
    │   ├── questao_02.rs
    │   ├── questao_03.rs
    │   ├── questao_04.rs
    │   ├── questao_05.rs
    │   ├── questao_06.rs
    │   ├── questao_07.rs
    │   ├── questao_08.rs
    │   ├── questao_09.rs
    │   └── questao_10.rs
    └── hash-tables/
        ├── questao_01.rs
        ├── questao_02.rs
        ├── questao_03.rs
        ├── questao_04.rs
        ├── questao_05.rs
        ├── questao_06.rs
        ├── questao_07.rs
        ├── questao_08.rs
        ├── questao_09.rs
        └── questao_10.rs
```

## algorithms/sorting — Sorting

### 1. Bubble sort
**Arquivo:** `algorithms/sorting/questao_01.rs`

Implemente o algoritmo de ordenação Bubble Sort para ordenar um `Vec<i32>` em ordem crescente, sem utilizar o método `.sort()` da biblioteca padrão.

### 2. Selection sort
**Arquivo:** `algorithms/sorting/questao_02.rs`

Implemente o algoritmo de ordenação Selection Sort para ordenar um `Vec<i32>` em ordem crescente, sem utilizar o método `.sort()` da biblioteca padrão.

### 3. Insertion sort
**Arquivo:** `algorithms/sorting/questao_03.rs`

Implemente o algoritmo de ordenação Insertion Sort para ordenar um `Vec<i32>` em ordem crescente, sem utilizar o método `.sort()` da biblioteca padrão.

### 4. Merge sort
**Arquivo:** `algorithms/sorting/questao_04.rs`

Implemente o algoritmo de ordenação Merge Sort (divisão e conquista) para ordenar um `Vec<i32>` em ordem crescente.

### 5. Quick sort
**Arquivo:** `algorithms/sorting/questao_05.rs`

Implemente o algoritmo de ordenação Quick Sort para ordenar um `Vec<i32>` em ordem crescente.

### 6. Heap sort
**Arquivo:** `algorithms/sorting/questao_06.rs`

Implemente o algoritmo de ordenação Heap Sort, construindo um heap máximo manualmente (sem usar `std::collections::BinaryHeap`) e extraindo os elementos ordenadamente.

### 7. Counting sort
**Arquivo:** `algorithms/sorting/questao_07.rs`

Implemente o algoritmo de ordenação Counting Sort para ordenar um `Vec<i32>` de números não-negativos dentro de um intervalo conhecido.

### 8. Radix sort
**Arquivo:** `algorithms/sorting/questao_08.rs`

Implemente o algoritmo de ordenação Radix Sort para ordenar um `Vec<u32>`, processando dígito a dígito.

### 9. Shell sort
**Arquivo:** `algorithms/sorting/questao_09.rs`

Implemente o algoritmo de ordenação Shell Sort, utilizando uma sequência de intervalos (gaps) decrescente.

### 10. Comparação de algoritmos de ordenação
**Arquivo:** `algorithms/sorting/questao_10.rs`

Utilizando `std::time::Instant`, escreva um programa que compare o tempo de execução de pelo menos três algoritmos de ordenação implementados anteriormente, para vetores de tamanhos crescentes.

## algorithms/searching — Searching

### 1. Busca linear
**Arquivo:** `algorithms/searching/questao_01.rs`

Implemente uma função que realize uma busca linear por um valor em um `Vec<i32>`, retornando `Option<usize>` com o índice encontrado.

### 2. Busca binária iterativa
**Arquivo:** `algorithms/searching/questao_02.rs`

Implemente uma função de busca binária iterativa em um `Vec<i32>` ordenado, retornando `Option<usize>` com o índice do valor buscado.

### 3. Busca binária recursiva
**Arquivo:** `algorithms/searching/questao_03.rs`

Implemente uma função de busca binária recursiva em um slice `&[i32]` ordenado, retornando `Option<usize>` com o índice do valor buscado.

### 4. Primeira e última ocorrência
**Arquivo:** `algorithms/searching/questao_04.rs`

Implemente uma função que encontre o primeiro e o último índice de um valor em um `Vec<i32>` ordenado que pode conter elementos repetidos, utilizando busca binária.

### 5. Busca em matriz ordenada
**Arquivo:** `algorithms/searching/questao_05.rs`

Implemente uma função que busque um valor em uma matriz (`Vec<Vec<i32>>`) cujas linhas e colunas estão ordenadas de forma crescente, em tempo eficiente.

### 6. Ponto de rotação em array ordenado
**Arquivo:** `algorithms/searching/questao_06.rs`

Implemente uma função que encontre o índice do menor elemento (ponto de rotação) em um `Vec<i32>` ordenado que foi rotacionado.

### 7. Busca binária em array rotacionado
**Arquivo:** `algorithms/searching/questao_07.rs`

Implemente uma função de busca binária adaptada para encontrar um valor em um `Vec<i32>` ordenado que foi rotacionado em um ponto desconhecido.

### 8. Busca interpolada
**Arquivo:** `algorithms/searching/questao_08.rs`

Implemente o algoritmo de busca interpolada (interpolation search) para encontrar um valor em um `Vec<i32>` ordenado e uniformemente distribuído.

### 9. Busca exponencial
**Arquivo:** `algorithms/searching/questao_09.rs`

Implemente o algoritmo de busca exponencial (exponential search) para encontrar um valor em um `Vec<i32>` ordenado.

### 10. K-ésimo menor elemento (Quickselect)
**Arquivo:** `algorithms/searching/questao_10.rs`

Implemente o algoritmo Quickselect para encontrar o k-ésimo menor elemento de um `Vec<i32>` não ordenado, sem ordenar o vetor inteiro.

## algorithms/recursion-and-backtracking — Recursion and Backtracking

### 1. Torres de Hanói
**Arquivo:** `algorithms/recursion-and-backtracking/questao_01.rs`

Implemente a solução recursiva para o problema da Torre de Hanói, imprimindo os movimentos necessários para mover N discos entre três torres.

### 2. Combinações de N elementos tomados K a K
**Arquivo:** `algorithms/recursion-and-backtracking/questao_02.rs`

Implemente uma função recursiva que gere todas as combinações possíveis de K elementos a partir de um `Vec<i32>` de N elementos.

### 3. Permutações
**Arquivo:** `algorithms/recursion-and-backtracking/questao_03.rs`

Implemente uma função recursiva (backtracking) que gere todas as permutações possíveis dos elementos de um `Vec<i32>`.

### 4. N-Rainhas
**Arquivo:** `algorithms/recursion-and-backtracking/questao_04.rs`

Implemente uma solução por backtracking para o problema das N-Rainhas, retornando todas as disposições válidas em um tabuleiro N x N.

### 5. Resolver labirinto
**Arquivo:** `algorithms/recursion-and-backtracking/questao_05.rs`

Implemente uma função de backtracking que encontre um caminho da entrada até a saída de um labirinto representado por uma `Vec<Vec<i32>>`, evitando obstáculos.

### 6. Subconjuntos (power set)
**Arquivo:** `algorithms/recursion-and-backtracking/questao_06.rs`

Implemente uma função recursiva que gere todos os subconjuntos possíveis (o conjunto das partes) de um `Vec<i32>`.

### 7. Resolver Sudoku
**Arquivo:** `algorithms/recursion-and-backtracking/questao_07.rs`

Implemente uma solução por backtracking que preencha um tabuleiro de Sudoku 9x9 (`[[u8; 9]; 9]`) parcialmente preenchido, respeitando as regras do jogo.

### 8. Parênteses balanceados
**Arquivo:** `algorithms/recursion-and-backtracking/questao_08.rs`

Implemente uma função de backtracking que gere todas as combinações válidas de N pares de parênteses balanceados, retornando um `Vec<String>`.

### 9. Soma de subconjuntos (Subset Sum)
**Arquivo:** `algorithms/recursion-and-backtracking/questao_09.rs`

Implemente uma função de backtracking que verifique se existe um subconjunto de um `Vec<i32>` cuja soma seja igual a um valor alvo.

### 10. Caixeiro viajante (força bruta)
**Arquivo:** `algorithms/recursion-and-backtracking/questao_10.rs`

Implemente uma solução recursiva de força bruta para o problema do caixeiro viajante, encontrando a rota de menor custo entre um pequeno conjunto de cidades representado por uma matriz de distâncias.

## algorithms/dynamic-programming — Dynamic Programming

### 1. Fibonacci com memoization
**Arquivo:** `algorithms/dynamic-programming/questao_01.rs`

Implemente uma função que calcule o n-ésimo termo da sequência de Fibonacci utilizando memoization (um `HashMap<u64, u64>` como cache de resultados já calculados), evitando recomputações.

### 2. Problema da mochila 0/1
**Arquivo:** `algorithms/dynamic-programming/questao_02.rs`

Implemente a solução do problema da mochila 0/1 (0/1 Knapsack) utilizando programação dinâmica (tabela `Vec<Vec<i32>>`), maximizando o valor total sem exceder a capacidade da mochila.

### 3. Maior subsequência comum (LCS)
**Arquivo:** `algorithms/dynamic-programming/questao_03.rs`

Implemente, utilizando programação dinâmica, uma função que encontre o comprimento da maior subsequência comum (LCS) entre duas strings (`&str`).

### 4. Maior subsequência crescente (LIS)
**Arquivo:** `algorithms/dynamic-programming/questao_04.rs`

Implemente, utilizando programação dinâmica, uma função que encontre o comprimento da maior subsequência estritamente crescente em um `Vec<i32>`.

### 5. Troca de moedas (Coin Change)
**Arquivo:** `algorithms/dynamic-programming/questao_05.rs`

Implemente, utilizando programação dinâmica, uma função que encontre o número mínimo de moedas necessárias para totalizar um valor alvo, dado um `Vec<i32>` com os valores de moedas disponíveis.

### 6. Distância de edição (Edit Distance)
**Arquivo:** `algorithms/dynamic-programming/questao_06.rs`

Implemente, utilizando programação dinâmica, uma função que calcule o número mínimo de operações (inserção, remoção, substituição) necessárias para transformar uma string em outra.

### 7. Caminho de soma mínima em grade
**Arquivo:** `algorithms/dynamic-programming/questao_07.rs`

Implemente, utilizando programação dinâmica, uma função que encontre o caminho de soma mínima do canto superior esquerdo ao canto inferior direito de uma `Vec<Vec<i32>>`, movendo-se apenas para baixo ou para a direita.

### 8. Partição de conjunto (Equal Subset Sum)
**Arquivo:** `algorithms/dynamic-programming/questao_08.rs`

Implemente, utilizando programação dinâmica, uma função que verifique se um `Vec<i32>` pode ser particionado em dois subconjuntos com somas iguais.

### 9. Escalada de escada (Climbing Stairs)
**Arquivo:** `algorithms/dynamic-programming/questao_09.rs`

Implemente, utilizando programação dinâmica, uma função que calcule de quantas maneiras distintas é possível subir uma escada de N degraus, podendo subir 1 ou 2 degraus por vez.

### 10. Corte de hastes (Rod Cutting)
**Arquivo:** `algorithms/dynamic-programming/questao_10.rs`

Implemente, utilizando programação dinâmica, uma função que determine o valor máximo obtido ao cortar uma haste de comprimento N em pedaços, dado um `Vec<i32>` com preços por tamanho de pedaço.

## data_structure/arrays-and-strings — Arrays and Strings

### 1. Segundo maior elemento
**Arquivo:** `data_structure/arrays-and-strings/questao_01.rs`

Implemente uma função que encontre o segundo maior elemento de um `Vec<i32>`, sem ordenar o vetor.

### 2. Remover duplicatas mantendo a ordem
**Arquivo:** `data_structure/arrays-and-strings/questao_02.rs`

Implemente uma função que remova elementos duplicados de um `Vec<i32>`, mantendo a ordem original de aparição, sem utilizar `HashSet`.

### 3. Verificar anagramas
**Arquivo:** `data_structure/arrays-and-strings/questao_03.rs`

Implemente uma função que verifique se duas strings (`&str`) são anagramas uma da outra (mesmas letras, possivelmente em ordem diferente).

### 4. Two Sum
**Arquivo:** `data_structure/arrays-and-strings/questao_04.rs`

Implemente uma função que receba um `Vec<i32>` e um valor alvo, retornando os índices dos dois elementos cuja soma seja igual ao alvo.

### 5. Rotação de array
**Arquivo:** `data_structure/arrays-and-strings/questao_05.rs`

Implemente uma função que rotacione os elementos de um `Vec<i32>` em k posições para a direita.

### 6. Elemento majoritário
**Arquivo:** `data_structure/arrays-and-strings/questao_06.rs`

Implemente uma função que encontre o elemento que aparece mais de N/2 vezes em um `Vec<i32>` de tamanho N (assumindo que ele existe), utilizando o algoritmo de votação de Boyer-Moore.

### 7. Caracteres únicos
**Arquivo:** `data_structure/arrays-and-strings/questao_07.rs`

Implemente uma função que verifique se uma string possui todos os caracteres únicos, sem utilizar estruturas de dados auxiliares (como `HashSet` ou `HashMap`).

### 8. Mesclar arrays ordenados
**Arquivo:** `data_structure/arrays-and-strings/questao_08.rs`

Implemente uma função que receba dois `Vec<i32>` já ordenados e retorne um único `Vec<i32>` ordenado contendo todos os elementos, sem utilizar `.sort()`.

### 9. Subarray de soma máxima
**Arquivo:** `data_structure/arrays-and-strings/questao_09.rs`

Implemente o algoritmo de Kadane para encontrar a soma máxima de um subarray contíguo em um `Vec<i32>` (que pode conter valores negativos).

### 10. Rotação válida de string
**Arquivo:** `data_structure/arrays-and-strings/questao_10.rs`

Implemente uma função que verifique se uma string é uma rotação válida de outra (ex: `"waterbottle"` é uma rotação de `"erbottlewat"`).

## data_structure/linked-lists — Linked Lists

### 1. Lista encadeada simples
**Arquivo:** `data_structure/linked-lists/questao_01.rs`

Implemente uma struct `LinkedList` (utilizando `Option<Box<No>>`) com métodos para inserir no início, inserir no final e converter para `Vec<i32>`.

### 2. Inverter lista encadeada
**Arquivo:** `data_structure/linked-lists/questao_02.rs`

Implemente um método que inverta uma lista encadeada simples, retornando a nova cabeça da lista.

### 3. Detectar ciclo
**Arquivo:** `data_structure/linked-lists/questao_03.rs`

Implemente uma função que detecte se uma lista encadeada possui um ciclo, utilizando o algoritmo de Floyd (dois ponteiros: lento e rápido).

### 4. Elemento do meio
**Arquivo:** `data_structure/linked-lists/questao_04.rs`

Implemente uma função que encontre o elemento do meio de uma lista encadeada percorrendo-a apenas uma vez, utilizando dois ponteiros.

### 5. Remover duplicatas
**Arquivo:** `data_structure/linked-lists/questao_05.rs`

Implemente um método que remova elementos duplicados de uma lista encadeada não ordenada.

### 6. Mesclar listas ordenadas
**Arquivo:** `data_structure/linked-lists/questao_06.rs`

Implemente uma função que mescle duas listas encadeadas já ordenadas em uma única lista encadeada ordenada.

### 7. Remover o N-ésimo nó do final
**Arquivo:** `data_structure/linked-lists/questao_07.rs`

Implemente um método que remova o N-ésimo nó a partir do final de uma lista encadeada, percorrendo a lista apenas uma vez.

### 8. Palíndromo em lista encadeada
**Arquivo:** `data_structure/linked-lists/questao_08.rs`

Implemente uma função que verifique se uma lista encadeada representa um palíndromo.

### 9. Lista duplamente encadeada
**Arquivo:** `data_structure/linked-lists/questao_09.rs`

Implemente uma struct de lista duplamente encadeada com métodos para inserir e remover elementos em ambas as extremidades.

### 10. Soma de números em listas encadeadas
**Arquivo:** `data_structure/linked-lists/questao_10.rs`

Implemente uma função que receba dois números representados como listas encadeadas (um dígito por nó, em ordem inversa) e retorne a soma como uma nova lista encadeada.

## data_structure/stacks-and-queues — Stacks and Queues

### 1. Pilha com Vec
**Arquivo:** `data_structure/stacks-and-queues/questao_01.rs`

Implemente uma struct `Pilha<T>` utilizando `Vec<T>` internamente, com os métodos `empilhar`, `desempilhar` e `topo`.

### 2. Fila com duas pilhas
**Arquivo:** `data_structure/stacks-and-queues/questao_02.rs`

Implemente uma struct `Fila<T>` que utilize internamente duas pilhas (`Vec<T>`) para simular o comportamento FIFO.

### 3. Balanceamento de parênteses
**Arquivo:** `data_structure/stacks-and-queues/questao_03.rs`

Implemente uma função que utilize uma pilha para verificar se uma string contendo parênteses, colchetes e chaves está corretamente balanceada.

### 4. Fila circular
**Arquivo:** `data_structure/stacks-and-queues/questao_04.rs`

Implemente uma struct de fila circular com capacidade fixa, reaproveitando os espaços liberados após remoções.

### 5. Avaliação de expressão pósfixa
**Arquivo:** `data_structure/stacks-and-queues/questao_05.rs`

Implemente uma função que utilize uma pilha para avaliar uma expressão matemática escrita em notação pósfixa (RPN).

### 6. Pilha com valor mínimo em O(1)
**Arquivo:** `data_structure/stacks-and-queues/questao_06.rs`

Implemente uma struct de pilha que, além das operações usuais, ofereça um método `obter_minimo` que retorne o menor elemento em tempo O(1).

### 7. Infixa para pósfixa
**Arquivo:** `data_structure/stacks-and-queues/questao_07.rs`

Implemente uma função que utilize uma pilha para converter uma expressão matemática da notação infixa para a notação pósfixa.

### 8. Fila de prioridade simples
**Arquivo:** `data_structure/stacks-and-queues/questao_08.rs`

Implemente uma struct de fila de prioridade simples (sem usar `std::collections::BinaryHeap`), onde o elemento de maior prioridade é sempre removido primeiro.

### 9. Histórico de navegador
**Arquivo:** `data_structure/stacks-and-queues/questao_09.rs`

Implemente uma struct que simule o histórico de navegação de um navegador (métodos `visitar`, `voltar` e `avancar`), utilizando duas pilhas.

### 10. Deque (fila de duas pontas)
**Arquivo:** `data_structure/stacks-and-queues/questao_10.rs`

Implemente uma struct de deque (sem usar `std::collections::VecDeque`) com métodos para inserir e remover elementos tanto no início quanto no final.

## data_structure/trees — Trees

### 1. Árvore binária de busca
**Arquivo:** `data_structure/trees/questao_01.rs`

Implemente uma struct de árvore binária de busca (ABB), utilizando `Option<Box<No>>`, com os métodos `inserir`, `buscar` e `remover`.

### 2. Percursos em árvore binária
**Arquivo:** `data_structure/trees/questao_02.rs`

Implemente três métodos que percorram uma árvore binária em pré-ordem, em-ordem e pós-ordem, retornando os valores visitados em um `Vec<i32>`.

### 3. Altura da árvore
**Arquivo:** `data_structure/trees/questao_03.rs`

Implemente uma função que calcule a altura (profundidade máxima) de uma árvore binária.

### 4. Árvore balanceada
**Arquivo:** `data_structure/trees/questao_04.rs`

Implemente uma função que verifique se uma árvore binária é balanceada, ou seja, se a diferença de altura entre as subárvores esquerda e direita de qualquer nó não é maior que 1.

### 5. Ancestral comum mais próximo (LCA)
**Arquivo:** `data_structure/trees/questao_05.rs`

Implemente uma função que encontre o ancestral comum mais próximo (LCA) de dois nós em uma árvore binária de busca.

### 6. Árvores idênticas
**Arquivo:** `data_structure/trees/questao_06.rs`

Implemente uma função que verifique se duas árvores binárias são estruturalmente idênticas e possuem os mesmos valores em cada nó.

### 7. ABB para lista ordenada
**Arquivo:** `data_structure/trees/questao_07.rs`

Implemente uma função que converta uma árvore binária de busca em um `Vec<i32>` ordenado, utilizando percurso em-ordem.

### 8. Percurso em largura (BFS)
**Arquivo:** `data_structure/trees/questao_08.rs`

Implemente uma função que percorra uma árvore binária em largura (nível a nível), utilizando `std::collections::VecDeque`, retornando os valores agrupados por nível.

### 9. Validar árvore binária de busca
**Arquivo:** `data_structure/trees/questao_09.rs`

Implemente uma função que verifique se uma árvore binária é uma árvore binária de busca válida.

### 10. Espelhar árvore binária
**Arquivo:** `data_structure/trees/questao_10.rs`

Implemente uma função que inverta (espelhe) uma árvore binária, trocando as subárvores esquerda e direita de cada nó recursivamente.

## data_structure/graphs — Graphs

### 1. Grafo com lista de adjacência
**Arquivo:** `data_structure/graphs/questao_01.rs`

Implemente uma struct de grafo (direcionado ou não), utilizando `HashMap<i32, Vec<i32>>` como lista de adjacência, com métodos para adicionar vértices e arestas.

### 2. Busca em profundidade (DFS)
**Arquivo:** `data_structure/graphs/questao_02.rs`

Implemente uma função que percorra um grafo em profundidade (DFS) a partir de um vértice inicial, retornando a ordem de visitação em um `Vec<i32>`.

### 3. Busca em largura (BFS)
**Arquivo:** `data_structure/graphs/questao_03.rs`

Implemente uma função que percorra um grafo em largura (BFS), utilizando `std::collections::VecDeque`, a partir de um vértice inicial, retornando a ordem de visitação.

### 4. Detectar ciclo em grafo direcionado
**Arquivo:** `data_structure/graphs/questao_04.rs`

Implemente uma função que detecte se um grafo direcionado possui algum ciclo.

### 5. Detectar ciclo em grafo não-direcionado
**Arquivo:** `data_structure/graphs/questao_05.rs`

Implemente uma função que detecte se um grafo não-direcionado possui algum ciclo.

### 6. Ordenação topológica
**Arquivo:** `data_structure/graphs/questao_06.rs`

Implemente uma função que realize a ordenação topológica de um grafo direcionado acíclico (DAG).

### 7. Caminho mais curto (não ponderado)
**Arquivo:** `data_structure/graphs/questao_07.rs`

Implemente uma função que encontre o caminho mais curto entre dois vértices em um grafo não ponderado, utilizando BFS.

### 8. Algoritmo de Dijkstra
**Arquivo:** `data_structure/graphs/questao_08.rs`

Implemente o algoritmo de Dijkstra para encontrar o caminho de menor custo entre um vértice de origem e todos os demais vértices de um grafo ponderado, utilizando `std::collections::BinaryHeap`.

### 9. Grafo conexo
**Arquivo:** `data_structure/graphs/questao_09.rs`

Implemente uma função que verifique se um grafo não-direcionado é conexo, ou seja, se existe caminho entre todos os pares de vértices.

### 10. Componentes conectados
**Arquivo:** `data_structure/graphs/questao_10.rs`

Implemente uma função que conte o número de componentes conectados em um grafo não-direcionado.

## data_structure/hash-tables — Hash Tables

### 1. Tabela hash com encadeamento
**Arquivo:** `data_structure/hash-tables/questao_01.rs`

Implemente uma struct de tabela hash do zero (sem usar `HashMap` da biblioteca padrão), tratando colisões por encadeamento (cada posição do vetor armazena uma lista de pares chave-valor).

### 2. Tabela hash com endereçamento aberto
**Arquivo:** `data_structure/hash-tables/questao_02.rs`

Implemente uma struct de tabela hash do zero (sem usar `HashMap` da biblioteca padrão), tratando colisões por endereçamento aberto (linear probing).

### 3. Anagramas com tabela hash
**Arquivo:** `data_structure/hash-tables/questao_03.rs`

Implemente uma função que verifique se duas strings são anagramas utilizando um `HashMap<char, i32>` para contar a frequência dos caracteres.

### 4. Primeiro elemento não repetido
**Arquivo:** `data_structure/hash-tables/questao_04.rs`

Implemente uma função que encontre o primeiro elemento não repetido em um `Vec<i32>`, utilizando um `HashMap` para contar as ocorrências.

### 5. Agrupar anagramas
**Arquivo:** `data_structure/hash-tables/questao_05.rs`

Implemente uma função que receba um `Vec<String>` de palavras e agrupe, utilizando um `HashMap`, todas as que são anagramas entre si.

### 6. Two Sum otimizado com hash
**Arquivo:** `data_structure/hash-tables/questao_06.rs`

Implemente uma versão otimizada do problema Two Sum, utilizando um `HashMap<i32, usize>` para encontrar o par de números que somam um valor alvo em tempo O(n).

### 7. Cache LRU
**Arquivo:** `data_structure/hash-tables/questao_07.rs`

Implemente uma struct de cache LRU (Least Recently Used) com capacidade fixa, utilizando um `HashMap` combinado com uma lista duplamente encadeada.

### 8. Contagem de frequência
**Arquivo:** `data_structure/hash-tables/questao_08.rs`

Implemente uma função que conte a frequência de cada elemento em um `Vec<i32>`, utilizando um `HashMap<i32, i32>`.

### 9. Subarray mais longo com soma zero
**Arquivo:** `data_structure/hash-tables/questao_09.rs`

Implemente, utilizando um `HashMap`, uma função que encontre o comprimento do subarray contíguo mais longo de um `Vec<i32>` cuja soma dos elementos seja igual a zero.

### 10. Detectar duplicatas
**Arquivo:** `data_structure/hash-tables/questao_10.rs`

Implemente uma função que detecte se um `Vec<i32>` contém elementos duplicados, utilizando um `HashSet` para verificação em tempo O(n).
