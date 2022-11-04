# Web Server that Opens Local Browser

Provides a web server that accepts requests to open a web browser. I'm using
this tool to tunnel `xdg-open` requests through an SSH tunnel to my desktop.

## Usage

Start the web server on a host with a web browser:

```sh
# Start nc-url-opener
desktop$ nc-url-opener
[2022-11-04T06:48:10Z INFO  nc_url_opener] Listening on http://127.0.0.1:12345
[2022-11-04T06:48:10Z INFO  actix_server::builder] Starting 4 workers
[2022-11-04T06:48:10Z INFO  actix_server::server] Actix runtime found; starting in Actix runtime
```

Connect to another host and open a browser remotely:

```sh
# SSH in to a server with an SSH tunnel
desktop$ ssh server -R 12345:localhost:12345

# From the server, use curl to open a browser on your desktop.
server$ curl http://localhost:12345/open -X POST --data https://www.google.com
```