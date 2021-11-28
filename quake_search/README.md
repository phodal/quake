# Quake Search

## Setup diesel

Create

```bash
curl \
  -X POST 'http://127.0.0.1:7700/indexes/microsoft_todo/documents' \
  -H 'Content-Type: application/json' \
  --data-binary @dump.json
```

Query

```bash
curl \
  -X POST 'http://localhost:7700/indexes/phodal_com/settings' \
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
      "searchableAttributes": null,
      "displayedAttributes": null,
      "stopWords": null,
      "sortableAttributes":null,
      "synonyms": null
  }'
 ``
```

Delete

```bash
curl -X DELETE 'http://localhost:7700/indexes/phodal_com/documents'
```