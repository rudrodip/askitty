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
-h, --help       Display help message
-v, --version    Display version
-m, --message    Message to send to the model
-r, --repl       Start a repl
-i, --imagine    Generate image from text
```

Todo

- [x] groq api
- [x] optional flags
- [x] markdown preview
- [x] image generation api
- [x] error handling
- [ ] streaming output
- [x] chat history
- [ ] new chat session command
- [ ] view chat history command
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
