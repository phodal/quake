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

use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;

lazy_static! {
    static ref NOTE_LINK_RE: Regex =
        Regex::new(r"^(?P<file>[^#|]+)??(#(?P<section>.+?))??(\|(?P<label>.+?))??$").unwrap();
}

#[derive(Debug, Clone, PartialEq)]
/// NoteReference represents the structure of a `[[note]]` or `![[embed]]` reference.
pub struct NoteReference<'a> {
    /// The file (note name or partial path) being referenced.
    /// This will be None in the case that the reference is to a section within the same document
    pub file: Option<&'a str>,
    /// If specific, a specific section/heading being referenced.
    pub section: Option<&'a str>,
    /// If specific, the custom label/text which was specified.
    pub label: Option<&'a str>,
}

#[derive(PartialEq)]
/// RefParserState enumerates all the possible parsing states [RefParser] may enter.
pub enum RefParserState {
    NoState,
    ExpectSecondOpenBracket,
    ExpectRefText,
    ExpectRefTextOrCloseBracket,
    ExpectFinalCloseBracket,
    Resetting,
}

/// RefType indicates whether a note reference is a link (`[[note]]`) or embed (`![[embed]]`).
pub enum RefType {
    Link,
    Embed,
}

/// RefParser holds state which is used to parse Obsidian WikiLinks (`[[note]]`, `![[embed]]`).
pub struct RefParser {
    pub state: RefParserState,
    pub ref_type: Option<RefType>,
    // References sometimes come in through multiple events. One example of this is when notes
    // start with an underscore (_), presumably because this is also the literal which starts
    // italic and bold text.
    //
    // ref_text concatenates the values from these partial events so that there's a fully-formed
    // string to work with by the time the final `]]` is encountered.
    pub ref_text: String,
}

impl RefParser {
    pub fn new() -> RefParser {
        RefParser {
            state: RefParserState::NoState,
            ref_type: None,
            ref_text: String::new(),
        }
    }

    pub fn transition(&mut self, new_state: RefParserState) {
        self.state = new_state;
    }

    pub fn reset(&mut self) {
        self.state = RefParserState::NoState;
        self.ref_type = None;
        self.ref_text.clear();
    }
}

impl<'a> NoteReference<'a> {
    pub fn from_str(text: &str) -> NoteReference {
        let captures = NOTE_LINK_RE
            .captures(text)
            .expect("note link regex didn't match - bad input?");
        let file = captures.name("file").map(|v| v.as_str());
        let label = captures.name("label").map(|v| v.as_str());
        let section = captures.name("section").map(|v| v.as_str());

        NoteReference {
            file,
            section,
            label,
        }
    }

    pub fn display(&self) -> String {
        format!("{}", self)
    }
}

impl<'a> fmt::Display for NoteReference<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label =
            self.label
                .map(|v| v.to_string())
                .unwrap_or_else(|| match (self.file, self.section) {
                    (Some(file), Some(section)) => format!("{} > {}", file, section),
                    (Some(file), None) => file.to_string(),
                    (None, Some(section)) => section.to_string(),

                    _ => panic!("Reference exists without file or section!"),
                });
        write!(f, "{}", label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_note_refs_from_strings() {
        assert_eq!(
            NoteReference::from_str("Just a note"),
            NoteReference {
                file: Some("Just a note"),
                label: None,
                section: None,
            }
        );
        assert_eq!(
            NoteReference::from_str("A note?"),
            NoteReference {
                file: Some("A note?"),
                label: None,
                section: None,
            }
        );
        assert_eq!(
            NoteReference::from_str("Note#with heading"),
            NoteReference {
                file: Some("Note"),
                label: None,
                section: Some("with heading"),
            }
        );
        assert_eq!(
            NoteReference::from_str("Note#Heading|Label"),
            NoteReference {
                file: Some("Note"),
                label: Some("Label"),
                section: Some("Heading"),
            }
        );
        assert_eq!(
            NoteReference::from_str("#Heading|Label"),
            NoteReference {
                file: None,
                label: Some("Label"),
                section: Some("Heading"),
            }
        );
    }

    #[test]
    fn test_display_of_note_refs() {
        assert_eq!(
            "Note",
            NoteReference {
                file: Some("Note"),
                label: None,
                section: None,
            }
            .display()
        );
        assert_eq!(
            "Note > Heading",
            NoteReference {
                file: Some("Note"),
                label: None,
                section: Some("Heading"),
            }
            .display()
        );
        assert_eq!(
            "Heading",
            NoteReference {
                file: None,
                label: None,
                section: Some("Heading"),
            }
            .display()
        );
        assert_eq!(
            "Label",
            NoteReference {
                file: Some("Note"),
                label: Some("Label"),
                section: Some("Heading"),
            }
            .display()
        );
        assert_eq!(
            "Label",
            NoteReference {
                file: None,
                label: Some("Label"),
                section: Some("Heading"),
            }
            .display()
        );
    }
}
