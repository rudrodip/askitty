# askitty

## Description

A simple CLI tool to interact with generative models like llms and image generation apis.

LLM api - any LLM that follows opneai/v1 spec

Image generation api - any image gen api that follows opneai/v1 spec

Usage:

```bash
askitty [FLAG] [MESSAGE]
```

Flags:

```bash
-h, --help                          Display help message
-v, --version                       Display version
-m, --message                       Message to send to the model
-i, --imagine                       Generate image from text
-r, --repl                          Start a repl
-n, --new                           Start a new session
-s, --sessions                      View all sessions
-s <session_id>                     View a specific session
-d <session_id>                     Delete a specific session
-c, --clear                         Clear all sessions
-p, --prompt                        Set global system prompt
-p <session_id>                     Set a specific session prompt
-ps, --prompt-show                  Show global system prompt
-pc, --prompt-clear                 Clear global system prompt
-pd, --prompt-delete <session_id>   Delete a specific session prompt              
-vc, --view-config                  View global configuration
-sc, --set-config                   Set global configuration
```

Todo

- [x] openai api
- [x] optional flags
- [x] markdown preview
- [x] image generation api
- [x] error handling
- [ ] streaming output
- [x] chat history
- [x] new chat session command
- [x] view chat history command
- [x] global system prompt
- [x] individual system prompt
- [x] set configuration command
- [x] view configuration command
- [x] kv store
- [x] image path
- [ ] tests
- [x] documentation
