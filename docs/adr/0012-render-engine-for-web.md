# 12. render engine for web

Date: 2021-11-27

## Status

2021-11-27 proposed

## Context

Sample:

 - [https://github.com/ledge-framework/engine](https://github.com/ledge-framework/engine)


### Mathematics

/// https://katex.org/
/// use: https://www.mathjax.org/

``A x^2 + B x + C = 0``

### Comments

```
/// some comments

```

### Link relations

```
<entry_type>:<entry_id> "<entry_title>" -> <entry_type>:<entry_id> "<entry_title>" 
```

```
note:0001 "title" -> content:0001 "2222" 
note:0002 -> title
```

exact link

[[note:0001 "markdown-syntax"]]

ambiguous link

[["title"]]

page link

### Admonition

```
!!! note "An optional title"
    Here is something you should pay attention to.
```

### Auto Todo

```
>>> todo "title"
    do something
```

### Code


#### interactive code

// like: https://juliadocs.github.io/Documenter.jl/stable/showcase/#Running-interactive-code 

Input:

```@example("java")
code_typed(sqrt, (Float64,))
```

Output:

```

```

#### repl code

run step by steps for repl languages

```@repl("repl") #1
using Statistics
xs = collect(1:10)
median(xs)
sum(xs)
```

`#1` to tag code, can be auto insert

```@repl_block("javascript")

```

#### auto code connect

```@connect

```

## Decision

Decision here...

## Consequences

Consequences here...
