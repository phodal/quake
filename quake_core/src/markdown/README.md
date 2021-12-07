# Syntax

1. page link

```markdown
Link to a page: [[Internal link]].
```

2. embed file

```
![[Obsidian#What is Obsidian]]
```

3. callout

```swift
/**
  An example of using the attention field
 
  - Attention: What I if told you
  you read this line wrong?
*/
```

 - Note:
 - 

```swift
case .callout(let callout):
    return Fragment {
        BlockQuote {
            "\(callout.delimiter.rawValue.capitalized): \(callout.content)"
        }
    }
```

or 

Admonitions

```
!!! note "An optional title"
    Here is something you should pay attention to.
```

4. run code

```python-repl
// deps:

```


5. @contents

[Documenter.jl](https://juliadocs.github.io/Documenter.jl/stable/man/syntax/#@raw-format-block)

```@contents
Pages = ["foo.md"]
Depth = 5
```

```@raw html
<svg style="display: block; margin: 0 auto;" width="5em" heigth="5em">
	<circle cx="2.5em" cy="2.5em" r="2em" stroke="black" stroke-width=".1em" fill="red" />
</svg>
```

Refs:

 - Apple's [ Markup Formatting Reference ](https://developer.apple.com/library/archive/documentation/Xcode/Reference/xcode_markup_formatting_ref/Attention.html#//apple_ref/doc/uid/TP40016497-CH29-SW1)
