# my-component



<!-- Auto Generated Below -->


## Properties

| Property   | Attribute   | Description | Type      | Default |
| ---------- | ----------- | ----------- | --------- | ------- |
| `content`  | `content`   |             | `string`  | `''`    |
| `hasEmbed` | `has-embed` |             | `boolean` | `false` |


## Events

| Event            | Description | Type                |
| ---------------- | ----------- | ------------------- |
| `clickEmbedLink` |             | `CustomEvent<Link>` |
| `clickPageLink`  |             | `CustomEvent<Link>` |


## Dependencies

### Used by

 - [embed-link](../embed-link)

### Depends on

- [data-grid](../data-grid)
- [graph-bar](../graph-bar)
- [graph-line](../graph-line)
- [graph-transflow](../graph-transflow)

### Graph
```mermaid
graph TD;
  quake-render --> data-grid
  quake-render --> graph-bar
  quake-render --> graph-line
  quake-render --> graph-transflow
  embed-link --> quake-render
  style quake-render fill:#f9f,stroke:#333,stroke-width:4px
```

----------------------------------------------

*Built with [StencilJS](https://stenciljs.com/)*
