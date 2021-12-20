---
title: heading support for pagelink
author: 
status: Spike
priority: Low
created_date: 2021-12-20 23:08:57
updated_date: 2021-12-20 23:08:57
---

refs samples: 

```rust
^(?P<file>[^#|]+)??(#(?P<section>.+?))??(\|(?P<label>.+?))??$
```

Note#Heading|Label


output

```bash
Some(Captures({
    0: Some("Note#Heading|Label"),
    1: Some("Note"),
    2: Some("#Heading"),
    3: Some("Heading"),
    4: Some("|Label"),
    5: Some("Label"),
})),

```

