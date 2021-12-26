---
title: "mapping dsl for transflow"
author: 
status: Spike
priority: Low
created_date: 2021-12-23 23:15:25
updated_date: 2021-12-24 10:04:16
---

## Design Principle

* reuse JavaScript standard library with default value
  * alias some method for fun, and can be handle in other function
  * math.js for math: <https://github.com/josdejong/mathjs>?
* pipe syntax for convert

## Pipe map Syntax

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

* method for string
* method for int
* method for object

```
blog.id -> id | limit(150) | first("。") | string_to_int
```

multiple

```
blog.title -> title | uppercase | lowercase,
```

math calculate

```javascript
blog.date -> date | int | * 1000
```

size(blog, 150)

methods:

* limit, limit by length

## **Transflow** standard lib

### String handle

```
- first/last(T = string) = split by chars
- first/last(T = number) = split by length
- uppercase/lowercase
- regex(string) = default get first
- replaceAll("a", "b")
- substring()
- trim
```

### Type Casting

* int = parseInt
* float = parseFloat
* string = toString
* date(str string) = toDate

### Math 

use default JavaScript math library

* sqrt | sin/cos/ = math functional
* fixed (parameter)

### Others

* other custom methods


## Tasks


1. load custom `transflow.lib.js`
2. Transform DSL


