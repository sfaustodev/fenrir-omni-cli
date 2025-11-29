# Fenrir Omni CLI

Fenrir Omni CLI é um playground de ferramentas de automação e segurança escrito em Rust. O workspace contém três binários principais:

- **fenrir_cli** (`src/fenrir_cli`): CLI polida com guardrails, configuração YAML e comandos de scan simulados.
- **fenrir** (`src/fenrir`): "God mode" experimental que mistura orquestração multi-IA, helpers de terminal e o fluxo Chain-of-Caralho.
- **grok-cli** (`grok-cli`): cliente direto para a API do Grok/xAI para prompts rápidos via terminal.

## Requisitos

- Rust 1.76+ (toolchain padrão do `rustup`).
- Git e GitHub CLI (`gh`) se quiser usar o comando automatizado de PR.
- Acesso a chaves de API se for usar Grok/xAI ou os fluxos Multi-IA (ver seção de variáveis de ambiente).

Instale dependências básicas e rode todos os binários com:

```bash
cargo run -p fenrir_cli -- --help
cargo run -p fenrir -- --help
cargo run -p grok-cli -- --help
```

## Configuração do Fenrir CLI

O `fenrir_cli` cria (ou carrega) a configuração em `~/.config/fenrir/fenrir_rules.yaml`.

Campos disponíveis (padrões entre parênteses):

- `version` (`"1.0"`)
- `content_policies`
  - `anti_pedophilia` (true): bloqueia operações envolvendo conteúdo ilegal.
  - `anti_sensitive_leaks` (true): evita vazamento de credenciais/dados sensíveis.
  - `allow_aggressive_pentest` (false): libera scans agressivos que podem causar DoS.
  - `respect_robots_txt` (true): segue robots.txt e rate limits.
  - `audit_logging` (true): habilita logs de auditoria.
  - `protect_critical_infra` (true): bloqueia ações em infra crítica conhecida.
- `scan`
  - `max_threads` (100): threads paralelas máximas.
  - `default_timeout` (5): timeout padrão por conexão (s).
  - `default_port_range` (`"1-1000"`): range padrão de portas.
  - `user_agent` (`"Fenrir/<versão> (Security Scanner)"`).
- `output`
  - `colors` (true): habilita cores.
  - `verbosity` (1): nível de verbosidade 0-3.
  - `format` (`"text"`): formatos aceitos `text|json|yaml`.

Gere o arquivo com valores padrão usando `fenrir init` (ver comandos abaixo). Use `--config <path>` para apontar para outro YAML sem sobrescrever o padrão.

## Comandos do Fenrir CLI

Use `fenrir <comando> --help` para opções detalhadas. Resumo rápido:

- `scan --target <alvo> [--port-range 1-1000] [--scan-type quick|full|stealth|aggressive] [--timeout 5] [--threads 100] [--verbose]`
  - Mostra parâmetros, respeita guardrails (`allow_aggressive_pentest`) e hoje retorna resultados simulados.
- `rules [--list] [--reload]`
  - Exibe as políticas de segurança atuais ou simula recarga do YAML.
- `init [--force]`
  - Cria `~/.config/fenrir/fenrir_rules.yaml` com comentários e padrões seguros; `--force` sobrescreve.
- `about`
  - Banner, lore e links do projeto.
- `gitar [--message "texto"] [--verbose]`
  - Workflow Git → commit → push → `gh pr create`; usa mensagem padrão se nenhuma for fornecida.
- `huh [--verbose]`
  - Front-end interativo estilo formulário que coleta alvo/portas/tipo de scan e chama o fluxo de `scan`.

Opções globais: `--verbose`, `--no-color`, `--config <caminho>`.

### Exemplo de sessão

```bash
# Inicializar config padrão
fenrir init

# Listar guardrails ativos
fenrir rules --list

# Rodar scan rápido simulado
fenrir scan --target exemplo.com --scan-type quick --verbose

# Abrir formulário interativo
fenrir huh
```

## Variáveis de ambiente e chaves de API

Fluxos que falam com modelos xAI/Grok e integrações multi-IA leem a primeira chave disponível na lista de prioridade:

```
API_KEY → KAT_KEY → QWEN_CODE → QWEN_CODE_KEY → GLM4_6_KEY → GLM_4_6_KEY → GLM_KEY → GLM_API_KEY → GROK_API_KEY → XAI_API_KEY → GLI_KEY
```

Defina pelo menos uma delas antes de usar os clientes Grok ou o modo multi-IA:

```bash
export KAT_KEY="sua_chave"
# ou
export GROK_API_KEY="sua_chave"
```

O `grok-cli` aceita `KAT_KEY`, `GROK_API_KEY`, `XAI_API_KEY` ou `GLI_KEY`. Se nenhuma estiver presente, ele falha explicando as variáveis esperadas.

## God Mode e Chain-of-Caralho (binário `fenrir`)

O binário `src/fenrir` é experimental e inicializa uma sequência barulhenta de módulos internos: manifesto de agentes, detecção de terminal, integração Starship/Ghostty, coordenador multi-IA, cliente Grok Code e Chain-of-Caralho (gestão de tarefas em cadeia). Ele também respeita as mesmas chaves de API descritas acima. Use apenas em ambientes de teste — a interface é deliberadamente verborrágica.

Execute com:

```bash
cargo run -p fenrir -- --help
```

## Cliente Grok (binário `grok-cli`)

Uso rápido:

```bash
cargo run -p grok-cli -- \
  --prompt "Explique rust async" \
  --model grok-4.1 \
  --temperature 0.7 \
  --max-tokens 512 \
  --timeout-secs 30
```

O cliente envia o prompt para `https://api.x.ai/v1/chat/completions`, exibe a resposta e mostra consumo de tokens quando disponível.

## Scripts auxiliares

- `setup_trinity.sh`: instala dependências e lembra de exportar `KAT_KEY` para habilitar integrações Grok/Trinity.
- `test_grok_api.sh`: valida rapidamente se a variável `KAT_KEY` está configurada e responde chamando a API.

## Desenvolvimento

- Formatação/testes: `cargo fmt` e `cargo test` em todo o workspace.
- O comando `fenrir gitar` pode automatizar commits/PRs se `git` e `gh` estiverem configurados.

## Licença

MIT. Veja o cabeçalho de cada crate para detalhes.
