use std::borrow::Cow;
use crate::text::essential::StringEssential;
use crate::text::token::ascii::AsciiTokenizer;

pub trait Naming {
    /// Convert string to CamelCase (upper case).
    /// Non ASCII alphabet or number characters are ignored.
    /// This function returns always upper case for the first char of the first token.
    /// Returns empty string if no ASCII alphabet/number character in given string.
    /// Example: "camel case" -> "CamelCase".
    fn to_ascii_camel_upper<'a>(&self) -> Cow<'a, str>;

    /// Convert string to CamelCase (lower case).
    /// Non ASCII alphabet or number characters are ignored.
    /// This function returns always lower case for the first char of the first token.
    /// Returns empty string if no ASCII alphabet/number character in given string.
    /// Example: "camel case" -> "camelCase".
    fn to_ascii_camel_lower<'a>(&self) -> Cow<'a, str>;

    /// Convert string to KEBAB-CASE (all capital).
    /// Non ASCII alphabet or number characters are ignored.
    /// Returns empty string if no ASCII alphabet/number character in given string.
    /// Example: "Kebab case" -> "KEBAB-CASE".
    fn to_ascii_kebab_capital<'a>(&self) -> Cow<'a, str>;

    /// Convert string to Kebab-Case (upper case).
    /// Non ASCII alphabet or number characters are ignored.
    /// Returns empty string if no ASCII alphabet/number character in given string.
    /// Example: "Kebab case" -> "kebab-case".
    fn to_ascii_kebab_upper<'a>(&self) -> Cow<'a, str>;

    /// Convert string to kebab-case (lower case).
    /// Non ASCII alphabet or number characters are ignored.
    /// Returns empty string if no ASCII alphabet/number character in given string.
    /// Example: "Kebab case" -> "kebab-case".
    fn to_ascii_kebab_lower<'a>(&self) -> Cow<'a, str>;

    /// Convert string to SNAKE_CASE (all capital).
    /// Non ASCII alphabet or number characters are ignored.
    /// Returns empty string if no ASCII alphabet/number character in given string.
    /// Example: "Snake case" -> "SNAKE_CASE".
    fn to_ascii_snake_capital<'a>(&self) -> Cow<'a, str>;

    /// Convert string to Snake_Case (upper case).
    /// Non ASCII alphabet or number characters are ignored.
    /// Returns empty string if no ASCII alphabet/number character in given string.
    /// Example: "Snake case" -> "Snake_Case".
    fn to_ascii_snake_upper<'a>(&self) -> Cow<'a, str>;

    /// Convert string to snake_case (lower case).
    /// Non ASCII alphabet or number characters are ignored.
    /// Returns empty string if no ASCII alphabet/number character in given string.
    /// Example: "Snake case" -> "snake_case".
    fn to_ascii_snake_lower<'a>(&self) -> Cow<'a, str>;
}


impl Naming for str {
    fn to_ascii_camel_upper<'a>(&self) -> Cow<'a, str> {
        Cow::Owned(self.tokenize_ascii_alpha_num_to_first_upper().join(""))
    }

    fn to_ascii_camel_lower<'a>(&self) -> Cow<'a, str> {
        let cu = self.to_ascii_camel_upper();
        match (cu.substring(0, 1), cu.substring_to_end(1)) {
            (Some(f), Some(r)) => Cow::Owned(f.to_string().to_lowercase() + r),
            (Some(f), None) => Cow::Owned(f.to_string().to_lowercase()),
            _ => Cow::Owned("".to_string()),
        }
    }

    fn to_ascii_kebab_capital<'a>(&self) -> Cow<'a, str> {
        Cow::Owned(self.tokenize_ascii_alpha_num_to_capital().join("-"))
    }

    fn to_ascii_kebab_upper<'a>(&self) -> Cow<'a, str> {
        Cow::Owned(self.tokenize_ascii_alpha_num_to_first_upper().join("-"))
    }

    fn to_ascii_kebab_lower<'a>(&self) -> Cow<'a, str> {
        Cow::Owned(self.tokenize_ascii_alpha_num_to_lower().join("-"))
    }

    fn to_ascii_snake_capital<'a>(&self) -> Cow<'a, str> {
        Cow::Owned(self.tokenize_ascii_alpha_num_to_capital().join("_"))
    }

    fn to_ascii_snake_upper<'a>(&self) -> Cow<'a, str> {
        Cow::Owned(self.tokenize_ascii_alpha_num_to_first_upper().join("_"))
    }

    fn to_ascii_snake_lower<'a>(&self) -> Cow<'a, str> {
        Cow::Owned(self.tokenize_ascii_alpha_num_to_lower().join("_"))
    }
}

#[cfg(test)]
mod tests {
    use crate::text::pattern::naming::Naming;

    #[test]
    fn test_to_ascii_camel_upper() {
        assert_eq!("CamelCase", "camel Case".to_ascii_camel_upper());
        assert_eq!("CamelCase", "CAMEL-case".to_ascii_camel_upper());
        assert_eq!("CamelCase", "-CAMEL-CASE-".to_ascii_camel_upper());
        assert_eq!("CamelCase", "=camel=case=".to_ascii_camel_upper());
        assert_eq!("", "*".to_ascii_camel_upper());
        assert_eq!("C", "c".to_ascii_camel_upper());
        assert_eq!("Camel", "  camel".to_ascii_camel_upper());
        assert_eq!("C3P0", " c***3***p***0".to_ascii_camel_upper());
    }

    #[test]
    fn test_to_ascii_camel_lower() {
        assert_eq!("camelCase", "camel Case".to_ascii_camel_lower());
        assert_eq!("camelCase", "CAMEL-case".to_ascii_camel_lower());
        assert_eq!("camelCase", "-CAMEL-CASE-".to_ascii_camel_lower());
        assert_eq!("camelCase", "=camel=case=".to_ascii_camel_lower());
        assert_eq!("", "*".to_ascii_camel_lower());
        assert_eq!("c", "c".to_ascii_camel_lower());
        assert_eq!("camel", "  camel".to_ascii_camel_lower());
        assert_eq!("c3P0", " c***3***p***0".to_ascii_camel_lower());
    }

    #[test]
    fn test_to_ascii_kebab_capital() {
        assert_eq!("KEBAB-CAPITAL", "kebab Capital".to_ascii_kebab_capital());
        assert_eq!("KEBAB-CAPITAL", "KEBAB-capital".to_ascii_kebab_capital());
        assert_eq!("KEBAB-CAPITAL", "-KEBAB-Capital-".to_ascii_kebab_capital());
        assert_eq!("KEBAB-CAPITAL", "=kebab=capital=".to_ascii_kebab_capital());
    }

    #[test]
    fn test_to_ascii_kebab_upper() {
        assert_eq!("Kebab-Upper", "kebab upper".to_ascii_kebab_upper());
        assert_eq!("Kebab-Upper", "KEBAB-upper".to_ascii_kebab_upper());
        assert_eq!("Kebab-Upper", "-KEBAB-Upper-".to_ascii_kebab_upper());
        assert_eq!("Kebab-Upper", "=kebab=upper=".to_ascii_kebab_upper());
    }

    #[test]
    fn test_to_ascii_kebab_lower() {
        assert_eq!("kebab-lower", "kebab lower".to_ascii_kebab_lower());
        assert_eq!("kebab-lower", "KEBAB lower".to_ascii_kebab_lower());
        assert_eq!("kebab-lower", "-KEBAB-Lower-".to_ascii_kebab_lower());
        assert_eq!("kebab-lower", "=kebab=LOWER=".to_ascii_kebab_lower());
    }

    #[test]
    fn test_to_ascii_snake_capital() {
        assert_eq!("SNAKE_CAPITAL", "snake capital".to_ascii_snake_capital());
        assert_eq!("SNAKE_CAPITAL", "SNAKE Capital".to_ascii_snake_capital());
        assert_eq!("SNAKE_CAPITAL", "-Snake-CAPITAL-".to_ascii_snake_capital());
        assert_eq!("SNAKE_CAPITAL", "=snake=Capital=".to_ascii_snake_capital());
    }

    #[test]
    fn test_to_ascii_snake_upper() {
        assert_eq!("Snake_Upper", "snake upper".to_ascii_snake_upper());
        assert_eq!("Snake_Upper", "SNAKE Upper".to_ascii_snake_upper());
        assert_eq!("Snake_Upper", "-Snake-UPPER-".to_ascii_snake_upper());
        assert_eq!("Snake_Upper", "=snake=Upper=".to_ascii_snake_upper());
     }

    #[test]
    fn test_to_ascii_snake_lower() {
        assert_eq!("snake_lower", "snake lower".to_ascii_snake_lower());
        assert_eq!("snake_lower", "SNAKE LOWER".to_ascii_snake_lower());
        assert_eq!("snake_lower", "-Snake-Lower-".to_ascii_snake_lower());
        assert_eq!("snake_lower", "=snake=Lower=".to_ascii_snake_lower());
    }
}