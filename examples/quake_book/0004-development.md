---
title: Development
created_date: 2021-12-10 19:21:29
updated_date: 2021-12-10 19:21:29
order: 
author: 
---

## Quake Markdown: QuakeDown 处理逻辑

1. 通过 marked.js 解析 markdown，获得 `block` 级别的 token
2. 遍历 token 构造不同类型（如 heading、code、list 等），生成新的 markdown 渲染数据。
3. 遍历 markdownData，根据不同的条件渲染出 `block` 级别的 HTML，重写部分 `inline` 级别的 markdown。
4. 分别为 `inline` 绑定事件。

相关代码：

- `extensions.ts` 相关扩展的声明代码。
- `quake-down.ts` 重新生成 markdown 数据。
- `quake-render.ts` 渲染页面。

## 添加新的 markdown 语法

1. 在 quake-render/src/utils/quake-down.ts 的 `quake-down.ts` 在 `extensions()` 方法中添加新的匹配规则。如：

```javascript
const admonition = {
  name: 'admonition',
  level: 'block',  // marked.js 支持 `block` 和 `inline` 两种类型
  start(src) {
    return src.match(/!!!/)?.index;
  },
  tokenizer(src) {
    const rule = /^!!! ([\w\-]+)(?: "([^\n]*?)")?(?:\s*\n|\s*$)((?:(?:\t| {4})[^\n]+(?:\n|$)|\s*(\n|$))*)?/;
    const match = rule.exec(src);
    if (match) {
      return {
        type: 'admonition',
        raw: match[0],
        display_type: match[1]?.trim(),
        title: match[2]?.trim(),
        body: match[3]?.trim(),
      };
    }
  },
  renderer(token) {
    return `<a>${token}</a>`;
  },
};
```

2. 在 `handleCustomBlock` 添加新类型的自定义数据。

3. 根据定义的是 `block`（块）或者 `inline`（内联）属性，来决定渲染方式。
   - `block` 需要在 `quake-render.tsx` 在 `conditionRender` 方法中添加渲染方式。
   - `inline` 类型需要在：`quake-render.tsx` 在 `parseInline` 方法中添加渲染方式。



