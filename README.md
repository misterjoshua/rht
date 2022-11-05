# Remote Host Tools

Remote Host Tools (rht)

## Usage

Download the binary for your desktop and start it - you will now be running an
HTTP server on localhost port 12345.

Now, connect to another host and open a browser remotely:

```sh
# SSH in to a server with an SSH tunnel
desktop$ ssh server -R 12345:127.0.0.1:12345

# Use the binary to open the url on your desktop; or
server$ rhc open https://www.example.com

# Configure your BROWSER env var and use xdg-open
server$ BROWSER=rhc xdg-open https://www.example.com

# Alternatively, connect directly to the server with this
desktop$ ssh server -R 12345:127.0.0.1:12345 -t BROWSER=rhc bash
```
