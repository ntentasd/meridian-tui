# meridian-tui

A terminal UI for [Meridian](https://github.com/ntentasd/meridian).

Meridian already gives you a web page with every route in your cluster. This
gives you the same thing in the terminal. It talks straight to Meridian's API
and keeps the list up to date as things change.

Browse your routes, filter them and open one in the browser without leaving
the terminal.

> Early days: right now it can only dump the route list. The actual TUI is coming.

## Installation

```bash
cargo install --git https://github.com/ntentasd/meridian-tui
```

## Usage

```bash
meridian-tui --url https://meridian.example.com --dump
```
