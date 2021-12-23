---
title: filter for flow codegen
author: 
status: Done
priority: Low
created_date: 2021-12-08 10:01:36
updated_date: 2021-12-08 10:01:36
---

## JSON Filter

[https://platform.data-axle.com/people/docs/filter_dsl](https://platform.data-axle.com/people/docs/filter_dsl)

samples 1

```bash
curl -X GET https://api.data-axle.com/v1/people/search -d '{
  "filter": {
    "relation": "equals",
    "attribute": "state",
    "value": "NY",
    "negated": true
  }
}'
```

samples 2

```bash
curl -X GET https://api.data-axle.com/v1/people/search -d '{
  "filter": {
    "relation": "in",
    "attribute": "state",
    "value": ["WA", "OR", "ID", "CA"]
  }
}'
```

map {
  created_date -> start_time
}