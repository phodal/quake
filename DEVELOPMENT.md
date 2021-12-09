# Development

![Architecture](docs/quake-arch.svg)

Languages: Rust, TypeScript (Node.js 16)

Tech stacks:

- Search Engine: [MeiliSearch](https://github.com/meilisearch/MeiliSearch)
- Web API: Rocket.rs
- Frontend:
    - MV* Frameworks: Stencil.js, React, Angular, Vue..
    - Charts: D3.js, MxGraph, Echarts
- Database: Git + CSV + Yaml
- Analysis API: Jieba.rs
- TUI: tbd
- GUI: tbd

系统主要由以下四个部分的应用构成交互：

- Terminal UI
- CLI
- GUI(tbd)
- Web

对应的操作有：

- action，每一个操作称为 action
- entry, 对应于不同类型内容的操作
- quake，系统相关的操作，如同步等
- transflow，提供自定义的数据到组件的流

对应的目录说明

- src/server/*_api.rs，提供对应的 HTTP API
- src/cli/*_action.rs，提供 CLI 的操作
- quake_tui，TUI 对应的代码。


## Setup

提前准备：安装 Rust 环境


1. clone 代码 `git clone https://github.com/phodal/quake`
2. 安装 serach engine

```bash
brew install meilisearch
```

插入测试数据

```
cargo run -- cmd -i "quake.feed"
```

3. 运行 Web API 服务

```
cargo run --  server -w
```

4. visit: [http://localhost:9999/](http://localhost:9999/) （需要构建前端部份）


### 前端部分

代码位于：`quake_webapp`

#### 搭建

项目使用 pnpm，需要先安装 `npm install -g pnpm`

1. install，在 `quake_webapp` 目录下执行 `pnpm recursive install`
2. build，在 `quake_webapp` 目录下执行 `yarn dist`
    - 构建每个前端应用，生成对应的 WebComponents 组件
    - 复制组件到 `dist` 目录
    
# Quake Web 开发指南

Quake 的组件都由 Web Component 构成，通过 Web Component Router 将它们组合在一起，即 [app.js](quake_webapp/app.js)。 如下一个简单的 Web Component 创建过程，基于纯原生的 JavaScript：

```javascript
const tl_timeline =  async (context, commands) => {
  const el = document.createElement('quake-calendar');

  let todos = await Quake.query('todo');
  let blogs = await Quake.query('blog');
  let data = from_todo_blog_to_quake_calendar(todos, blogs);

  el.setAttribute('data', JSON.stringify(data));

  return el;
}
```

通过 `Quake.query` 可以直接访问搜索引擎，获得对应的数据，并将数据置换为 `<quake-calendar data="[]"></quake-calendar>` 需要的形式。

随后，通过路由将组件组合在一起：

```javascript
router.setRoutes([
  {path: '/', action: home},
  {path: '/transflow/timeline', action: tl_timeline},
  {path: '/entry/:type/new', action: create_entry},
  {path: '/edit/:type/:id', action: edit_entry},
]);
```

因此，剩下的重点就是如何去创建一个个的 Web Component 组件：

- quake-dashboard. 首页默认的是 Quake Dashboard 应用，一个基于 Stencil.js 的 App。
- quake-editor.    Quake 的编辑器（待完善）
- quake-render.    Quake 的渲染器（待开发）
- typeform.        渲染 entry 的表单（待开发）
- packages         其它  Quake 组件
    - calendar     日历
    - 其它

## 创建新的 Quake 组件

### React 篇

可以参考示例应用：[Quake Calendar](quake_webapp/packages/calendar)

使用 `create-react-app` 创建，与常规的应用开发大致相同，稍有不同的是：

- public/index.html   添加我们的组件，用来测试，如：`<quake-calendar-timeline data="{}"></quake-calendar-timeline>`
- config-overrides.js 添加修改暴露的组件名称
- src/index.ts        用于自动化绑定生成的 Web Component 组件


还需要注意对于 CSS 的使用。

### Angular 篇

参考官方的构建指南即可「TBD」

### Vue 篇

参考官方的构建指南即可「TBD」


### Stencil.js 篇

使用标准的 Stencil.js 开发即可。


## Transflow 转换数据流

Quake 采用了简单的 Transflow DSL 设计：

```groovy
transflow { 
    from('todo','blog').to(<quake-calendar>); 
}
```

即将 `todo` 和 `blog` 渲染到 `<quake-calendar>` 组件中：

在组件和数据对应的情况下，上述的代码，可以生成如下的两部分代码：

```javascript
// 数据转换
function from_todo_blog_to_quake_calendar(todos, blogs) {
  let results = [];
  results = results.concat(todos);
  results = results.concat(blogs);
  return results;
}

// 渲染组件
const tl_show_timeline = async (context, commands) => {
  const el = document.createElement('quake-calendar');

  let todos = await Quake.query('todo');

  let blogs = await Quake.query('blog');

  let data = from_todo_blog_to_quake_calendar(todos, blogs);
  el.setAttribute('data', JSON.stringify(data));

  return el;
}
```


可以通过 `transflows.yaml` 来自定义不同的数据流及转换：


```yaml
- name: "show_timeline"
  target: "quake-calendar-timeline"
  defines_map: ~
  flows:
    - name: "from_todo_blog_to_quake_calendar_timeline"
      from: [ "todo", "blog" ]
      to: "<quake-calendar-timeline>"
      mappings:
        - entry: "todo"
          source: ["id", "title", "content", "created_date", "updated_date"]
          target: ["id", "title", "content", "created_date", "updated_date"]
        - entry: "blog"
          source: ["id", "title", "content", "created_date", "updated_date"]
          target: ["id", "title", "content", "created_date", "updated_date"]
      filter: # 尚在开发中
        - entry: "todo"
          expression: "created_date > 2020.12.30 AND updated_date < 2022.01.01"
```


还有尚在开发中的数据过滤。


## Query Syntax

more in: [quake.pest](quake_core/src/parser/quake.pest)

```pest
action_decl = {
    object ~ "." ~ action ~ parameters? ~ ":"? ~ " "* ~ text?
}

parameters = {
    "(" ~ parameter ~ ("," ~ parameter)* ~ ")"
}
```

## Markdown Extends syntax [TBD]

1. tag: `#{tag}`
2. custom function: `#{$block}, #{$toc}, #{$link}, #{$file = Hello.pptx}`

## Quake Output

examples output:

```
├── entries-define.yaml         # define all type data
├── web
│   ├── index.html
│   └── js
├── yarn.lock
└── yiki                        # data type
    ├── 0001-hello-world.md     # a `yiki` file
    ├── entries.csv             # tables for all `yiki`
    └── entry-node-info.yaml    # `yiki` node info
```

# Issues

IndexMap not working: [https://github.com/intellij-rust/intellij-rust/issues/8007](https://github.com/intellij-rust/intellij-rust/issues/8007)
