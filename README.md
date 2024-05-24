# askitty

![askitty](https://github.com/rudrodip/askitty/assets/77154365/e7a832f5-a45b-4999-b762-77a19992614e)

## Description

`askitty` is a simple CLI tool designed to interact with generative models like large language models (LLMs) and image generation APIs. The tool supports various features, including chat history, new chat session commands, global and individual system prompts, and more.

## Features

- **LLM API**: Supports any LLM that follows the OpenAI/v1 spec.
- **Image Generation API**: Supports any image generation API that follows the OpenAI/v1 spec.
- **Chat History**: View and manage chat history.
- **Sessions**: Start new sessions, view existing sessions, and delete sessions.
- **System Prompts**: Set and manage global and session-specific prompts.
- **Configuration**: Set and view global configuration.
- **Error Handling**: Robust error handling for a seamless experience.
- **Markdown Preview**: Preview messages in Markdown format.

## Installation

To get started with `askitty`, you can use the following command to download and install the latest release:

```bash
curl -sSL https://askitty.rdsx.dev/install.sh | bash
```

## Usage

Once installed, you can use `askitty` from the command line with the following syntax:

```bash
askitty [FLAG] [MESSAGE]
```

### Flags

- `-h, --help`                          Display help message
- `-v, --version`                       Display version
- `-m, --message`                       Message to send to the model
- `-i, --imagine`                       Generate image from text
- `-r, --repl`                          Start a REPL
- `-n, --new`                           Start a new session
- `-s, --sessions`                      View all sessions
- `-s <session_id>`                     View a specific session
- `-d <session_id>`                     Delete a specific session
- `-c, --clear`                         Clear all sessions
- `-p, --prompt`                        Set global system prompt
- `-p <session_id>`                     Set a specific session prompt
- `-ps, --prompt-show`                  Show global system prompt
- `-pc, --prompt-clear`                 Clear global system prompt
- `-pd, --prompt-delete <session_id>`   Delete a specific session prompt
- `-vc, --view-config`                  View global configuration
- `-sc, --set-config`                   Set global configuration

## Examples

### Sending a Message to the Model

```bash
askitty -m "Hello, how are you?"
```

### Generating an Image from Text

```bash
askitty -i "A beautiful sunset over the mountains"
```

### Starting a New Session

```bash
askitty -n
```

### Viewing All Sessions

```bash
askitty -s
```

### Viewing a Specific Session

```bash
askitty -s <session_id>
```

### Setting a Global System Prompt

```bash
askitty -p "You are a helpful assistant."
```

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request with your changes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
