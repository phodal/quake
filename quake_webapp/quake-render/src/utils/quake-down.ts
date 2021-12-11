import { marked, Slugger } from 'marked';
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
          text: this.renderInline(token.text),
          headingIndex: this.headingIndex,
          anchor: this.slugger.slug(this.unescape(this.renderInline(token.text))),
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
          data: this.renderInline(token.text),
        });
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

  private renderInline(tokenText: string) {
    const renderer = new marked.Renderer();
    const linkRenderer = renderer.link;
    renderer.link = (href, title, text) => {
      const html = linkRenderer.call(renderer, href, title, text);
      return html.replace(/^<a /, '<a target="_blank" ');
    };

    return marked.parseInline(tokenText, { renderer });
  }

  private buildTableHeader(cells: marked.Tokens.TableCell[]) {
    const results = [];
    for (const cell of cells) {
      results.push(this.renderInline(cell.text));
    }
    return results;
  }

  private buildTableRows(cells: marked.Tokens.TableCell[][]) {
    const results = [];
    for (const column of cells) {
      const newCol = [];
      for (const cell of column) {
        newCol.push(this.renderInline(cell.text));
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
