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
    properties:
      - title: Title
      - author: String
      - content: Body
      - status: Flow
      - priority: State
      - created_date: Date
      - updated_date: Date
    flows:
      - property: status
        items: ['Spike', 'Todo', 'Doing', 'Done']
    states:
      - property: priority
        items: ['Low', 'Medium', 'High']
```

## property type: Date

```yaml
- explain: a date property will be convert to a standard Unix timestamp.
- ability:
    - can be filter by date
- examples:
    - created_date > 2021.01.01
    - created_date > 2021.01.01 AND created_date < 2021.12.31
```

## property type: Flow (Todo)

```yaml
- explain: a flow can show in Kanban mode and loging to system.
- ability:
    - kanban model
    - loging
```

a changing example (TODO):

```yaml
quake_change:
  - 2021-12-09 09:32:28 "Todo"
  - 2021-12-09 09:40:28 "Spike" -> "Todo"
  - 2021-12-10 12:12:28 "Todo" -> "Doing"
  - 2021-12-10 12:12:28 "Doing" -> "Done"
```

## property type: File

support for content process, such as `.pdf` file:

```yaml
- type: papers
  display: ""
  processors:
    file_engines: ['pdf', 'mobi', 'epub']
  properties:
    - title: Title
    - file: File
```

## property type: Attachment

support for open attachment, or viewer?

```yaml
- type: papers
  display: ""
  processors:
    file_flows:
      - from("file").filter(regex("*.pdf")).to(<pdf-view>)
  properties:
    - title: Title
    - file: Attachment
```

