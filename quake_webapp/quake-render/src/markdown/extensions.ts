function getAdmonition() {
  return {
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
          display_type: match[1]?.trim(),
          title: match[2]?.trim(),
          body: match[3]?.trim(),
        };
      }
    }
  };
}

function getPageLink() {
  return {
    name: 'page_link',
    level: 'inline',
    tokenizer(src, _tokens) {
      const rule = /^\[\[(.+(?=:)):(\d+)(#(.+)([|\s]))?(|(.+))? "(.+?(?="]]))"]]/;
      const match = rule.exec(src);
      if (match) {
        return {
          type: 'page_link',
          raw: match[0],
          entry_type: match[1].trim(),
          entry_id: match[2].trim(),
          entry_title: match[3].trim(),
          entry_heading: '',
          entry_label: ''
        };
      }
    }
  };
}

function getEmbedLink() {
  return {
    name: 'embed_link',
    level: 'inline',
    tokenizer(src, _tokens) {
      const rule = /^!\[\[([a-zA-Z_-]+):(\d{1,4}) "(.+?(?="]]))"]]/;
      const match = rule.exec(src);

      if (match) {
        return {
          type: 'embed_link',
          raw: match[0],
          entry_type: match[1].trim(),
          entry_id: match[2].trim(),
          entry_title: match[3].trim()
        };
      }
    }
  };
}

function extensions(): any {
  return [getEmbedLink(), getPageLink(), getAdmonition()];
}

export default extensions;
