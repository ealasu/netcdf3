/// Maximum size (number of bytes) allowed for the NetCDF names.
///
pub const NC_MAX_NAME_SIZE: usize = 256;

/// Checks that `name` follows the NetCDF-3 naming convention.
///
/// # Examples
///
/// ```
/// use netcdf3::{is_valid_name};
///
/// assert_eq!(true,    is_valid_name("title"));
/// assert_eq!(true,    is_valid_name("standard_name"));
/// assert_eq!(true,    is_valid_name("_FillValue"));
/// assert_eq!(true,    is_valid_name("café"));  // the UTF-8 encoded characters are supported
/// assert_eq!(true,    is_valid_name("A"));
///
/// assert_eq!(false,   is_valid_name(""));
/// assert_eq!(false,   is_valid_name("!invalid_name"));
/// ```
pub fn is_valid_name(name: &str) -> bool {
    // check the first character
    match name.chars().nth(0) {
        None => {
            // then the name string is empty
            return false;
        }
        Some(c) => {
            if c.is_ascii() {
                if !(c.is_alphanumeric() || c == '_') {
                    return false;
                }
            }
        }
    }
    if name.as_bytes().len() > NC_MAX_NAME_SIZE {
        return false;
    }
    for c in name.chars().skip(1) {
        if !(c.is_alphanumeric()) {
            if c.is_ascii() {
                if !(is_special_1(c) || is_special_2(c)) {
                    return false;
                }
            }
        }
    }
    return true;
}

/// Returns `true` if the `char` is a NetCDF-3 special1 characters.
///
/// ``` text
/// special1     = '_''.''@''+''-'
/// ```
fn is_special_1(chr: char) -> bool {
    return chr == '_' || chr == '.' || chr == '@' || chr == '+' || chr == '-';
}

/// Returns `true` if the `char` is a NetCDF-3 special2 characters.
///
/// ``` text
/// special2     = ' ' | '!' | '"' | '#' | '$' | '%' | '&' | '\'' |
/// '(' | ')' | '*' | ','  | ':' | ';' | '<' | '='  |
/// '>' | '?' | '[' | '\\' | ']' | '^' | '`' | '{'  |
/// '|' | '}' | '~'
/// ```
fn is_special_2(chr: char) -> bool {
    return chr == ' '
        || chr == '!'
        || chr == '"'
        || chr == '#'
        || chr == '$'
        || chr == '%'
        || chr == '&'
        || chr == '\''
        || chr == '('
        || chr == ')'
        || chr == '*'
        || chr == ','
        || chr == ':'
        || chr == ';'
        || chr == '<'
        || chr == '='
        || chr == '>'
        || chr == '?'
        || chr == '['
        || chr == '\\'
        || chr == ']'
        || chr == '^'
        || chr == '`'
        || chr == '{'
        || chr == '|'
        || chr == '}'
        || chr == '~';
}

#[cfg(test)]
mod tests {

    use super::{is_special_1, is_special_2, is_valid_name, NC_MAX_NAME_SIZE};

    #[test]
    fn test_some_valid_name_strings() {
        let valid_name_strings: &'static [&str] = &["f", "foo", "_foo", "àfoo", "éfoo", "èfoo", "ëfoo", "€foo"];

        for name in valid_name_strings {
            assert_eq!(true, is_valid_name(name),
                "The valid NetCDF-3 name_string '{}' has been checked as invalid.", name
            )
        }
    }

    #[test]
    fn test_some_invalid_name_strings() {
        let invalid_name_strings: &'static [&str] = &[
            "", ".foo", "@foo", "+foo", "-foo", " foo", " foo", "!foo", r#""foo"#, "#foo", "$foo", "%foo", "&foo", r#"'foo"#, "(foo",
            ")foo", "*foo", ",foo", ":foo", ";foo", "<foo", "=foo", ">foo", "?foo", "[foo", r#"\foo"#, "]foo", "^foo", "`foo", "{foo",
            "|foo", "}foo", "~foo",
        ];

        for name in invalid_name_strings {
            assert_eq!(false, is_valid_name(name),
                "The invalid NetCDF-3 name_string '{}' has been checked as valid.", name
            )
        }
    }

    #[test]
    fn test_is_special_1() {
        // test all special 1 characters
        assert_eq!(true,            is_special_1('_'));
        assert_eq!(true,            is_special_1('.'));
        assert_eq!(true,            is_special_1('@'));
        assert_eq!(true,            is_special_1('+'));
        assert_eq!(true,            is_special_1('-'));

        // test some non-special 1 characters
        assert_eq!(false,           is_special_1('A'));
        assert_eq!(false,           is_special_1('Z'));
        assert_eq!(false,           is_special_1('a'));
        assert_eq!(false,           is_special_1('z'));
        assert_eq!(false,           is_special_1('0'));
        assert_eq!(false,           is_special_1('9'));
        assert_eq!(false,           is_special_1('/'));
        assert_eq!(false,           is_special_1('!'));
        assert_eq!(false,           is_special_1(' '));
    }

    #[test]
    fn test_is_special_2() {
        // test all special 2 characters
        assert_eq!(true,            is_special_2(' '));
        assert_eq!(true,            is_special_2('!'));
        assert_eq!(true,            is_special_2('"'));
        assert_eq!(true,            is_special_2('#'));
        assert_eq!(true,            is_special_2('$'));
        assert_eq!(true,            is_special_2('%'));
        assert_eq!(true,            is_special_2('&'));
        assert_eq!(true,            is_special_2('\''));
        assert_eq!(true,            is_special_2('('));
        assert_eq!(true,            is_special_2(')'));
        assert_eq!(true,            is_special_2('*'));
        assert_eq!(true,            is_special_2(','));
        assert_eq!(true,            is_special_2(':'));
        assert_eq!(true,            is_special_2(';'));
        assert_eq!(true,            is_special_2('<'));
        assert_eq!(true,            is_special_2('='));
        assert_eq!(true,            is_special_2('>'));
        assert_eq!(true,            is_special_2('?'));
        assert_eq!(true,            is_special_2('['));
        assert_eq!(true,            is_special_2('\\'));
        assert_eq!(true,            is_special_2(']'));
        assert_eq!(true,            is_special_2('^'));
        assert_eq!(true,            is_special_2('`'));
        assert_eq!(true,            is_special_2('{'));
        assert_eq!(true,            is_special_2('|'));
        assert_eq!(true,            is_special_2('}'));
        assert_eq!(true,            is_special_2('~'));

        // test some non-special 2 characters
        assert_eq!(false,           is_special_2('A'));
        assert_eq!(false,           is_special_2('Z'));
        assert_eq!(false,           is_special_2('a'));
        assert_eq!(false,           is_special_2('z'));
        assert_eq!(false,           is_special_2('0'));
        assert_eq!(false,           is_special_2('9'));
        assert_eq!(false,           is_special_2('/'));
        assert_eq!(false,           is_special_2('_'));
    }

    #[test]
    fn test_max_name_size() {
        let valid_ascii_name: String = "a".chars().cycle().take(NC_MAX_NAME_SIZE).collect();
        assert_eq!(true,                    is_valid_name(&valid_ascii_name));
        
        let invalid_ascii_name: String = valid_ascii_name + "a";
        assert_eq!(false,                   is_valid_name(&invalid_ascii_name));

        let valid_utf8_name: String = "é".chars().cycle().take(NC_MAX_NAME_SIZE / 2).collect();
        assert_eq!(NC_MAX_NAME_SIZE,        valid_utf8_name.len());
        assert_eq!(true,                    is_valid_name(&valid_utf8_name));

        let invalid_utf8_name: String = valid_utf8_name + "a";
        assert_eq!(false,                   is_valid_name(&invalid_utf8_name));
    }
}
