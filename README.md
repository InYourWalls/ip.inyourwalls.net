# iptool_v2
The new version of [ip.inyourwalls.net](https://ip.inyourwalls.net/), now with more Rust.

## Features
* Templating with [Handlebars](https://handlebarsjs.com/).
* That's it.

This program is written primarily for my own purposes, but you're welcome to compile and use it if you wish.
It's built with [Rocket](https://github.com/SergioBenitez/Rocket), and uses its template library. Templates
are loaded from `/templates` in your server folder.

The server is meant to run behind a reverse proxy (I use Nginx for this). It must pass the `X-Real-IP` header.

The included template is the one used on my website. Edit this file to create your own.