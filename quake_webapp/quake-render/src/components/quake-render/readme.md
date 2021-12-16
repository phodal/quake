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

- [graph-bar](../graph-bar)
- [graph-line](../graph-line)

### Graph
```mermaid
graph TD;
  quake-render --> graph-bar
  quake-render --> graph-line
  embed-link --> quake-render
  style quake-render fill:#f9f,stroke:#333,stroke-width:4px
```

----------------------------------------------

*Built with [StencilJS](https://stenciljs.com/)*
