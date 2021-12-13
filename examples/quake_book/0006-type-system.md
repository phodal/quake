---
title: Type System
created_date: 2021-12-13 08:11:40
updated_date: 2021-12-13 12:10:40
order: 
author: 
---

Examples:

```yaml
  - type: story
    display: "Story"
    fields:
      - title: Title
      - author: String
      - content: Body
      - status: Flow
      - priority: State
      - created_date: Date
      - updated_date: Date
    flows:
      - field: status
        items: ['Spike', 'Todo', 'Doing', 'Done']
    states:
      - field: priority
        items: ['Low', 'Medium', 'High']
```

## field type: Date

```yaml
- explain: a date field will be convert to a standard Unix timestamp.
- ability:
    - can be filter by date
- examples:
    - created_date > 2021.01.01
    - created_date > 2021.01.01 AND created_date < 2021.12.31
```

## field type: Flow

```yaml
- explain: a flow can show in Kanban mode and loging to system.
- ability:
    - kanban model
    - loging
```

a changing example:

```yaml
quake_changing:
  - 2021-12-09 09:32:28 "Todo"
  - 2021-12-09 09:40:28 "Spike" -> "Todo"
  - 2021-12-10 12:12:28 "Todo" -> "Doing"
  - 2021-12-10 12:12:28 "Doing" -> "Done"
```

## field type: State

