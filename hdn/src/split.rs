#![forbid(unsafe_code)]

#[derive(Debug)]
pub struct SplitString<'life> {
    remainder: Option<&'life str>,
    delimiter: &'life str,
    prev: usize,
    extra_str: String,
    extra_cap: usize,
}

impl<'a> SplitString<'a> {
    pub fn new(input: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: (Some(input)),
            delimiter: (delimiter),
            prev: (0),
            extra_str: ((*input).to_string()),
            extra_cap: (0),
        }
    }
}

impl<'a> Iterator for SplitString<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.delimiter.is_empty() {
            let size = self.remainder.unwrap().len();
            if self.prev >= size {
                return None;
            }

            let sz = self
                .remainder
                .unwrap()
                .chars()
                .nth(self.extra_cap)
                .unwrap()
                .len_utf8()
                + self.prev;
            let ans = &self.remainder.unwrap().chars().as_str()[self.prev..sz];
            self.prev = sz;
            self.extra_cap += 1;
            return Some(ans);
        }
        let tmp = self.extra_str.find(self.delimiter);
        if tmp.is_none() {
            let sz = self.remainder.unwrap().len();
            if self.prev <= sz {
                let ans = &self.remainder.unwrap().chars().as_str()[self.prev..sz];
                self.prev = sz + 1;
                return Some(ans);
            }
            return None;
        }
        let ans = &self.remainder.unwrap().chars().as_str()[self.prev..tmp.unwrap()];
        let size = self.delimiter.len();
        self.prev = tmp.unwrap();
        self.prev += size;
        // self.extra_str.replacen(self.delimiter, "?", 1);
        //self.remainder.unwrap().replacen(ans, "#", 1);
        let mut replaced: String = String::new();
        for _i in 0..size {
            replaced.push('?');
        }
        self.extra_str = self.extra_str.replacen(self.delimiter, &replaced, 1);
        //self.remainder = Some(&self.remainder.unwrap().replacen(self.delimiter, "?", 1));
        Some(ans)
    }
}

pub fn split<'life>(input: &'life str, delimiter: &'life str) -> SplitString<'life> {
    SplitString::new(input, delimiter)
}
