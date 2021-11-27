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
          "keywords_string",
          "content"
      ],
      "displayedAttributes": [
          "title",
          "description",
          "keywords_string",
          "content",
          "created_date"
      ],
      "stopWords": null,
      "sortableAttributes": [
        "title",
        "keywords_string",
        "content"
      ],
      "synonyms": null
  }'
 
```