---
title: pagelink storage
author: 
status: Spike
priority: Low
created_date: 2021-12-14 23:26:04
updated_date: 2021-12-14 23:26:04
---

need a syntax or file for storage items: Hashmap for search and index

graphviz

```
a -> b
b -> c
```

Plantuml

```
a -> b
b -> c
```

## idea 0.1: all in one

also need to index:

```yaml
notes:
 - source: 01
 - target:
     - story: 01, 02, 03, 04
```

## idea 0.2: one by one

by paths:

```bash
links
├── blog.yaml
└── notes.yaml
```
