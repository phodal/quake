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

- action，每一个操作称为 action，比如 add/show/edit
- entry, 对应于不同类型内容的操作
- quake，系统相关的操作，如同步等
- transflow，提供自定义的数据到组件的流

对应的目录说明

- src/server/*_api.rs，提供对应的 HTTP API
- src/cli/*_action.rs，提供 CLI 的操作
- quake_tui，TUI 对应的代码
- quake_webapp，Web 对应的代码

## 整体结构
项目后端语言是 Rust，使用 Rocket 框架，`.quake.yaml` 是配置文件，在 `mod.rs` 文件中会读取相应信息并将 `quake_webapp` 目录下的 `index.html` 以 FileServer 的形式作为项目 Web 的入口。 

该 html 入口将直接引用前端构建好的 js，具体细节可以看 `quake_webapp` 目录内的打包代码和 html 文件。

从整体看，项目前端(GUI/TUI/Web)响应用户操作，调取后端接口，后端调取 MeiliSearch 对本地文件目录进行增删改查，从而实现以 git 为核心的 storage。

需要注意的是，后端 server 其实是很薄的一层，可以看做前端的 adapter，Rust 原生的核心在 `quake_core` 中。

由于需要自定义类型，目前 MeiliSearch 还没完全接入到 rocket.rs 里，，所以 web 现在会直接调用 MeiliSearch。

## Setup

提前准备：[安装 Rust 环境](https://www.rust-lang.org/learn/get-started)

1. clone 代码 `git clone https://github.com/phodal/quake`
1. 安装 search engine 并运行
    ```bash
    brew install meilisearch
    ```
    ```bash
    meilisearch
    ```
1. 插入测试数据
    ```bash
    cargo run -- cmd -i "quake.feed"
    ```
1. 运行 Web API 服务
    ```bash
    cargo run --  server -w
    ```
1. 安装前端依赖
    ```bash
    # 项目使用 pnpm，需要先安装 `npm install -g pnpm`
    cd quake_webapp && pnpm recursive install
    ```
1. 构建前端代码
    ```bash
    yarn dist
    ```
1. visit: [http://localhost:9999/](http://localhost:9999/)
    
# Quake Web 开发指南

## build

前端项目基于 `pnpm` 来管理并减少依赖体积，使用 [nx](https://nx.dev/#getting-started) 来管理整个项目，nx cloud可以有效减少每一次构建/测试的时长。

## 开发

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

## Others

see in [Quake book](examples/quake_book/)
