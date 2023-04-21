use crate::FuzzyOptions;
use lazy_static::lazy_static;
use regex::Regex;
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

lazy_static! {
    static ref WHITESPACE_REGEX: Regex = Regex::new(r"s+").unwrap();
    static ref NON_WORD_REGEX: Regex =
        Regex::new(r#"^[`~!@#$%^&*()\-=_+{}[\]\|\\;':",./<>?]+$"#).unwrap();
}

#[allow(dead_code)]
pub struct Normalized {
    original: String,
    normal: Vec<String>,
    map: Vec<usize>,
}

pub fn normalize<T>(string: String, options: &FuzzyOptions<T>) -> Normalized {
    let str_len = string.len();

    let lower = if options.ignore_case {
        string.to_lowercase()
    } else {
        string.clone()
    };

    let mut normal = vec![];
    let mut map = vec![];
    let mut last_whitespace = true;
    let mut length = 0;

    let grapheme_list = if options.use_separated_unicode {
        lower
            .nfkd()
            .collect::<String>()
            .split("")
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    } else {
        lower.graphemes(false).map(ToString::to_string).collect()
    };

    for grapheme in grapheme_list {
        length += grapheme.len();

        if options.normalize_whitespace && WHITESPACE_REGEX.is_match(&grapheme) {
            if !last_whitespace {
                normal.push(" ".into());
                map.push(length);
                last_whitespace = true;
            }
        } else if !(options.ignore_symbols && NON_WORD_REGEX.is_match(&grapheme)) {
            normal.push(grapheme);
            map.push(length);
            last_whitespace = false;
        }
    }

    map.push(str_len);

    while {
        if let Some(last) = normal.last() {
            last == " "
        } else {
            false
        }
    } {
        normal.pop();
        map.pop();
    }

    Normalized {
        original: string,
        normal,
        map,
    }
}
