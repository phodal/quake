---
title: Setup
created_date: 2021-12-10 19:21:23
updated_date: 2021-12-10 19:21:23
order: 
author: 
---


1. init, `quake init`
2. config
 - `.quake.yml` to config quake
 -  edit `entries-define.yaml` to

## Terminal GUI

```
quake tui
```

### CLI

0. install

```bash
git clone https://github.com/phodal/quake && cd quake
cargo install --path .  # or: just install
```

1. add entry by type

```bash
quake cmd -i "todo.add: time support"
```

2. edit entry by index

```bash
quake cmd -i "todo.edit(1)"
```

3. list entry by type

```bash
quake cmd -i "todo.list"
```

4. sync list

```bash
quake cmd -i "todo.sync"
```

5. show entry

```bash
quake cmd -i "todo.show(1)"
```

6. generate from pdf

filter with `Grep` regex syntax

```
quake generate --flow "from('examples').to('papers').filter('.pdf$')"
```

### Web

1. setup search engine

```bash
brew install meilisearch
```

index

```
quake cmd -i "quake.feed"
```

2. run server

```
quake server
```

3. visit: [http://localhost:9999/](http://localhost:9999/)

### GUI (TBD)

TBD
