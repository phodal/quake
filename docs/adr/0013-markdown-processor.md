# 13. markdown processor

Date: 2021-11-27

## Status

2021-11-27 proposed

2021-12-07 accepted

## Context

markdown:

- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) this library is a pull parser for CommonMark, written in Rust. It comes with a simple command-line tool, useful for rendering to HTML, and is also designed to be easy to use from as a library.
- [Comrak](https://github.com/kivikakk/comrak) is a  CommonMark + GFM compatible Markdown parser and renderer.

return to string

- [pulldown-cmark-to-cmark](https://github.com/Byron/pulldown-cmark-to-cmark), A utility library which translates Event back to markdown. It's the prerequisite for writing markdown filters which can work as mdbook-preprocessors.

parser html:

- https://github.com/servo/html5ever

samples:

- https://github.com/zoni/obsidian-export

syntax support:

```markdown
Link to a page: [[Internal link]].
``

Embeds

```
![[Obsidian#What is Obsidian]]
```

## Decision

Decision here...

## Consequences

Consequences here...
