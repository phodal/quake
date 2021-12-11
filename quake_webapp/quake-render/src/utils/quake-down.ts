import { marked, Renderer, Slugger } from 'marked';
import Token = marked.Token;
import TokensList = marked.TokensList;

class QuakeDown {
  content = '';
  token = null;
  tokens: TokensList | any = [];
  markdownData: any[] = [];
  slugger = new Slugger();

  headingIndex = 0;
  headingMap = {};
  indexHeadingMap = {};

  constructor(content: string) {
    this.content = content;
  }

  private next(): Token {
    this.token = this.tokens.pop();
    return this.token;
  }

  // @ts-ignore
  private peek() {
    return this.tokens[this.token.length - 1] || 0;
  }

  gen() {
    const tokens = marked.lexer(this.content);
    this.tokens = tokens.reverse();

    while (this.next()) {
      const token: Token = this.token;
      this.tok(token);
    }

    return this.markdownData;
  }

  private tok(token: marked.Token) {
    switch (token.type) {
      case 'heading':
        this.markdownData.push({
          type: 'heading',
          depth: token.depth,
          text: this.renderInline(token.tokens),
          headingIndex: this.headingIndex,
          anchor: this.slugger.slug(this.unescape(this.renderInline(token.tokens))),
        });
        break;
      case 'blockquote':
        this.markdownData.push({ type: 'blockquote', text: token.text, raw: token.raw });
        break;
      case 'hr':
        this.markdownData.push({ type: 'hr', raw: token.raw });
        break;
      case 'space':
        this.markdownData.push({ type: 'space', raw: token.raw });
        break;
      case 'paragraph':
        this.markdownData.push({
          type: 'paragraph',
          data: this.renderInline(token),
        });
        break;
      case 'list':
        this.markdownData.push({
          type: 'list',
          start: token.start,
          ordered: token.ordered,
          loose: token.loose,
          items: token.items
        })
        break;
      case 'table':
        let align = token.align;
        let header = this.buildTableHeader(token.header);
        let rows = this.buildTableRows(token.rows);
        this.markdownData.push({
          type: 'table',
          align,
          rows,
          header,
        });
        break;
      default:
        console.log(token);
    }
  }

  private renderInline(tokens: any) {
    let renderer = new marked.Renderer();
    return this.parseInline(tokens, renderer);
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

  // todo: parse inline
  parseInline(tokens, renderer) {
    let out = '',
      i,
      token;

    const l = tokens.length;
    for (i = 0; i < l; i++) {
      token = tokens[i];

      switch (token.type) {
        case 'escape': {
          out += renderer.text(token.text);
          break;
        }
        case 'html': {
          out += renderer.html(token.text);
          break;
        }
        case 'link': {
          out += renderer.link(token.href, token.title, this.parseInline(token.tokens, renderer));
          break;
        }
        case 'image': {
          out += renderer.image(token.href, token.title, token.text);
          break;
        }
        case 'strong': {
          out += renderer.strong(this.parseInline(token.tokens, renderer));
          break;
        }
        case 'em': {
          out += renderer.em(this.parseInline(token.tokens, renderer));
          break;
        }
        case 'codespan': {
          out += renderer.codespan(token.text);
          break;
        }
        case 'br': {
          out += renderer.br();
          break;
        }
        case 'del': {
          out += renderer.del(this.parseInline(token.tokens, renderer));
          break;
        }
        case 'text': {
          out += renderer.text(token.text);
          break;
        }
        default: {
          const errMsg = 'Token with "' + token.type + '" type was not found.';
          console.error(errMsg);
        }
      }
    }
    return out;
  }
}

export default QuakeDown;
