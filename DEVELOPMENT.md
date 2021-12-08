# Development

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

4. visit: [http://localhost:8000/](http://localhost:8000/) （需要构建前端部份）


### 前端部分

代码位于：`quake_webapp`

#### 搭建

项目使用 pnpm，需要先安装 `npm install -g pnpm`

1. install，在 `quake_webapp` 目录下执行 `pnpm recursive install`
2. build，在 `quake_webapp` 目录下执行 `yarn dist`
	- 构建每个前端应用，生成对应的 WebComponents 组件
	- 复制组件到 `dist` 目录