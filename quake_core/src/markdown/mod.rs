use pulldown_cmark::{Event, Options, Parser, Tag};

pub fn jsonify(text: String) {
    let parser = Parser::new_ext(text.as_str(), Options::empty());
    for event in parser {
        match event {
            Event::Start(tag) => {
                start_tag(tag);
            }
            Event::End(tag) => {
                end_tag(tag);
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

fn start_tag(tag: Tag) {
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

fn end_tag(tag: Tag) {
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
