use regex::Split as RegexSplit;

#[derive(Debug)]
pub struct Split<'r, 't> {
    s: RegexSplit<'r, 't>,
}

impl<'r, 't> Split<'r, 't> {
    pub fn new(s: RegexSplit<'r, 't>) -> Self { Self { s } }
}

impl<'r, 't> Iterator for Split<'r, 't> {
    type Item = &'t str;

    fn next(&mut self) -> Option<Self::Item> {
        self.s.next()
    }
}