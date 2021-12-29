---
title: auto generate pdf papers to entry
author: 
status: Spike
priority: Low
created_date: 2021-12-29 08:20:11
updated_date: 2021-12-29 20:06:11
---

Set auto processing pdf files;

1. watch path for pdf files join
2. processing pdf content
3. rename pdf content papers
4. search support in quake

##

```yaml
process_rules:
   - engine: pdf
     flow: from("file").to("content")
   - engine: epub
     flow: from("file").to("content")
```

or

```yaml
process_rules:
    - from("file").to("content").processor("epub")
    - from("file").to("content").processor("pdf")
    - from("file").to("content").processor("mobi")
```
