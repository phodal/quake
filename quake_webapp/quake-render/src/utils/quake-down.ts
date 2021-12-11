import { marked, Slugger } from 'marked';
import TokensList = marked.TokensList;
import Token = marked.Token;

class QuakeDown {
  content = '';
  tokens: TokensList | any = [];
  markdownData: any[] = [];
  slugger = new Slugger();

  parseInline: (tokens, renderer) => string;

  headingIndex = 0;

  renderer: marked.Renderer;
  token: Token;

  constructor(content: string, parseInline: (tokens, renderer) => string) {
    this.parseInline = parseInline;
    this.content = content;
  }

  extensions(): any {
    const admonition = {
      name: 'admonition',
      level: 'block',
      start(src) {
        return src.match(/!!!/)?.index;
      },
      tokenizer(src) {
        /// modified from https://github.com/haishanh/hs-marked-extra/blob/master/lib/marked_extra.js
        /// LICENSE MIT
        const rule = /^!!! ([\w\-]+)(?: "([^\n]*?)")?(?:\s*\n|\s*$)((?:(?:\t| {4})[^\n]+(?:\n|$)|\s*(\n|$))*)?/;
        const match = rule.exec(src);
        if (match) {
          return {
            type: 'admonition',
            raw: match[0],
            title: match[2].trim(),
            body: match[3].trim(),
          };
        }
      },
      renderer(token) {
        return `<a>${token}</a>`;
      },
    };
    const page_link = {
      name: 'page_link',
      level: 'inline',
      tokenizer(src, _tokens) {
        const rule = /^\[\[(.*)\]\]/;
        const match = rule.exec(src);
        if (match) {
          return {
            type: 'page_link',
            raw: match[0],
          };
        }
      },
      renderer(token) {
        return `<a>${token}</a>`;
      },
    };

    return [page_link, admonition];
  }

  gen() {
    marked.use({
      extensions: this.extensions()
    });
    const tokens = marked.lexer(this.content);
    this.renderer = new marked.Renderer();
    this.tokens = tokens.reverse();

    while (this.next()) {
      const token: Token = this.token;
      this.markdownData.push(this.tok(token));
    }
    return this.markdownData;
  }

  private next(): Token {
    this.token = this.tokens.pop();
    return this.token;
  }

  // private peek() {
  //   return this.tokens[this.tokens.length - 1] || 0;
  // }

  private parseList(items: marked.Tokens.ListItem[]) {
    let result = [];
    for (let item of items) {
      console.log(item);
      let list_item = {
        type: 'list_item',
        text: item.text,
        checked: item.checked,
        task: item.task,
        loose: item.loose,
        children: []
      };

      result.push(list_item)
    }

    return result;
  }

  private tok(token: marked.Token) {
    let data: any;
    switch (token.type) {
      case 'heading':
        data = {
          type: 'heading',
          depth: token.depth,
          text: this.renderInline(token.tokens),
          headingIndex: this.headingIndex,
          anchor: this.slugger.slug(this.unescape(this.renderInline(token.tokens))),
        }
        break;
      case 'blockquote':
        data = { type: 'blockquote', text: token.text, raw: token.raw }
        break;
      case 'hr':
        data = { type: 'hr', raw: token.raw }
        break;
      case 'space':
        data = { type: 'space', raw: token.raw }
        break;
      case 'paragraph':
        data = {
          type: 'paragraph',
          text: this.renderInline(token.tokens),
        }
        break;
      case 'list':
        let children = this.parseList(token.items);

        data = {
          type: 'list',
          children: children,
          start: token.start,
          ordered: token.ordered,
          loose: token.loose,
          items: token.items,
        }

        break;
      case 'list_item':
        //
        break;
      case 'table':
        let align = token.align;
        let header = this.buildTableHeader(token.header);
        let rows = this.buildTableRows(token.rows);
        data = {
          type: 'table',
          align,
          rows,
          header,
        }
        break;
      default:
        let custom_type = token as any;
        switch (custom_type.type) {
          case 'admonition':
            data = {
              type: 'admonition',
              title: custom_type.title,
              body: custom_type.body,
              raw: custom_type.raw,
            }
            break;
          case 'page_link':
            data = {
              type: 'page_link',
              raw: custom_type.raw
            }
            break;
          default:
            // console.log(token);
        }
    }

    return data
  }

  private renderInline(tokens: any) {
    return this.parseInline(tokens, this.renderer);
  }

  private buildTableHeader(cells: marked.Tokens.TableCell[]) {
    const results = [];
    for (const cell of cells) {
      results.push(this.renderInline(cell.tokens));
    }
    return results;
  }

  private buildTableRows(cells: marked.Tokens.TableCell[][]) {
    const results = [];
    for (const column of cells) {
      const newCol = [];
      for (const cell of column) {
        newCol.push(this.renderInline(cell.tokens));
      }

      results.push(newCol);
    }
    return results;
  }

  private unescape(html) {
    const unescapeTest = /&(#(?:\d+)|(?:#x[0-9A-Fa-f]+)|(?:\w+));?/gi;
    return html.replace(unescapeTest, (_, n) => {
      n = n.toLowerCase();
      if (n === 'colon') {
        return ':';
      }
      if (n.charAt(0) === '#') {
        return n.charAt(1) === 'x'
          ? String.fromCharCode(parseInt(n.substring(2), 16))
          : String.fromCharCode(+n.substring(1));
      }
      return '';
    });
  }
}

export default QuakeDown;
