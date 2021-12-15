# Quake

[![Build](https://github.com/phodal/quake/actions/workflows/build.yml/badge.svg)](https://github.com/phodal/quake/actions/workflows/build.yml)
[![Coverage Status](https://coveralls.io/repos/github/phodal/quake/badge.svg?branch=master)](https://coveralls.io/github/phodal/quake?branch=master)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/phodal/quake)
[![Lint](https://github.com/phodal/quake/actions/workflows/lint.yml/badge.svg)](https://github.com/phodal/quake/actions/workflows/lint.yml)
[![web](https://github.com/phodal/quake/actions/workflows/web.yml/badge.svg)](https://github.com/phodal/quake/actions/workflows/web.yml)

> A meta knowledge management tool for geek.

Quake is a knowledge management meta-framework for geeks. It can:

- freedom text content management. todo lists, article management, book reviews, notes, etc.
- construct a knowledge network system. customized markdown link
- fleeting inspiration.support quick start (CLI, TUI, Web, GUI) and global search
- freely present the canvas. DSL and free Sketchpad

Examples：

1. [examples](./examples)
2. ...

Architecture:

![Architecture](docs/quake-arch.svg)

Screenshots:

![Web Samples](http://quake-demo.inherd.org/web.gif)

## Setup

1. download from release or `cargo install quake`
2. download web from release
3. init with `quake init`
4. use `quake cmd -i "todo.add: hello, world"` to add new todo
5. use `quake server` to start server (ps: need to install `meilisearch` for search entries).

more in [Setup](examples/quake_book/0002-setup.md);

## Features

### Entry (meta-data for content)

Every content type in quake is call a `entry`.

run:

```bash
quake cmd -i "todo.add: hello, world"
```

will become:

```bash
.
├── entries-define.yaml       # entry defines
└── todo
    ├── 0001-hello-world.md   # entry content
    ├── entries.csv           # entry collections for display
    └── entry-node-info.yaml  # entry node infos
```

### Markdown with Front matter

a entry file will a front matter for meta-info, others will be `content`

```markdown
---
title: hello, world
author:
created_date: 2021-12-10 20:24:25
updated_date: 2021-12-10 20:24:25
---

> a hello, world
```

### Custom frontend

Quake will load `web` directory code, and start a web server.Everyone can build then pages with Quake.

Simple Layout Engine:

```
----------------------------------------------------------------
|      Calendar(flow("show_calendar"), 12x)                   |
----------------------------------------------------------------
| Empty(2x) | Timeline(flow("show_timeline"), 8x) | Empty(2x) |
----------------------------------------------------------------
```

### Transflow

A DSL to connect entry data and component, then auto render it.

```javascript
from('todo','blog').to(<quake-calendar>);
```

will fetch `todo` and `blog`, render to `quake-calendar` web component, and create route in page

also can filter data

```javascript
from('todo','blog').to(<quake-calendar>).filter('created_date > 2021.01.01 AND created_date < 2021.12.31')
```

### Free sketchpad

by Web Component

To be design.

### Powerful markdown

Flowy to chart:

```
from('todo','blog').to(<quake-network>);
```

Graph and chart in markdown

````@graph('bar')
```@graph('bar')
|-----------|--------------|
| 1         |   12.0       |
| 2         |   2.0        |
| 3         |   4.0        |
```
````

## Docs

DEVELOPMENT: see in [DEVELOPMENT.md](./DEVELOPMENT.md)

欢迎入群讨论：

![Wechat Code](http://quake-demo.inherd.org/qrcode.jpg)

License
---

markdown parser based on: [https://github.com/zoni/obsidian-export](https://github.com/zoni/obsidian-export) with Apache and MIT

@ 2021 This code is distributed under the MIT license. See `LICENSE` in this directory.
