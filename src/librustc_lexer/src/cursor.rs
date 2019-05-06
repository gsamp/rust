use std::str::Chars;

pub(crate) struct Cursor<'a> {
    chars: Chars<'a>,
    #[cfg(debug_assertions)]
    prev: char,
}

pub(crate) const EOF_CHAR: char = '\0';

impl<'a> Cursor<'a> {
    pub(crate) fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            chars: input.chars(),
            #[cfg(debug_assertions)]
            prev: EOF_CHAR,
        }
    }
    /// For debug assertions only
    pub(crate) fn prev(&self) -> char {
        #[cfg(debug_assertions)]
        {
            self.prev
        }

        #[cfg(not(debug_assertions))]
        {
            '\0'
        }
    }
    pub(crate) fn nth_char(&self, n: usize) -> char {
        self.chars().nth(n).unwrap_or(EOF_CHAR)
    }
    pub(crate) fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }
    /// Moves to the next character.
    pub(crate) fn bump(&mut self) -> Option<char> {
        let c = self.chars.next()?;

        #[cfg(debug_assertions)]
        {
            self.prev = c;
        }

        Some(c)
    }
    pub(crate) fn leftover(&self) -> usize {
        self.chars.as_str().len()
    }
    /// Returns an iterator over the remaining characters.
    fn chars(&self) -> Chars<'a> {
        self.chars.clone()
    }
}
