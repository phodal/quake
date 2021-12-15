---
title: filter for parse item
author: 
status: Spike
priority: Low
created_date: 2021-12-09 09:32:28
updated_date: 2021-12-09 09:32:28
---

## MeiliSearch Query Parser

MeiliSearch Core: [https://github.com/meilisearch/milli](https://github.com/meilisearch/milli)

a concurrent indexer combined with fast and relevant search algorithms

BNF:

```text
filter         = expression ~ EOF
expression     = or
or             = and (~ "OR" ~ and)
and            = not (~ "AND" not)*
not            = ("NOT" ~ not) | primary
primary        = (WS* ~ "("  expression ")" ~ WS*) | geoRadius | condition | to
condition      = value ("==" | ">" ...) value
to             = value value TO value
value          = WS* ~ ( word | singleQuoted | doubleQuoted) ~ WS*
singleQuoted   = "'" .* all but quotes "'"
doubleQuoted   = "\"" .* all but double quotes "\""
word           = (alphanumeric | _ | - | .)+
geoRadius      = WS* ~ "_geoRadius(" ~ WS* ~ float ~ WS* ~ "," ~ WS* ~ float ~ WS* ~ "," float ~ WS* ~ ")"
```

## Elasticsearch Range Query

https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-range-query.html


```json
{
  "query": {
    "range": {
      "age": {
        "gte": 10,
        "lte": 20,
        "boost": 2.0
      }
    }
  }
}
```

## Elasticsearch DSL

[https://github.com/cch123/elastic-rs](https://github.com/cch123/elastic-rs)

Grammer

```
bool_expr = { SOI ~ expr ~ EOI }

expr = {
    (paren_bool | comp_expr) ~ ( (and_op|or_op)~ (paren_bool| comp_expr))*
}

and_op = { "and" }
or_op = { "or" }

paren_bool = { "(" ~ expr ~  ")" }

comp_expr = { field ~ op ~ value }

field = @{ (ASCII_ALPHA ~ ASCII_ALPHANUMERIC*) }
op = { eq | neq | op_in | op_not_in | gt | gte | lt | lte | like | not_like }
eq = { "=" }
neq = { "!=" | "<>"}
op_in = { "in" }
op_not_in= { "not" ~ "in"}
gt = { ">" }
gte = { ">=" }
lt = { "<" }
lte = { "<=" }
like = { "like" }
not_like = { "not" ~ "like" }

value = {
    string_literal
    | num_literal
    | "(" ~ string_literal ~("," ~ string_literal)* ~ ")"
    | "(" ~ num_literal ~("," ~ num_literal)* ~ ")"
}

num_literal = @{
    "-"?
    ~ ("0" | ASCII_NONZERO_DIGIT ~ ASCII_DIGIT*)
    ~ ("." ~ ASCII_DIGIT*)?
    ~ (^"e" ~ ("+" | "-")? ~ ASCII_DIGIT+)?
}

string_literal = ${ "\"" ~ string ~ "\"" }
string = @{ char* }
char = {
    !("\"" | "\\") ~ ANY
    | "\\" ~ ("\"" | "\\" | "/" | "b" | "f" | "n" | "r" | "t")
    | "\\" ~ ("u" ~ ASCII_HEX_DIGIT{4})
}

WHITESPACE = _{ " " | "\n" | "\r" }
```

## Human language

Commons Range

```bash
now
today
tomorrow
yesterday
last/this/next week
last/this/next month
last/this/next year
```

Range

```
2011
2011-03
2011-03-04

2011-03-04 04
```

Atlas Sample:

- https://atlas.apache.org/#/SearchAdvance

Antlr DSL: [AtlasDSLParser.g4](https://github.com/apache/atlas/blob/master/repository/src/main/java/org/apache/atlas/query/antlr4/AtlasDSLParser.g4)

```sql
from Table select owner as Owner, name as Name, qualifiedName as FullName
```
