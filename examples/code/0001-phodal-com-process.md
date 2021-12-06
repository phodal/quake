---
title: Phodal.com Process
created_date: 2021-12-02 20:06:20
updated_date: 2021-12-02 20:06:20
---

```javascript
let text = "";
$("h2.mdl-card__title-text > a").each(function(index, link) {
    text += `[${link.innerText}](${link.href}) \n\n`
});

console.log(text);
```

