---
title: Development
created_date: 2021-12-10 19:21:29
updated_date: 2021-12-10 19:21:29
order: 
author: 
---

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