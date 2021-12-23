---
title: mapping dsl for transflow
author: 
status: Spike
priority: Low
created_date: 2021-12-23 23:15:25
updated_date: 2021-12-23 23:15:25
---

default by fields:

```
.created_date, blog.date -> date, blog.created_date
```

one by one mode:

```
blog.title -> title, blog.content -> content, blog.action -> action
```

use `pipe` symbol as pipe

with convert:

```
blog.title -> title | restrict([length = 150, first = "。", toUppercase])
```

- method for string
- method for int
- method for object

```
blog.id -> id | limit(150) | first("。") | string_to_int
```

multiple

```
blog.title -> title | uppercase | lowercase,
```

size(blog, 150)

methods:

- limit, limit by length
- first/last(T = string) = split by chars
- first/last(T = number) = split by length
- uppercase/lowercase

- int = parseInt
- string = toString
- type system?


- date(str string) = toDate

- sqrt | sin/cos/ = math functional
- regex(string) = default get first

- other custom methods


load custom `transflow.lib.js`

