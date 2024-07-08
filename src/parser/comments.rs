use crate::util::Span;
use ecow::EcoString;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct CommentModule {
    pub mod_comments: Vec<Span>,
    pub doc_comments: Vec<Span>,
    pub multi_comments: Vec<Span>,
    pub single_comments: Vec<Span>,
    pub empty_lines: Vec<u32>,
    pub new_lines: Vec<u32>,
}

impl CommentModule {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_within_comment(&self, index: u32) -> bool {
        let cmp = |span: &Span| {
            if index < span.start {
                Ordering::Less
            } else if span.end < index {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        };

        self.single_comments.binary_search_by(cmp).is_ok()
            || self.multi_comments.binary_search_by(cmp).is_ok()
            || self.doc_comments.binary_search_by(cmp).is_ok()
            || self.mod_comments.binary_search_by(cmp).is_ok()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Comment<'a> {
    pub content: &'a str,
    pub start: u32,
}

impl<'a> From<(&'a EcoString, &Span)> for Comment<'a> {
    fn from(value: (&'a EcoString, &Span)) -> Self {
        Self::from((value.0.as_str(), value.1))
    }
}

impl<'a> From<(&'a str, &Span)> for Comment<'a> {
    fn from(source: (&'a str, &Span)) -> Comment<'a> {
        let start = source.1.start;
        let end = source.1.end;
        let ustart = start as usize;
        let uend = end as usize;

        Comment {
            content: source.0.get(ustart..uend).expect("Span::from((&'a str, &Span)) -> Comment<'a>"),
            start,
        }
    }
}
