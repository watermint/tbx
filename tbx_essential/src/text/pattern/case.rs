
pub trait Pattern {
    // Split into alpha-numeric tokens. This tokenizer ignores characters except alpha-numeric.
    // This tokenizer splits token on case change. For example,
    // "Powered by Rust lang version1.65.0." is tokenized to "Powered", "by", "Rust", "lang", "version1", "65", and "0".
    fn tokenize_alpha_num_case(&self) -> Vec<&str>;

    fn to_camel(&self) -> &str;
    fn to_kebab(&self) -> &str;
}

// refs: https://github.com/Anders429/substring/blob/master/src/lib.rs
impl Pattern for str {
    fn tokenize_alpha_num_case(&self) -> Vec<&str> {
        todo!()
        // let mut tokens = vec![];
        // let mut token = String::new();
        // let mut last_lower = false;
        //
        // for c in self.chars() {
        //     if c.is_lowercase() {
        //         token.push(c);
        //         last_lower = true;
        //     } else if c.is_uppercase() {
        //         if last_lower {
        //             if token.len() > 0 {
        //                 let t = token.chars().collect();
        //                 tokens.push(t);
        //                 token.clear();
        //                 last_lower = false;
        //             }
        //         }
        //         token.push(c);
        //     } else if c.is_numeric() {
        //         token.push(c);
        //     } else {
        //         if token.len() > 0 {
        //             let t = token.chars().collect();
        //             tokens.push(t);
        //             token.clear();
        //             last_lower = false;
        //         }
        //     }
        // }
        // if token.len() > 0 {
        //     let t = token.chars().collect();
        //     tokens.push(t);
        //     token.clear();
        //     last_lower = false;
        // }
        //
        // tokens
    }

    fn to_camel(&self) -> &str {
        todo!()
    }

    fn to_kebab(&self) -> &str {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::text::pattern::case::Pattern;

    #[test]
    fn tokenize_alpha_num_case() {
        assert_eq!("Powered by Rust lang version1.65.0.".tokenize_alpha_num_case(),
                   vec!["Powered", "by", "Rust", "lang", "version1", "65", "0"])
    }
}