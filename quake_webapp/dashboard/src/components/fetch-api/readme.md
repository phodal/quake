# fetch-api



<!-- Auto Generated Below -->


## Properties

| Property       | Attribute       | Description | Type       | Default |
| -------------- | --------------- | ----------- | ---------- | ------- |
| `entryType`    | --              |             | `string[]` | `[]`    |
| `searchEngine` | `search-engine` |             | `boolean`  | `false` |
| `type`         | `type`          |             | `string`   | `''`    |
| `url`          | `url`           |             | `string`   | `''`    |


## Events

| Event             | Description | Type               |
| ----------------- | ----------- | ------------------ |
| `fetchAllSuccess` |             | `CustomEvent<any>` |
| `fetchSuccess`    |             | `CustomEvent<any>` |


## Dependencies

### Used by

 - [quake-dashboard](../quake-dashboard)

### Graph
```mermaid
graph TD;
  quake-dashboard --> fetch-api
  style fetch-api fill:#f9f,stroke:#333,stroke-width:4px
```

----------------------------------------------

*Built with [StencilJS](https://stenciljs.com/)*
