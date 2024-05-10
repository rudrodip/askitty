# askitty

## Description

A simple CLI tool to interact with generative models like llms and image generation apis.

LLM api - [groq](https://groq.com/)
Image generation api - [stable diffusion](https://replicate.com/stability-ai/stable-diffusion)

Usage:

```bash
askitty [FLAG] [MESSAGE]
```

Flags:

```bash
-h, --help                          Display help message
-v, --version                       Display version
-m, --message                       Message to send to the model
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
-i, --imagine                       Generate image from text
```

Todo

- [x] groq api
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
- [ ] set configuration command
- [ ] view configuration command
- [ ] pretty print
- [x] kv store
- [ ] config store
- [ ] image path
- [ ] context strategy [sliding window, fixed window, summarize context, etc]
- [ ] function calling from scratch
- [ ] utils - writing LICENSE, SEO image resizing, etc
- [ ] tests
- [ ] documentation
