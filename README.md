# Secret Friend Sorter

Um aplicativo em Rust para realizar sorteios de amigo secreto de forma automatizada, enviando os resultados por email para todos os participantes.

## 📋 Características

- Suporta múltiplos sorteios por pessoa
- Envia emails automaticamente para cada participante
- Garante que ninguém tire a si mesmo
- Distribui os amigos secretos de forma equilibrada

## 🚀 Como usar

### 1. Configuração inicial

Primeiro, clone o repositório e instale as dependências:

```bash
git clone https://github.com/kszinhu/secret-friend-sorter
cd secret-friend-sorter
cargo build
```

### 2. Configure as variáveis de ambiente

Crie um arquivo `.env` na raiz do projeto com as seguintes informações:

```env
SMTP_SERVER=smtp.gmail.com
SMTP_USERNAME=seu_email@gmail.com
SMTP_PASSWORD=sua_senha_de_app
SMTP_PORT=587
```

> **Nota**: Para Gmail, você precisará criar uma senha de aplicativo específica nas configurações de segurança da sua conta Google.

### 3. Configure a lista de participantes

Crie um arquivo `friends.json` na raiz do projeto com a lista de participantes:

```json
[
  {
    "name": "João Silva",
    "email": "joao@example.com"
  },
  {
    "name": "Maria Santos",
    "email": "maria@example.com"
  }
  // ... adicione mais participantes
]
```

### 4. Execute o sorteio

```bash
cargo run
```

## 🎲 Como funciona

O programa utiliza um algoritmo que garante que cada pessoa receba um amigo secreto de forma justa e equilibrada. Aqui está uma explicação detalhada do processo:

1. **Validação Inicial**:

   - O algoritmo começa validando a lista de participantes e a quantidade de sorteios por pessoa.
   - Verifica se a quantidade de sorteios é maior que 0.
   - Verifica se o número de participantes é maior que 1.
   - Verifica se a quantidade de sorteios não é maior que o número de participantes.
   - Verifica se o número de participantes é par, garantindo que todos possam ser emparelhados.

2. **Preparação**:

   - Cria uma matriz de disponibilidade (`available_slots`) que rastreia quantas vezes cada pessoa ainda pode ser sorteada.
   - Inicializa um vetor (`result`) para armazenar os pares de amigos secretos.

3. **Sorteio**:

   - Para cada pessoa na lista de participantes:
     - Cria um conjunto (`chosen_friends`) para rastrear os amigos secretos já escolhidos por essa pessoa.
     - Para cada sorteio dessa pessoa:
       - Cria uma lista de candidatos válidos (`valid_receivers`), filtrando aqueles que:
         - Não são a própria pessoa.
         - Não foram escolhidos anteriormente por essa pessoa.
         - Ainda têm slots disponíveis para serem sorteados.
       - Se não houver candidatos válidos, o algoritmo retorna um erro indicando que não foi possível encontrar uma distribuição válida.
       - Ordena os candidatos válidos com base no número de slots disponíveis, em ordem decrescente.
       - Seleciona os melhores candidatos (`best_candidates`) que têm o maior número de slots disponíveis.
       - Escolhe aleatoriamente um amigo secreto dos melhores candidatos.
       - Atualiza o conjunto de amigos escolhidos e decrementa o número de slots disponíveis para o amigo secreto escolhido.
       - Adiciona o par de amigo secreto ao vetor de resultados.

4. **Resultado**:

   - Retorna o vetor de resultados contendo todos os pares de amigos secretos.

5. **Envio de Emails**:
   - Para cada par de amigo secreto, o programa envia um email para a pessoa informando quem é seu amigo secreto.
