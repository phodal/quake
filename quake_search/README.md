# Quake Search

## Setup diesel


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
      "searchableAttributes": [
          "title",
          "description",
          "keywords",
          "content"
      ],
      "displayedAttributes": [
          "title",
          "description",
          "keywords",
          "content",
          "created_date"
      ],
      "stopWords": null,
      "sortableAttributes": [
        "title",
        "keywords",
        "content"
      ],
      "synonyms": null
  }'
 
```