---
title: "Transflow"
created_date: 2021-12-10 19:21:10
updated_date: 2021-12-26 23:35:39
order: 
author: 
---

## Two-phase：二阶段式转换

Transflow 的设计起源于：[类型流（TypeFlow）——世俗化的函数式编程和改进的过程式设计](https://zhuanlan.zhihu.com/p/341089716)》，在 Typeflow 有几个主要的规则：

1. 从可视化模型上就可以看出：共存在 4 个待实现的函数，其中两个纯函数，参数校验和返回结果包装；一个副作用函数，保存代办事项；还有一个输入端口，即把这个几个函数编排起来完成业务的程序入口。
2. 每个函数有明确的输入输出类型
3. 函数之间通过匹配的输入输出类型连接起来。
4. 输入输出类型使用业务人员能够理解的业务概念，从而符合DDD的要求。
5. 可视化

在有了设计之后，我们可以为功能生成对应的有输入和输出的函数，并可以通过规则将它们匹配起来。原理，就是这么简单。

## Transflow DSL： any data to any component

在 Quake 中拥有：

- 任意的数据源，即不同的 entry
- 任意的展示组件：即 Quake component。

随后，只提供一个从数据流向组件的规则，就可以实现 any to any，因此，Quake 中设计了 Transflow DSL。

### Transflow 示例

举个例子，当我们想有一个 calendar 来展示所有的 todo 和 blog 时，我们就需要从数据源中取得 todo 和 blog，对数据进行转换然后传输给 calendar 组件。用一句话来表达便是：

```javascript
from('todo','blog').to(<quake-calendar>);
```

这个 Transflow 的 DSL，最简模式下（即没有函数名、不添加数据映射（mapping）和过滤器（filter）、组件定义的情况下），它可以生成以下的 JavaScript  代码：

```javascript
function from_todo_blog_to_quake_calendar(todos, blogs) {
  let results = [];
  results = results.concat(todos);
  results = results.concat(blogs);
  return results;
}

const tl_temp_1 = async (context, commands) => {
  const el = document.createElement('quake-calendar');

  let todos = await Quake.query('todo');

  let blogs = await Quake.query('blog');

  let data = from_todo_blog_to_quake_calendar(todos, blogs);
  el.setAttribute('data', JSON.stringify(data));

  return el;
}

 Quake.router.addRoutes({path: '/transflow/show_temp_1', action: tl_temp_1 },)
```

代码逻辑上是：

1. 创建一个新的 Quake Calendar 组件（Web Component）
2. 获取 todo、blog 相关的数据
3. 执行对应的数据转换规则
4. 将数据传递给组件
5. 添加可访问的路由

这样一来，只需要跳转到相应的路由即可。如果需要的话，也可以直接生成临时的组件。另外一部分，则是由 Web Components 所构建的组件体系。

## 目标函数式的 Transflow

既然，我们是对数据流进行操作，那么理想情况下，Transflow 的 DSL 就可以设计为向函数式靠齐。不过，当前，我们还没有理由实现这么复杂的功能，可以在后续展开。

## From..to

`from` 用于定义数据源，当前支持的方式：

* Quake 自定义的 Entry 类型
* RESTful  API （待完善）

### 实现二次聚合 - 多条 Transflow

如果有多条 Transflow 规则时：

```javascript
transflow show_calendar {
  from('todo','blog').to('record'),
  from('record').to(<quake-calendar>);
}
```

就会生成多个 Transflow 函数（部分）：

```javascript
function from_todo_blog_to_record(todos, blogs) {
  let results = [];
  results = results.concat(todos);
  results = results.concat(blogs);
  return results;
}

function from_record_to_quake_calendar(records) {
  let results = [];
  results = results.concat(records);
  return results;
}
```

Transflow 生成的代码，面临的最大问题是数据量大时的性能问题，但是 Quake 的场景下，不会有这样的问题。

### Filter

在 filter 方面，我做了一些简化设计（\~~~偷懒~~\~），因为需要的是搜索引擎，可以可以直接使用搜索引擎的 fliter 功能。在评估了多个 filter-parser 的库之后，我发现没有理由在当前做这么复杂的设计。所以，针对于一些特别的过滤条件做了一些特别的处理。

如下是一个过滤时间的表达式：

```javascript
from('todo','blog').to(<quake-calendar>).filter('created_date > 2021.01.01 AND created_date < 2021.12.31')
```

由于搜索引擎并不支持各种各样的时间处理，所以我们可以替换对应的字符器。当前有：

* `quake_time.rs` 的时间转换逻辑。

随后转换为：

```javascript
created_date > 1609459200 AND created_date < 1640908800
```

等时机成熟，再完成整体的 filter 规则设计。

### Map

基于 Unix 的 pipe 思想与语法，结合 Transflow 的 map 标准库（transflow.lib.js），进行数值上的转换。

```javascript
from('todo','blog').to(<quake-calendar>).map('blog.content => content | uppercase | substring(1, 150)')
```

语法的符号参考自 Unix Shell 的设计思想，即使用 `|` 作为表示 pipe 的转换符。语法如下：

```pest
map_expr = {
    source ~ "=>" ~ target ~ ("|" ~ pipe_func)*
}

source = { ident ~ ("." ~ ident)*}
pipe_func = { ident ~ parameters? }
target = { ident }
```

生成的数据转换代码示例：

```javascript
results.push({
  type: "blog",
  title: blog.title,
  content: blog.content.uppercase().substring(1,150),
  created_date: blog.created_date
})
```

即：`blog.content.uppercase().substring(1,150)`

主要类型：

* string。转换规则：uppercase, substring, date, float, int,
* int。转换规则：sqrt, sin, cos
* date。

处理逻辑：


1. `quake_parser.rs` 解析 Transflow 中的 map DSL。
2. 生成 `quake.rs` 中对应的数据结构
3. 转换为 Transflow，
4. 通过 `js_flow_codegen.rs` 生成最后的函数
    - 反序列化 `element-define.yml`，获利组件的类型定义，设置默认值
    - 将 DSL 中的算子转换为函数代码。

转换算子的处理逻辑：

```rust
for operator in &stream.operators {
    str = format!(
        "{:}.{:}({:})",
        str,
        operator.operator,
        operator.params_stringify().join(",")
    );
}
```

more javascript operators: [https://github.com/codalien/operator-overloading-js](https://github.com/codalien/operator-overloading-js)

### Mapping 函数（临时 API）

上述的 `from('todo','blog').to(<quake-calendar>);` 会在转化时生成特定的数据结构。因此，也可以直接从数据结构中读取对应的 Transflow，对它们进行存储：

```yaml
- name: "from_todo_blog_to_quake_calendar_timeline"
  from: [ "todo", "blog" ]
  to: "<quake-calendar>"
  mapping:
    - entry: "todo"
      source: ["title", "content", "start_time", "updated_date"]
      target: ["title", "content", "created_date", "updated_date"]
    - entry: "blog"
      source: ["title", "content", "start_time", "updated_date"]
      target: ["title", "content", "created_date", "updated_date"]
```

