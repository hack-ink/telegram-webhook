<div align="center">

# Telegram Webhook
### A webhook server can forward messages from a source to Telegram channel(s).

[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Checks](https://github.com/hack-ink/telegram-webhook/actions/workflows/checks.yml/badge.svg?branch=main)](https://github.com/hack-ink/telegram-webhook/actions/workflows/checks.yml)
[![Release](https://github.com/hack-ink/telegram-webhook/actions/workflows/release.yml/badge.svg)](https://github.com/hack-ink/telegram-webhook/actions/workflows/release.yml)
[![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/hack-ink/telegram-webhook)](https://github.com/hack-ink/telegram-webhook/tags)
[![GitHub code lines](https://tokei.rs/b1/github/hack-ink/telegram-webhook)](https://github.com/hack-ink/telegram-webhook)
[![GitHub last commit](https://img.shields.io/github/last-commit/hack-ink/telegram-webhook?color=red&style=plastic)](https://github.com/hack-ink/telegram-webhook)

</div>


### Usage
```
A server can forward messages from a webhook to a Telegram channel.

Usage: telegram-webhook [OPTIONS] --chat-id <ID>

Options:
  -p, --port <PORT>   Port to listen on [default: 8080]
  -c, --chat-ids <ID>  Telegram chat ID
  -h, --help          Print help
  -V, --version       Print version
```

#### Set the bot token use:
```
export BOT_TOKEN=the-token-that-you-got-from-the-telegram-bot-father
```
