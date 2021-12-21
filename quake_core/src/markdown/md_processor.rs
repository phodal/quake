// Copyright (c) 2020 Nick Groenen
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
use std::error::Error;

use crate::markdown::entry_reference::EntryReference;
use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag};
use pulldown_cmark_to_cmark::cmark_with_options;

use crate::markdown::references::{RefParser, RefParserState, RefType};

pub type MarkdownEvents<'a> = Vec<Event<'a>>;

#[derive(Debug, Default)]
pub struct MdProcessor {}

impl MdProcessor {
    pub fn transform(content: &str) -> Result<String, Box<dyn Error>> {
        let mut down = MdProcessor::default();
        let mut links: Vec<EntryReference> = vec![];
        let events = down.add_custom_syntax(content, &mut links)?;
        let mapping = events.into_iter().map(event_to_owned).collect();

        Ok(events_to_text(mapping))
    }

    pub fn pagelinks(content: &str) -> Result<Vec<EntryReference>, Box<dyn Error>> {
        let mut down = MdProcessor::default();
        let mut links: Vec<EntryReference> = vec![];
        let _ = down.add_custom_syntax(content, &mut links)?;

        Ok(links)
    }

    // based on https://github.com/zoni/obsidian-export/blob/main/src/lib.rs
    fn add_custom_syntax<'a>(
        &mut self,
        content: &'a str,
        links: &mut Vec<EntryReference>,
    ) -> Result<Vec<Event<'a>>, Box<dyn Error>> {
        let mut parser_options = Options::empty();
        parser_options.insert(Options::ENABLE_TABLES);
        parser_options.insert(Options::ENABLE_FOOTNOTES);
        parser_options.insert(Options::ENABLE_STRIKETHROUGH);
        parser_options.insert(Options::ENABLE_TASKLISTS);
        parser_options.insert(Options::ENABLE_SMART_PUNCTUATION);

        let mut events = vec![];
        let mut ref_parser = RefParser::new();
        let mut buffer = Vec::with_capacity(5);

        for event in Parser::new_ext(content, parser_options) {
            if ref_parser.state == RefParserState::Resetting {
                events.append(&mut buffer);
                buffer.clear();
                ref_parser.reset();
            }
            buffer.push(event.clone());

            match ref_parser.state {
                RefParserState::NoState => {
                    match event {
                        Event::Text(CowStr::Borrowed("![")) => {
                            ref_parser.ref_type = Some(RefType::Embed);
                            ref_parser.transition(RefParserState::ExpectSecondOpenBracket);
                        }
                        Event::Text(CowStr::Borrowed("[")) => {
                            ref_parser.ref_type = Some(RefType::Link);
                            ref_parser.transition(RefParserState::ExpectSecondOpenBracket);
                        }
                        _ => {
                            events.push(event);
                            buffer.clear();
                        },
                    };
                }
                RefParserState::ExpectSecondOpenBracket => match event {
                    Event::Text(CowStr::Borrowed("[")) => {
                        ref_parser.transition(RefParserState::ExpectRefText);
                    }
                    _ => {
                        ref_parser.transition(RefParserState::Resetting);
                    }
                },
                RefParserState::ExpectRefText => match event {
                    Event::Text(CowStr::Borrowed("]")) => {
                        ref_parser.transition(RefParserState::Resetting);
                    }
                    Event::Text(text) => {
                        ref_parser.ref_text.push_str(&text);
                        ref_parser.transition(RefParserState::ExpectRefTextOrCloseBracket);
                    }
                    _ => {
                        ref_parser.transition(RefParserState::Resetting);
                    }
                },
                RefParserState::ExpectRefTextOrCloseBracket => match event {
                    Event::Text(CowStr::Borrowed("]")) => {
                        ref_parser.transition(RefParserState::ExpectFinalCloseBracket);
                    }
                    Event::Text(text) => {
                        ref_parser.ref_text.push_str(&text);
                    }
                    _ => {
                        ref_parser.transition(RefParserState::Resetting);
                    }
                },
                RefParserState::ExpectFinalCloseBracket => match event {
                    Event::Text(CowStr::Borrowed("]")) => match ref_parser.ref_type {
                        Some(RefType::Link) => {
                            let reference = EntryReference::from_str(
                                ref_parser.ref_text.clone().as_ref()
                            );
                            links.push(reference.clone());
                            let mut elements = self.make_link_to_file(
                                reference
                            );
                            events.append(&mut elements);
                            buffer.clear();
                            ref_parser.transition(RefParserState::Resetting);
                        }
                        Some(RefType::Embed) => {
                            let mut elements = self.embed_file(
                                ref_parser.ref_text.clone().as_ref()
                            )?;
                            events.append(&mut elements);
                            buffer.clear();
                            ref_parser.transition(RefParserState::Resetting);
                        }
                        None => panic!("In state ExpectFinalCloseBracket but ref_type is None"),
                    },
                    _ => {
                        ref_parser.transition(RefParserState::Resetting);
                    }
                },
                RefParserState::Resetting => panic!("Reached Resetting state, but it should have been handled prior to this match block"),
            }
        }

        Ok(events)
    }

    pub fn make_link_to_file<'c>(&mut self, reference: EntryReference) -> MarkdownEvents<'c> {
        let link = format!("/{:}/{:}", reference.entry_type, reference.entry_id);

        let link_tag = pulldown_cmark::Tag::Link(
            pulldown_cmark::LinkType::Inline,
            CowStr::from(link),
            CowStr::from(""),
        );

        vec![
            Event::Start(link_tag.clone()),
            Event::Text(CowStr::from(reference.display())),
            Event::End(link_tag.clone()),
        ]
    }

    pub fn embed_file<'b>(
        &mut self,
        link_text: &str,
    ) -> Result<MarkdownEvents<'b>, Box<dyn Error>> {
        let note_ref = EntryReference::from_str(link_text);

        Ok(self.make_link_to_file(note_ref))
    }
}

pub fn events_to_text(markdown: Vec<Event>) -> String {
    let mut buffer = String::new();
    cmark_with_options(
        markdown.iter(),
        &mut buffer,
        None,
        pulldown_cmark_to_cmark::Options::default(),
    )
    .expect("formatting to string not expected to fail");

    // buffer.push('\n');
    buffer
}

fn event_to_owned<'a>(event: Event) -> Event<'a> {
    match event {
        Event::Start(tag) => Event::Start(tag_to_owned(tag)),
        Event::End(tag) => Event::End(tag_to_owned(tag)),
        Event::Text(cowstr) => Event::Text(CowStr::from(cowstr.into_string())),
        Event::Code(cowstr) => Event::Code(CowStr::from(cowstr.into_string())),
        Event::Html(cowstr) => Event::Html(CowStr::from(cowstr.into_string())),
        Event::FootnoteReference(cowstr) => {
            Event::FootnoteReference(CowStr::from(cowstr.into_string()))
        }
        Event::SoftBreak => Event::SoftBreak,
        Event::HardBreak => Event::HardBreak,
        Event::Rule => Event::Rule,
        Event::TaskListMarker(checked) => Event::TaskListMarker(checked),
    }
}

fn tag_to_owned<'a>(tag: Tag) -> Tag<'a> {
    match tag {
        Tag::Paragraph => Tag::Paragraph,
        Tag::Heading(level) => Tag::Heading(level),
        Tag::BlockQuote => Tag::BlockQuote,
        Tag::CodeBlock(codeblock_kind) => Tag::CodeBlock(codeblock_kind_to_owned(codeblock_kind)),
        Tag::List(optional) => Tag::List(optional),
        Tag::Item => Tag::Item,
        Tag::FootnoteDefinition(cow_str) => {
            Tag::FootnoteDefinition(CowStr::from(cow_str.into_string()))
        }
        Tag::Table(alignment_vector) => Tag::Table(alignment_vector),
        Tag::TableHead => Tag::TableHead,
        Tag::TableRow => Tag::TableRow,
        Tag::TableCell => Tag::TableCell,
        Tag::Emphasis => Tag::Emphasis,
        Tag::Strong => Tag::Strong,
        Tag::Strikethrough => Tag::Strikethrough,
        Tag::Link(link_type, cow_str1, cow_str2) => Tag::Link(
            link_type,
            CowStr::from(cow_str1.into_string()),
            CowStr::from(cow_str2.into_string()),
        ),
        Tag::Image(link_type, cow_str1, cow_str2) => Tag::Image(
            link_type,
            CowStr::from(cow_str1.into_string()),
            CowStr::from(cow_str2.into_string()),
        ),
    }
}

fn codeblock_kind_to_owned<'a>(codeblock_kind: CodeBlockKind) -> CodeBlockKind<'a> {
    match codeblock_kind {
        CodeBlockKind::Indented => CodeBlockKind::Indented,
        CodeBlockKind::Fenced(cow_str) => {
            CodeBlockKind::Fenced(CowStr::from(cow_str.into_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::markdown::md_processor::MdProcessor;
    use std::fs;
    use std::path::PathBuf;

    #[ignore]
    #[test]
    fn markdown_test() {
        let base = PathBuf::from("_fixtures").join("md");
        let origin = base.join("origin.md");
        let src = fs::read_to_string(origin).unwrap();

        let target = base.join("new.md");
        let expect = fs::read_to_string(target).unwrap();

        let actual = MdProcessor::transform(src.as_str()).unwrap();

        assert_eq!(actual, expect);
    }

    #[test]
    fn br_tag_in_html() {
        let string = MdProcessor::transform("demo `<br />` demo").unwrap();
        assert_eq!("demo `<br />` demo", string);
    }

    #[test]
    fn transform_page_link() {
        let _string = MdProcessor::transform("[[note::SourceCode]]").unwrap();
        // assert_eq!("[note::SourceCode](note::SourceCode)", string);
    }

    #[test]
    fn transform_page_file() {
        let _string = MdProcessor::transform("![[Note:0001#Heading|Label \"file name\"]]").unwrap();
        // assert_eq!("[Label “file name”](Note:0001)", string);
    }

    #[test]
    fn get_entry_refs() {
        let _links = MdProcessor::pagelinks("![[Note:0001#Heading|Label \"file name\"]]").unwrap();
        // assert_eq!(1, links.len());
    }
}
