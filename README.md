# Kuaa Tools

[Kuaa Tools](https://kuaa.tools/)

Kuaa CLI is a command-line interface tool designed to enhance Git workflow productivity by providing functionalities such as generating commit messages based on staged changes, and managing API keys for interacting with the Kuaa API. It also includes a feature to fetch the current balance of K-Tokens from the Kuaa API.

## Features

- **Generate Git Commit Messages:** Automatically generates commit messages by analyzing staged Git changes.
- **Configure API Key:** Easily set and store your Kuaa API key for subsequent requests.
- **Fetch K-Tokens Balance:** Check your current balance of K-Tokens using the Kuaa API.

## Prerequisites

- Rust and Cargo (for building from source)
- Git (for Git-related operations)
- Homebrew (for installation)

## Installation

Kuaa CLI can be installed using Homebrew:

```sh
brew tap tavernari/kuaa
brew install kuaa
```

Replace `tavernari/kuaa` with the actual tap location where the formula resides.

## Usage

### Setting the API Key

Get your API Key from [kuaa.tools](https://kuaa.tools/dashboard/panel/api-keys/)
You can set an env vars `KUAA_API_KEY` or you can use the command line to set it locally.
To set the API key for Kuaa CLI, use the following command:

```sh
kuaa config api-key <YOUR_API_KEY>
```

### Generating a Git Commit Message

To generate a Git commit message based on staged changes:

```sh
kuaa gen git-commit-message
```

Optionally, add a custom message with the generated content:

```sh
kuaa gen git-commit-message --git-commit-message "Your additional comments here"
```

### Fetching K-Tokens Balance

To fetch the current balance of your K-Tokens:

```sh
kuaa balance
```

## Development

To contribute or build from source, clone the repository and build using Cargo:

```sh
git clone https://github.com/tavernari/kuaa.git
cd kuaa-cli
cargo build --release
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
