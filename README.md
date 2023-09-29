<div align="center">

# Telegram Webhook
### A webhook server can forward messages from a source to Telegram chat(s).

[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Checks](https://github.com/hack-ink/telegram-webhook/actions/workflows/checks.yml/badge.svg?branch=main)](https://github.com/hack-ink/telegram-webhook/actions/workflows/checks.yml)
[![Release](https://github.com/hack-ink/telegram-webhook/actions/workflows/release.yml/badge.svg)](https://github.com/hack-ink/telegram-webhook/actions/workflows/release.yml)
[![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/hack-ink/telegram-webhook)](https://github.com/hack-ink/telegram-webhook/tags)
[![GitHub code lines](https://tokei.rs/b1/github/hack-ink/telegram-webhook)](https://github.com/hack-ink/telegram-webhook)
[![GitHub last commit](https://img.shields.io/github/last-commit/hack-ink/telegram-webhook?color=red&style=plastic)](https://github.com/hack-ink/telegram-webhook)

</div>


### Installation options
1. Download from https://github.com/hack-ink/telegram-webhook/releases.
2. Install from crates.io `cargo install telegram-webhook`.
3. Clone the repository and execute `cargo build --release`.

### Usage
```
### A webhook server can forward messages from a source to Telegram chat(s).

Usage: telegram-webhook [OPTIONS] --chat-id <ID>

Options:
  -p, --port <PORT>   Port to listen on [default: 8080]
  -c, --chat-ids <ID>  Telegram chat ID
  -h, --help          Print help
  -V, --version       Print version
```

#### Set the bot token
```
export BOT_TOKEN=the-token-that-you-got-from-the-telegram-bot-father
```

#### How to obtain the chat ID
1. Search for the bot @raw_data_bot on Telegram and start a conversation with it by sending `/start`.
2. Tap on the `Chat` button and choose the specific chat you wish to retrieve the ID for.
