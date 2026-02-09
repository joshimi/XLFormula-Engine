use crate::types::{self, XlNum};
use std::{fmt::Debug, str::FromStr};

pub fn calculate_concat_operator(str1: &str, str2: &str) -> String {
    str1.to_owned() + str2
}

fn calculate_string_operation_rhs<N>(
    l: &str,
    rhs: types::Value<N>,
    f: fn(str1: &str, str2: &str) -> String,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match rhs {
        types::Value::Boolean(_) | types::Value::Error(_) => rhs,
        types::Value::Number(r) => types::Value::Text(f(l, &r.to_string())),
        types::Value::Text(r) => types::Value::Text(f(l, &r)),
        types::Value::Iterator(_) | types::Value::Date(_) => {
            types::Value::Error(types::Error::Value)
        }
        types::Value::Blank => types::Value::Text(f(l, "")),
    }
}

pub fn calculate_string_operator<N>(
    lhs: types::Value<N>,
    rhs: types::Value<N>,
    f: fn(str1: &str, str2: &str) -> String,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match lhs {
        types::Value::Boolean(_) | types::Value::Error(_) => lhs,
        types::Value::Number(l) => calculate_string_operation_rhs(&l.to_string(), rhs, f),
        types::Value::Text(l) => calculate_string_operation_rhs(&l, rhs, f),
        types::Value::Iterator(_) | types::Value::Date(_) => {
            types::Value::Error(types::Error::Value)
        }
        types::Value::Blank => calculate_string_operation_rhs("", rhs, f),
    }
}

pub fn compare_strings<N>(
    string1: String,
    string2: String,
    f: impl Fn(String, String) -> bool,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    types::Value::Boolean(f(string1, string2).into())
}

/// Result of coercing a formula value to string for FIND/SEARCH (Excel semantics).
#[derive(Debug, Clone)]
pub enum CoerceForFind<N>
where
    N: XlNum,
{
    /// Value was coerced to string; use for FIND/SEARCH.
    Coerced(String),
    /// Propagate this value as the formula result (e.g. existing error or #VALUE! for Date/Iterator).
    Propagate(types::Value<N>),
}

/// Coerce a formula value to string for FIND/SEARCH (Excel semantics).
pub fn value_to_string_for_find<N>(v: &types::Value<N>) -> CoerceForFind<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match v {
        types::Value::Error(e) => CoerceForFind::Propagate(types::Value::Error(*e)),
        types::Value::Number(n) => CoerceForFind::Coerced(n.to_string()),
        types::Value::Text(s) => CoerceForFind::Coerced(s.clone()),
        types::Value::Boolean(b) => CoerceForFind::Coerced(b.to_string()),
        types::Value::Blank => CoerceForFind::Coerced(String::new()),
        types::Value::Date(_) | types::Value::Iterator(_) => {
            CoerceForFind::Propagate(types::Value::Error(types::Error::Value))
        }
    }
}

/// FIND: case-sensitive, no wildcards. Returns 1-based character position or None.
pub fn find_position_case_sensitive(
    find_text: &str,
    within_text: &str,
    start_num_1based: i64,
) -> Option<i64> {
    if start_num_1based < 1 {
        return None;
    }
    let start = (start_num_1based - 1) as usize;
    // Character count (not byte len) for Excel 1-based character position semantics and UTF-8.
    let char_count = within_text.chars().count();
    if start >= char_count {
        return None;
    }
    if find_text.is_empty() {
        return Some(start_num_1based);
    }
    let rest: String = within_text.chars().skip(start).collect();
    let byte_offset = rest.find(find_text)?;
    let chars_before = rest[..byte_offset].chars().count();
    let one_based = (start + chars_before + 1) as i64;
    Some(one_based)
}

/// SEARCH: case-insensitive with wildcards ? * and ~ escape. Returns 1-based character position or None.
pub fn search_position_with_wildcards(
    find_text: &str,
    within_text: &str,
    start_num_1based: i64,
) -> Option<i64> {
    if start_num_1based < 1 {
        return None;
    }
    let start = (start_num_1based - 1) as usize;
    // Character count (not byte len) for Excel 1-based character position semantics and UTF-8.
    let char_count = within_text.chars().count();
    if start >= char_count {
        return None;
    }
    if find_text.is_empty() {
        return Some(start_num_1based);
    }
    let rest: String = within_text.chars().skip(start).collect();
    let rest_lower: String = rest.to_lowercase();
    let pattern_orig: Vec<char> = find_text.chars().collect();
    let pattern_lower: Vec<char> = find_text.to_lowercase().chars().collect();
    let rest_orig: Vec<char> = rest.chars().collect();
    let rest_chars: Vec<char> = rest_lower.chars().collect();
    let mut best: Option<usize> = None;
    for i in 0..=rest_chars.len() {
        if let Some(offset) = match_pattern(
            &pattern_lower,
            &pattern_orig,
            &rest_chars,
            &rest_orig,
            i,
            0,
            i,
        ) {
            let pos = start + offset + 1;
            best = Some(match best {
                None => pos,
                Some(p) => p.min(pos),
            });
        }
    }
    best.map(|p| p as i64)
}

/// Match pattern (with ? * ~) against text from position. Returns Some(offset) where offset
/// is the 0-based position in text where the matched substring starts (for leading *, this
/// is where the remainder matches, not where * started).
fn match_pattern(
    pattern_lower: &[char],
    pattern_orig: &[char],
    text_lower: &[char],
    text_orig: &[char],
    text_pos: usize,
    pattern_pos: usize,
    match_start: usize,
) -> Option<usize> {
    if pattern_pos >= pattern_lower.len() {
        return Some(match_start);
    }
    let mut p = pattern_pos;
    let mut t = text_pos;
    while p < pattern_lower.len() {
        if t > text_lower.len() {
            return None;
        }
        if p + 1 < pattern_orig.len() && pattern_orig[p] == '~' {
            let next = pattern_orig[p + 1];
            if next == '?' || next == '*' || next == '~' {
                if t >= text_orig.len() || text_orig[t] != next {
                    return None;
                }
                p += 2;
                t += 1;
                continue;
            }
        }
        match pattern_lower[p] {
            '?' => {
                if t >= text_lower.len() {
                    return None;
                }
                p += 1;
                t += 1;
            }
            '*' => {
                let rest_pat_lower: Vec<char> = pattern_lower[p + 1..].to_vec();
                let rest_pat_orig: Vec<char> = pattern_orig[p + 1..].to_vec();
                for skip in 0..=(text_lower.len() - t) {
                    if let Some(off) = match_pattern(
                        &rest_pat_lower,
                        &rest_pat_orig,
                        text_lower,
                        text_orig,
                        t + skip,
                        0,
                        t + skip,
                    ) {
                        return Some(off);
                    }
                }
                return None;
            }
            c => {
                if t >= text_lower.len() || text_lower[t] != c {
                    return None;
                }
                p += 1;
                t += 1;
            }
        }
    }
    Some(match_start)
}
