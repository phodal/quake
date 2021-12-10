# Quake

[![Build](https://github.com/phodal/quake/actions/workflows/build.yml/badge.svg)](https://github.com/phodal/quake/actions/workflows/build.yml)
[![Coverage Status](https://coveralls.io/repos/github/phodal/quake/badge.svg?branch=master)](https://coveralls.io/github/phodal/quake?branch=master)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/phodal/quake)

> A meta knowledge management tool for geek.

Quake is a knowledge management meta-framework for geeks. It can:

- freedom text content management.todo lists, article management, book reviews, notes, etc.
- construct a knowledge network system. customized markdown link
- grab the fleeting inspiration.support quick start (CLI, TUI) and global search
- freely present the canvas. DSL and Free Sketchpad

Examples：

1. [examples](./examples)
2. ...

Setup:

1. download from release or `cargo install quake`
2. download web from release
3. init with `quake init`
4. use `quake cmd -i "todo.add: hello, world"` to add new todo
5. use `quake server` to start server

more see in [Setup](examples/quake_book/0002-setup.md);

## Docs

DEVELOPMENT: see in [DEVELOPMENT.md](examples/quake_book/004-development.md)

Architecture:

![Architecture](docs/quake-arch.svg)

Screenshots:

![Web Samples](http://quake-demo.inherd.org/web.gif)

欢迎入群讨论：

![Wechat Code](http://quake-demo.inherd.org/qrcode.jpg)

License
---

markdown parser based on: [https://github.com/zoni/obsidian-export](https://github.com/zoni/obsidian-export) with Apache and MIT

@ 2021 This code is distributed under the MIT license. See `LICENSE` in this directory.
