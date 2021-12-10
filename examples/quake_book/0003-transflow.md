---
title: Transflow
created_date: 2021-12-10 19:21:10
updated_date: 2021-12-10 19:21:10
order: 
author: 
---

Transflow 的设计起源需要那篇《[类型流（TypeFlow）——世俗化的函数式编程和改进的过程式设计](https://zhuanlan.zhihu.com/p/341089716)》说起。Typeflow 有几个主要的规则：

1. 从可视化模型上就可以看出：共存在4个待实现的函数，其中两个纯函数，参数校验和返回结果包装；一个副作用函数，保存代办事项；还有一个输入端口，即把这个几个函数编排起来完成业务的程序入口。
2. 每个函数有明确的输入输出类型
3. 函数之间通过匹配的输入输出类型连接起来。
4. 输入输出类型使用业务人员能够理解的业务概念，从而符合DDD的要求。
5. 可视化

简单来说，在有了设计之后，我们可以为功能生成对应的有输入和输出的函数，并可以通过规则将它们匹配起来。原理，就是这么简单。

## Transflow 初识：DSL 生成代码

回到 Quake 的场景里，我们有固定的数据源，即不同的 entry。与此同时我们还有不同的用于展示这些数据的组件。

然后，我们所需要做的便是，提供一个从数据流向组件的规则，即 Transflow DSL。

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

这样一来，只需要跳转到相应的路由即可。如果需要的话，也可以直接生成临时的组件。另外一部分，则是由 Web Components 所构建的组件体系，我们将会在另外一篇文章中展开介绍。

### 多条 Transflow

（PS：虽然尚未进行测试，但是我相信它当前是**不** work 的）

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

## 目标函数式的 Transflow

既然，我们是对数据流进行操作，那么理想情况下，Transflow 的 DSL 就可以设计为向函数式靠齐。不过，当前，我们还没有理由实现这么复杂的功能，可以在后续展开。

### 一个实现一点点的 map

上述的 `from('todo','blog').to(<quake-calendar>);` 会在转化时生成特定的数据结构。因此，也可以直接从数据结构中读取对应的 Transflow，对它们进行存储：

```yaml
- name: "from_todo_blog_to_quake_calendar_timeline"
  from: [ "todo", "blog" ]
  to: "<quake-calendar-timeline>"
  map:
    - entry: "todo"
      source: ["title", "content", "start_time", "updated_date"]
      target: ["title", "content", "created_date", "updated_date"]
    - entry: "blog"
      source: ["title", "content", "start_time", "updated_date"]
      target: ["title", "content", "created_date", "updated_date"]
```


这里的 `map` 是一个尚未在 DSL 设计的功能，也需要进一步验证是否真的需要。除此，这个 YAML 的设计也是有问题的。

### 还有，一个刚可用的 filter

在 filter 方面，我做了一些简化设计（\~~~偷懒~~\~），因为需要的是搜索引擎，可以可以直接使用搜索引擎的 fliter 功能。在评估了多个 filter-parser 的库之后，我发现没有理由在当前做这么复杂的设计。所以，针对于一些特别的过滤条件做了一些特别的处理。

如下是一个过滤时间的表达式：

```javascript
from('todo','blog').to(<quake-calendar>).filter('created_date > 2021.01.01 AND created_date < 2021.12.31')
```

由于搜索引擎并不支持各种各样的时间处理，所以我们可以替换对应的字符器，然后：

```javascript
created_date > 1609459200 AND created_date < 1640908800
```

等时机成熟，再完成整体的 filter 规则设计。

## 下一步：更简单的 Transflow

还在设计中，预期可能会有组件中的编排等。不过，首先我们得需要有足够的 Web Components 组件，才能完成基本的功能开发，并收集这些数据场景。诸如于：

- [ ] Todo 应用
- [ ] Kanban 应用
- [ ] Typeform 编辑器
- [ ] 白板
- [ ] ……

### 双向绑定的中间组件：ComponentFlow

理想的情况下，我们应该在 Transflow 中生成的是一个新的 Web Components 组件，以提供数据到组件的通道。只是呢，当前受限于当前的场景有限，所以提供的是简单的代码生成。等组件库进一步完善之后，便可以尝试引入这个新的设计。

### 面向专业人士的 Transflow

在 Quake 现有的设计里，专业人士可以自由自在的对 Quake 进行定制，所以并不需要高级的 Transflow 存在。
