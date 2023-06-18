use std::{iter::Peekable, str::Chars};

// TODO: implement unit tests for Cursor

pub struct Cursor<'a> {
    input: &'a str,
    iter: Peekable<Chars<'a>>,
    offset: usize,
    mark: usize,
}

impl<'a> From<&'a str> for Cursor<'a> {
    fn from(input: &'a str) -> Self {
        Cursor {
            input,
            iter: input.chars().peekable(),
            offset: 0,
            mark: 0,
        }
    }
}

impl<'a> Cursor<'a> {
    pub fn pop(&mut self) -> Option<char> {
        match self.iter.next() {
            Some(ch) => {
                self.offset += ch.len_utf8();
                Some(ch)
            }
            None => None,
        }
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.iter.peek()
    }

    pub fn pop_while<P>(&mut self, pred: P) -> Option<&str>
    where
        P: Fn(char) -> bool,
    {
        let start = self.offset;
        while self.iter.peek().filter(|ch| pred(**ch)).is_some() {
            self.pop();
        }
        let end = self.offset;
        if end > start {
            Some(&self.input[start..end])
        } else {
            None
        }
    }

    pub fn rest_of_line(&mut self) -> Option<&'a str> {
        let start = self.offset;
        let end = self.input.len();
        if end > start {
            Some(&self.input[start..end])
        } else {
            None
        }
    }

    pub fn mark(&mut self) {
        self.mark = self.offset;
    }

    pub fn has_moved(&mut self) -> bool {
        self.offset > self.mark
    }

    pub fn rewind(&mut self) {
        self.offset = self.mark;
        self.iter = self.input[..].chars().peekable();
    }

    pub fn marked_input(&mut self) -> Option<&'a str> {
        if self.offset > self.mark {
            Some(&self.input[self.mark..self.offset])
        } else {
            None
        }
    }
}
