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

## DSL design

```yaml
processors:
  file_engines: ['pdf', 'mobi', 'epub']
```



```yaml
processors:
  file_flows:
     - from("file").to("content").processor(["epub", "pdf", "mobi"])
```

also custom engine for whiteboard?

```yaml
processors:
  file_flows:
    - from("graph").to("content").processor(component("whiteboard"))
```
