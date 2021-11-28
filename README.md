# Quake

> A meta knowledge management tool for geek.

Quake 是面向极客的知识管理元框架，它可以：

- 自由的文本内容管理。Todo 清单、文章管理、书评、笔记等。
- 构建知识网络体系。定制化 markdown 链接
- 抓住稍纵即逝的灵感。支持快速启动（CLI、TUI）与全局搜索
- 自由的呈现画布。DSL 与自由画板

## Usage

### GUI (TBD)



### Terminal

1. add entry by type

```
quake command -i "todo.add: time support"
```

2. edit entry by index

```
quake command -i "todo.update(1)"
```

3. list entry by type

```
quake command -i "todo.list"
```

4. sync list

```
quake command -i "todo.sync"
```

5. search [tbd]

```
quake command -i "phodal_com.dump"
```

then

```
curl -i -X POST 'http://127.0.0.1:7700/indexes/phodal_com/documents' \
  --header 'content-type: application/json' \
  --data-binary @dump.json
```

7. concept [tbd]

8. network [tbd]

### Query Syntax

more in: [quake.pest](quake_core/src/parser/quake.pest)

```pest
action_decl = {
    object ~ "." ~ action ~ parameters? ~ ":"? ~ " "* ~ text?
}

parameters = {
    "(" ~ parameter ~ ("," ~ parameter)* ~ ")"
}
```

### Markdown Extends syntax [TBD]

1. tag: `#{tag}`
2. custom function: `#{$block}, #{$toc}, #{$link}, #{$file = Hello.pptx}`

## Setup

1. install `quake` from rust
2. install [meilisearch](https://github.com/meilisearch/MeiliSearch)

## Design principle

我使用了不同的工具来管理知识，Microsoft To Do 管理 idea、Phodit + [Phodal.com](https://www.phodal.com/) 发布文章、Apple Notes 记录笔记等等，知识被分散在各个工具中。不利于我进行洞见，寻找灵感，与此同时，还缺乏书写和记录的方式。

我需要一个新的工具来融合到我的知识体系里，它应该是：

- 开源的。可以自由扩展。
- 分布式 + 本地化的。可以离线使用，方便于出差旅途中使用。
- 版本化的。可以自由查看变更历史。
- 开放的。可以自由与其它工具组合。如 Vim、VSCode 等。
- 易于扩展。可以结合习惯用的工具。诸如于，基于 DSL 的编辑-发布分离的类 Web 模式，用于展示。如 MxGraph、Mermaid、Ledge Framework 等

## Developoment

Languages: Rust, TypeScript

Tech stacks:

- Search Engine: [MeiliSearch](https://github.com/meilisearch/MeiliSearch)
- Frontend:
  - MV* Frameworks: Stencil.js, TBD
  - Charts: D3.js, MxGraph, Echarts
  - Editor: Quill?
- Database: Git + CSV + Yaml
- Web API: Rocket.rs
- Analysis API: Jieba.rs
- TUI: tbd
- GUI: tbd

## Roadmap

### MVP

- [x] 导入
  - [x] Django/Mezzanine
  - [x] Apple Notes
  - [x] Microsoft Todo
  - [ ] Microsoft Onenote
- [x] 自定义条目类型
  - [x] CRUD
  - [x] dynamic update entries-define
- [x] CLI 交互与列表呈现
- [ ] MVP API 设计
  - [x] Web Server
  - [ ] GraphQL
- [x] 基于 Git 的数据存储
- [ ] Search anywhere
  - [x] search frameworks
  - [ ] auto suggest

### 1.0: Knowledge map 

- [ ] 嵌入式脚本语言
- [ ] 知识空间
  - [ ] 自定义项目视图
  - [ ] 年份视图
- [ ] 知识可视化
  - [ ] Mindmap from Ledge Framework?
  - [ ] Mindmap connect to Entry (`${connect.todo("0001", "${title}"}`)
- [ ] Dashboard
- [ ] Editor (TBD)
  - markdown editor


License
---

@ 2021 This code is distributed under the MIT license. See `LICENSE` in this directory.
