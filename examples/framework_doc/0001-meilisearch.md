---
title: meilisearch
created_date: 2021-12-07 21:18:03
updated_date: 2021-12-07 21:18:03
category: search engine
---

# Quake Search

## Setup diesel

Create

```bash
curl \
  -X POST 'http://127.0.0.1:7700/indexes/phodal_com/documents' \
  -H 'Content-Type: application/json' \
  --data-binary @dump.json
```

Settings

```bash
curl \
  -X POST 'http://localhost:7700/indexes/blog/settings' \
  -H 'Content-Type: application/json' \
  --data-binary '{
      "rankingRules": [
          "words",
          "typo",
          "proximity",
          "attribute",
          "sort",
          "exactness",
          "created_date:desc",
          "rank:desc"
      ],
      "distinctAttribute": null,
      "searchableAttributes": [
          "title",
          "content"
      ],
      "displayedAttributes": [
          "id",
          "title",
          "created_date"
      ],
      "stopWords": null,
      "sortableAttributes": [
        "title",
        "created_date",
        "content"
      ],
      "synonyms": null
  }'
```

Query

```bash
curl \
  -X POST 'http://localhost:7700/indexes/blog/settings' \
  -H 'Content-Type: application/json' \
  --data-binary @resources/search_rule.json
```

Delete

```bash
curl -X DELETE 'http://localhost:7700/indexes/phodal_com/documents'
```

