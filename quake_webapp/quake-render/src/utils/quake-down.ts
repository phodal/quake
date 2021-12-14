import { marked, Slugger } from 'marked';
import Prism from 'prismjs';
import getExtensions from "./extensions";

class QuakeDown {
  content = '';
  slugger = new Slugger();
  renderer: marked.Renderer;

  parseInline: (tokens, renderer) => string;

  constructor(content: string, parseInline: (tokens, renderer) => string) {
    this.parseInline = parseInline;
    this.content = content;
  }

  build() {
    marked.use({
      extensions: QuakeDown.extensions(),
    });
    this.renderer = new marked.Renderer();

    return this.build_data(this.content);
  }

  // add extensions in extension.ts file for split
  private static extensions() {
    return getExtensions();
  }

  private build_data(src: string) {
    let output = [];
    const tokens = marked.lexer(src);

    for (let token of tokens) {
      output.push(this.tok(token));
    }

    return output;
  }

  private tok(token: marked.Token) {
    let data: any;
    switch (token.type) {
      case 'heading':
        data = {
          type: 'heading',
          depth: token.depth,
          text: this.renderInline(token.tokens),
          anchor: this.slugger.slug(this.unescape(this.renderInline(token.tokens))),
        };
        break;
      case 'blockquote':
        data = { type: 'blockquote', text: token.text, raw: token.raw };
        break;
      case 'hr':
        data = { type: 'hr', raw: token.raw };
        break;
      case 'space':
        data = { type: 'space', raw: token.raw };
        break;
      case 'paragraph':
        data = {
          type: 'paragraph',
          text: this.renderInline(token.tokens),
        };
        break;
      case 'list':
        let children = [];
        for (let item of token.items) {
          children.push(this.tok(item));
        }

        data = {
          type: 'list',
          children: children,
          start: token.start,
          ordered: token.ordered,
          loose: token.loose,
          items: token.items,
        };

        break;
      case 'list_item':
        let child = [];
        for (let item of token.tokens) {
          if (item.type == 'list') {
            child.push(this.tok(item));
          }
        }

        data = {
          type: 'list_item',
          children: child,
          text: this.renderInline((token.tokens[0] as marked.Tokens.Text).tokens),
          loose: token.loose,
          checked: token.checked,
          task: token.task,
        };
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
        };
        break;
      case 'code':
        let text = token.text;
        if (Prism.languages[token.lang]) {
          text = Prism.highlight(token.text, Prism.languages[token.lang], token.lang);
        }
        data = {
          type: 'code',
          lang: token.lang,
          text: text,
        };
        break;
      default:
        let custom_type = token as any;
        data = this.handleCustomBlock(custom_type);
    }

    return data;
  }

  private handleCustomBlock(token) {
    let data;
    switch (token.type) {
      case 'admonition':
        let content = this.build_data(token.body);
        data = {
          type: 'admonition',
          title: token.title,
          display_type: token.display_type,
          data: content,
          raw: token.raw,
        };
        break;
      case 'page_link':
        data = {
          type: 'page_link',
          raw: token.raw,
        };
        break;
      default:
        console.log(token);
    }

    return data;
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
