---
title: auto generate pdf papers to entry
author: 
status: Spike
priority: Low
created_date: 2021-12-29 08:20:11
updated_date: 2021-12-29 20:06:11
---

Set auto generated properties for node-info ? or in entries define?

1. watch path for pdf files join
2. generate pdf content
3. rename pdf content papers
4. search support in quake

##

```yaml
generate_rules:
   - engine: pdf
     flow: from("file").to("content")
   - engine: epub
     flow: from("file").to("content")
```

or

```yaml
generate_rules:
    - from("file").to("content").processor("epub")
    - from("file").to("content").processor("pdf")
    - from("file").to("content").processor("mobi")
```
