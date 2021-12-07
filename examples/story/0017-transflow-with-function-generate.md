---
title: transflow with function generate
author: Phodal
status: Doing
priority: Low
created_date: 2021-12-05 02:03:24
updated_date: 2021-12-05 02:03:24
---

1. transforms.js for loading and process

2. use codeshift to generate different data struct

3. import transform.js to core

4. provide some commons functions


## Define flows:

1. use QuakeParser to parse block and generate function
2. function binding to `.yaml` or `.js` files
3. loading to webserver for core.

one commmits

```bash
define { from("todo", "blog", "yiki").to(<quake-calendar>) }
```

second commits

```
define {
    from("todo").to("simple_todo"),
    from("simple_todo", "todo").to(<quake-calendar>);
}    
```

parsed:

```json
{
  "routes": [
    {
      "from": [
        {
          "field": ""
        }
      ],
      "to": "",
      "name": "",
      "is_end_way": ""
    }
  ],
  "defines": {
    "todo": {},
    "blog": {},
    "yiki": {}
  },
  "target": "quake-calendar"
}
```

simple query for expression: `simple("${body.address.street}")`； 

also generate `quake-calendar` date_type from TypeScript/JavaScript to Yaml.

```typescript
interface QuakeCalendar {
   input: {
      data: ""
   },
   output: {
      event: ""
   } 
}
```

and also output defines if it will save:

```javascript

```

## Camel DSL examples

YAML: https://camel.apache.org/components/3.13.x/others/yaml-dsl.html

```yaml
- from: (1)
    uri: "direct:start"
    steps: (2)
      - filter:
          expression:
            simple: "${in.header.continue} == true"
          steps: (2)
            - to:
                uri: "log:filtered"
      - to:
          uri: "log:original"
```

