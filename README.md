# Remote Host Tools (rht)

A tool to ease working on remote hosts over SSH.

* Open your local browser from a remote SSH host (e.g., for SSO login)
* View a remote file with local tools

## Installation

Install rht with cargo on your desktop and on a server where you want to use it.

```sh
cargo install rht
```

Prebuilt binaries are available for select platforms, attached to the GitHub
releases.

## Usage

Start rht on your desktop by running `rht serve. Now, you can connect to another
host and use the tool:

```sh
# SSH in to a server with an SSH tunnel
desktop$ ssh server -R 12345:127.0.0.1:12345

# Use the binary to open the url on your desktop; or
server$ rht open https://www.example.com

# Configure your BROWSER env var and use xdg-open
server$ BROWSER="rht open" xdg-open https://www.example.com

# Alternatively, connect directly to the server with the env
desktop$ ssh server -tR 12345:127.0.0.1:12345 "BROWSER='rht open'" bash
```
