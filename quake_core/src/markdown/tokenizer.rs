use crate::markdown::md_struct::MdStruct;
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};

pub struct QuakeDown {
    pub pieces: Vec<MdStruct>,
    pub end_newline: bool,
    pub is_in_code_block: bool,
}

impl Default for QuakeDown {
    fn default() -> Self {
        QuakeDown {
            pieces: vec![],
            end_newline: false,
            is_in_code_block: false,
        }
    }
}

impl QuakeDown {
    pub fn from(content: &str) {
        let mut parser_options = Options::empty();
        parser_options.insert(Options::ENABLE_TABLES);
        parser_options.insert(Options::ENABLE_FOOTNOTES);
        parser_options.insert(Options::ENABLE_STRIKETHROUGH);
        parser_options.insert(Options::ENABLE_TASKLISTS);
        parser_options.insert(Options::ENABLE_SMART_PUNCTUATION);
        let mut parser = Parser::new_ext(&content, parser_options);
        let mut down = QuakeDown::default();
        down.transform(&mut parser);
    }

    pub fn transform(&mut self, parser: &mut Parser) {
        while let Some(event) = parser.into_iter().next() {
            match event {
                Event::Start(tag) => {
                    match tag {
                        Tag::Paragraph => {}
                        Tag::Heading(_) => {}
                        Tag::BlockQuote => {}
                        Tag::CodeBlock(info) => match info {
                            CodeBlockKind::Indented => {
                                self.is_in_code_block = true;
                            }
                            CodeBlockKind::Fenced(_info) => {
                                self.is_in_code_block = true;
                            }
                        },
                        Tag::List(_) => {}
                        Tag::Item => {}
                        Tag::FootnoteDefinition(_) => {}
                        Tag::Table(_) => {}
                        Tag::TableHead => {}
                        Tag::TableRow => {}
                        Tag::TableCell => {}
                        Tag::Emphasis => {}
                        Tag::Strong => {}
                        Tag::Strikethrough => {}
                        Tag::Link(_, _, _) => {}
                        Tag::Image(_, _, _) => {}
                    };
                }
                Event::End(tag) => {
                    // something here
                    match tag {
                        Tag::Paragraph => {}
                        Tag::Heading(_) => {}
                        Tag::BlockQuote => {}
                        Tag::CodeBlock(_) => {}
                        Tag::List(_) => {}
                        Tag::Item => {}
                        Tag::FootnoteDefinition(_) => {}
                        Tag::Table(_) => {}
                        Tag::TableHead => {}
                        Tag::TableRow => {}
                        Tag::TableCell => {}
                        Tag::Emphasis => {}
                        Tag::Strong => {}
                        Tag::Strikethrough => {}
                        Tag::Link(_, _, _) => {}
                        Tag::Image(_, _, _) => {}
                    }
                }
                Event::Text(_) => {}
                Event::Code(_) => {}
                Event::Html(_) => {}
                Event::FootnoteReference(_) => {}
                Event::SoftBreak => {}
                Event::HardBreak => {}
                Event::Rule => {}
                Event::TaskListMarker(_) => {}
            }
        }
    }
}
