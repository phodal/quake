---
title: Phodal.com Process
language: JavaScript
---

```javascript
let text = "";
$("h2.mdl-card__title-text > a").each(function(index, link) {
    text += `[${link.innerText}](${link.href}) \n\n`
});

console.log(text);
```

