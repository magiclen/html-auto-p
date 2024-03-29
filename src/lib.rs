/*!
# HTML auto_p

This library provides a function like `wpautop` in Wordpress. It uses a group of regex replaces used to identify text formatted with newlines and replace double line-breaks with HTML paragraph tags.

Someone who familiars with HTML would prefer directly writing plain HTML instead of using an editor like TinyMCE or Gutenberg. However, it takes time to manually add newlines and paragraphs in HTML. Wordpress provides a handy function called `wpautop` which can replace double line-breaks with paragraph elements (`<p>`) and convert remaining line-breaks to `<br>` elements.

The `auto_p` function in this library can be used like `wpautop`.

```rust
use html_auto_p::*;

assert_eq!("<p>Hello world!</p>", auto_p("Hello world!", Options::new()));
assert_eq!("<p>Line 1<br>\nLine 2</p>", auto_p("Line 1\nLine 2", Options::new().br(true)));
assert_eq!("<p>Line 1<br>\nLine 2</p>", auto_p("Line 1<br>\nLine 2", Options::new().br(true)));
assert_eq!("<p>Paragraph 1</p>\n<p>Paragraph 2</p>", auto_p("Paragraph 1\n\nParagraph 2", Options::new()));
assert_eq!("<pre>Line 1<br>\nLine 2</pre>", auto_p("<pre>Line 1<br>\nLine 2</pre>", Options::new().br(true)));
assert_eq!("<pre>Line 1&lt;br&gt;\nLine 2</pre>", auto_p("<pre>Line 1<br>\nLine 2</pre>", Options::new().br(true).esc_pre(true)));
assert_eq!("<pre>Line 1\nLine 2</pre>", auto_p("<pre>\nLine 1\nLine 2\n</pre>", Options::new().remove_useless_newlines_in_pre(true)));
```

## Onig Support (alternative, unstable)

To use the [`onig`](https://crates.io/crates/onig) crate, enable the `onig` feature.

```toml
[dependencies.html-auto-p]
version = "*"
features = ["onig"]
```
*/

#[cfg(feature = "onig")]
extern crate onig as regex;

mod options;

#[cfg(not(feature = "onig"))]
use std::borrow::Cow;
use std::{fmt::Write, str::from_utf8_unchecked};

use once_cell::sync::Lazy;
pub use options::*;
use regex::Regex;
use trim_in_place::TrimInPlace;

macro_rules! all_blocks_tag_names_except_p {
    () => {
        "table|thead|tfoot|caption|col|colgroup|tbody|tr|td|th|div|dl|dd|dt|ul|ol|li|pre|form|map|area|blockquote|address|math|h[1-6]|hr|fieldset|legend|section|article|aside|hgroup|header|footer|nav|figure|figcaption|details|menu|summary"
    }
}

macro_rules! all_blocks_tag_names {
    () => {
        concat!(all_blocks_tag_names_except_p!(), "|p")
    };
}

macro_rules! all_preserved_tag_names {
    () => {
        "textarea|script|style|svg"
    };
}

macro_rules! all_block_and_preserved_tag_names {
    () => {
        concat!(all_blocks_tag_names!(), "|", all_preserved_tag_names!())
    };
}

macro_rules! pattern_all_blocks_except_p {
    () => {
        concat!("(?i:", all_blocks_tag_names_except_p!(), ")")
    };
}

macro_rules! pattern_all_blocks {
    () => {
        concat!("(?i:", all_blocks_tag_names!(), ")")
    };
}

macro_rules! pattern_all_block_and_preserved_tag_names {
    () => {
        concat!("(?i:", all_block_and_preserved_tag_names!(), ")")
    };
}

macro_rules! pattern_attributes {
    () => {
        "(?:\\s+[^<>\\s=]+(?:=(?:|(?:[^'\"])|(?:[^'\"][^\\s<>]*[^'\"])|(?:\"[^\"]*\")|(?:'[^']*'\
         )))?)*\\s*"
    };
}

static RE_PRE_ELEMENT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(concat!("(?i)", "(<pre", pattern_attributes!(), r">)([\s\S]*?)(</pre\s*>)")).unwrap()
});
static RE_TEXTAREA_ELEMENT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(concat!(
        "(?i)",
        "(<textarea",
        pattern_attributes!(),
        r">)([\s\S]*?)(</textarea\s*>)"
    ))
    .unwrap()
});
static RE_SCRIPT_ELEMENT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(concat!("(?i)", "(<script", pattern_attributes!(), r">)([\s\S]*?)(</script\s*>)"))
        .unwrap()
});
static RE_STYLE_ELEMENT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(concat!("(?i)", "(<style", pattern_attributes!(), r">)([\s\S]*?)(</style\s*>)"))
        .unwrap()
});
static RE_SVG_ELEMENT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(concat!("(?i)", "(<svg", pattern_attributes!(), r">)([\s\S]*?)(</svg\s*>)")).unwrap()
});
static RE_BR_ELEMENT: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)<br\s*/?>").unwrap());

static RE_TAG: Lazy<Regex> =
    Lazy::new(|| Regex::new(concat!(r"</?[^\s<]+(", pattern_attributes!(), r")/?>")).unwrap());

static RE_OTHER_NEWLINE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?:\r\n|\r)").unwrap());
#[allow(clippy::trivial_regex)]
static RE_EMPTY_PARAGRAPH: Lazy<Regex> = Lazy::new(|| Regex::new(r"<p></p>").unwrap());

static RE_P_END_TAG_MISSING_START: Lazy<Regex> = Lazy::new(|| {
    Regex::new(concat!(
        "(?i)",
        r"(<",
        pattern_all_blocks_except_p!(),
        pattern_attributes!(),
        r">)(\s*)([^<]+)</p>"
    ))
    .unwrap()
});
static RE_P_START_TAG_MISSING_END: Lazy<Regex> = Lazy::new(|| {
    Regex::new(concat!("(?i)", r"<p>([^<]+)(\s*)(</", pattern_all_blocks_except_p!(), r"\s*>)"))
        .unwrap()
});

static RE_LI_IN_PARAGRAPH: Lazy<Regex> = Lazy::new(|| {
    Regex::new(concat!("(?i)", r"<p>(<li", pattern_attributes!(), r">[\s\S]*)</p>")).unwrap()
});

static RE_BLOCK_AND_PRESERVED_TAG_AFTER_P_START_TAG: Lazy<Regex> = Lazy::new(|| {
    Regex::new(concat!(
        "(?i)",
        r"<p>(</?",
        pattern_all_block_and_preserved_tag_names!(),
        pattern_attributes!(),
        r">)"
    ))
    .unwrap()
});
static RE_BLOCK_AND_PRESERVED_TAG_BEFORE_P_END_TAG: Lazy<Regex> = Lazy::new(|| {
    Regex::new(concat!(
        "(?i)",
        r"(</?",
        pattern_all_block_and_preserved_tag_names!(),
        pattern_attributes!(),
        r">)</p>"
    ))
    .unwrap()
});

static RE_BR_ELEMENT_AFTER_BLOCK_TAG: Lazy<Regex> = Lazy::new(|| {
    Regex::new(concat!("(?i)", r"(</?", pattern_all_blocks!(), pattern_attributes!(), r">)<br>\n"))
        .unwrap()
});
static RE_BR_ELEMENT_BEFORE_BLOCK_TAG: Lazy<Regex> = Lazy::new(|| {
    Regex::new(concat!("(?i)", r"<br>\n(</?", pattern_all_blocks!(), pattern_attributes!(), r">)"))
        .unwrap()
});

/// A group of regex replaces used to identify text formatted with newlines and replace double line-breaks with HTML paragraph tags.
///
/// The original algorithm can be found in [wp-includes/formatting.php](https://github.com/WordPress/WordPress/blob/101d00601e8d00041218e31194c6f5e0dc4940aa/wp-includes/formatting.php#L442)
///
/// This function does not 100% work like `wpautop` does.
pub fn auto_p<S: Into<String>>(pee: S, options: Options) -> String {
    let mut pee = pee.into();

    pee.trim_in_place();

    if pee.is_empty() {
        return pee;
    }

    let mut pre_inner_html_buffer: Vec<(String, usize, usize)> = Vec::new();
    let mut script_inner_html_buffer: Vec<(String, usize, usize)> = Vec::new();
    let mut style_inner_html_buffer: Vec<(String, usize, usize)> = Vec::new();
    let mut textarea_inner_html_buffer: Vec<(String, usize, usize)> = Vec::new();
    let mut svg_inner_html_buffer: Vec<(String, usize, usize)> = Vec::new();

    // The inner HTML in `<pre>`, `<textarea>`, `<script>`, `<style>` and `<svg>` elements should not get `auto_p`ed, so temporarily copy it out, and fill the inner HTML with `'0'`
    {
        fn reserve(pee: &mut String, regex: &Regex, buffer: &mut Vec<(String, usize, usize)>) {
            for captures in regex.captures_iter(pee) {
                let (s, start, end) = get(&captures, 2);

                buffer.push((String::from(s), start, end));
            }

            let bytes = unsafe { pee.as_mut_vec() };

            for (_, start, end) in buffer.iter() {
                for e in bytes[*start..*end].iter_mut() {
                    *e = b'0';
                }
            }
        }

        reserve(&mut pee, &RE_PRE_ELEMENT, &mut pre_inner_html_buffer);
        reserve(&mut pee, &RE_TEXTAREA_ELEMENT, &mut textarea_inner_html_buffer);
        reserve(&mut pee, &RE_SCRIPT_ELEMENT, &mut script_inner_html_buffer);
        reserve(&mut pee, &RE_STYLE_ELEMENT, &mut style_inner_html_buffer);
        reserve(&mut pee, &RE_SVG_ELEMENT, &mut svg_inner_html_buffer);
    }

    // Standardize newline characters to `"\n"`.
    let mut pee = replace_all(&RE_OTHER_NEWLINE, pee, "\n");

    // Find newlines in all tags and replace them to `'\r'`s.
    {
        let mut newlines_in_tags: Vec<usize> = Vec::new();

        for captures in RE_TAG.captures_iter(&pee) {
            let (s, start, _) = get(&captures, 1);

            for (i, e) in s.bytes().enumerate() {
                if e == b'\n' {
                    newlines_in_tags.push(i + start);
                }
            }
        }

        let bytes = unsafe { pee.as_mut_vec() };

        for newline_index in newlines_in_tags {
            bytes[newline_index] = b'\r';
        }
    }

    // Split up the contents into an array of strings, separated by at-least-two line breaks.
    let pees = pee.split("\n\n");

    // Reset `pee` prior to rebuilding.
    let mut pee = String::with_capacity(pee.len());

    // Rebuild the content as a string, wrapping every bit with a `<p>`.
    for tinkle in pees {
        pee.write_fmt(format_args!("<p>{}</p>\n", tinkle.trim())).unwrap();
    }

    // Remove empty paragraphs.
    let mut pee = replace_all(&RE_EMPTY_PARAGRAPH, pee, "");

    pee.trim_matches_in_place('\n');

    // Add a starting `<p>` inside a block element if missing.
    let pee = replace_all(&RE_P_END_TAG_MISSING_START, pee, "$1$2<p>$3</p>");

    // Add a closing `<p>` inside a block element if missing.
    let pee = replace_all(&RE_P_START_TAG_MISSING_END, pee, "<p>$1</p>$2$3");

    // In some cases `<li>` may get wrapped in `<p>`, fix them.
    let pee = replace_all(&RE_LI_IN_PARAGRAPH, pee, "$1");

    // If an opening or closing block element tag is preceded by an opening `<p>` tag, remove the `<p>` tag.
    let pee = replace_all(&RE_BLOCK_AND_PRESERVED_TAG_AFTER_P_START_TAG, pee, "$1");

    // If an opening or closing block element tag is followed by a closing `</p>` tag, remove the `</p>` tag.
    let pee = replace_all(&RE_BLOCK_AND_PRESERVED_TAG_BEFORE_P_END_TAG, pee, "$1");

    // Optionally insert line breaks.
    #[allow(clippy::let_and_return)]
    let mut pee = if options.br {
        // Normalize `<br>`
        let mut pee = replace_all(&RE_BR_ELEMENT, pee, "<br>");

        // Replace any new line characters that aren't preceded by a `<br>` with a `<br>`.
        let mut v = Vec::new();

        {
            let bytes = pee.as_bytes();

            let mut p = bytes.len();

            loop {
                if p == 0 {
                    break;
                }

                p -= 1;

                let e = bytes[p];

                if e == b'\n' {
                    let mut pp = p;

                    loop {
                        if pp == 0 {
                            break;
                        }

                        pp -= 1;

                        let e = bytes[pp];

                        if !e.is_ascii_whitespace() {
                            break;
                        }
                    }

                    if pp < 3 || &bytes[(pp - 3)..=pp] != b"<br>" {
                        v.push((pp + 1)..p);
                    }

                    p = pp;
                }
            }
        }

        for range in v.into_iter() {
            pee.replace_range(range, "<br>");
        }

        // If a `<br>` tag is after an opening or closing block tag, remove it.
        let pee = replace_all(&RE_BR_ELEMENT_AFTER_BLOCK_TAG, pee, "$1\n");

        // If a `<br>` tag is before an opening or closing block tags, remove it.
        let pee = replace_all(&RE_BR_ELEMENT_BEFORE_BLOCK_TAG, pee, "\n$1");

        pee
    } else {
        pee
    };

    // Recover the inner HTML that have been filled with `'0'` before.
    {
        fn recover(pee: &mut String, regex: &Regex, buffer: &[(String, usize, usize)]) {
            let mut v = Vec::with_capacity(buffer.len());

            for (captures, inner_html) in regex.captures_iter(pee).zip(buffer.iter()) {
                let (_, start, end) = get(&captures, 2);

                v.push((start..end, inner_html.0.as_str()));
            }

            for (range, inner_html) in v.into_iter().rev() {
                pee.replace_range(range, inner_html);
            }
        }

        recover(&mut pee, &RE_SVG_ELEMENT, &svg_inner_html_buffer);
        recover(&mut pee, &RE_STYLE_ELEMENT, &style_inner_html_buffer);
        recover(&mut pee, &RE_SCRIPT_ELEMENT, &script_inner_html_buffer);
        recover(&mut pee, &RE_TEXTAREA_ELEMENT, &svg_inner_html_buffer);

        if options.esc_pre || options.remove_useless_newlines_in_pre {
            let mut v = Vec::with_capacity(pre_inner_html_buffer.len());

            for (captures, inner_html) in
                RE_PRE_ELEMENT.captures_iter(pee.as_str()).zip(pre_inner_html_buffer.iter())
            {
                let (_, start, end) = get(&captures, 2);

                v.push((start..end, inner_html.0.as_str()));
            }

            if options.esc_pre {
                if options.remove_useless_newlines_in_pre {
                    for (range, inner_html) in v.into_iter().rev() {
                        pee.replace_range(
                            range,
                            html_escape::encode_safe(trim_newline_exactly_one(inner_html)).as_ref(),
                        );
                    }
                } else {
                    for (range, inner_html) in v.into_iter().rev() {
                        pee.replace_range(range, html_escape::encode_safe(inner_html).as_ref());
                    }
                }
            } else if options.remove_useless_newlines_in_pre {
                for (range, inner_html) in v.into_iter().rev() {
                    pee.replace_range(range, trim_newline_exactly_one(inner_html));
                }
            } else {
                for (range, inner_html) in v.into_iter().rev() {
                    pee.replace_range(range, inner_html);
                }
            }
        } else {
            recover(&mut pee, &RE_PRE_ELEMENT, &pre_inner_html_buffer);
        }
    }

    // Recover the newlines in tags that have been replaced with `'\r'` before.
    {
        let bytes = unsafe { pee.as_mut_vec() };

        for e in bytes {
            if *e == b'\r' {
                *e = b'\n';
            }
        }
    }

    pee
}

fn trim_newline_exactly_one<S: ?Sized + AsRef<str>>(s: &S) -> &str {
    let s = s.as_ref();
    let bytes = s.as_bytes();
    let length = bytes.len();

    if length == 0 {
        return "";
    }

    // from the start
    let bytes = match bytes[0] {
        b'\n' => {
            if length == 1 {
                return "";
            } else if bytes[1] != b'\n' && bytes[1] != b'\r' {
                &bytes[1..]
            } else {
                bytes
            }
        },
        b'\r' => {
            if length == 1 {
                return "";
            } else if bytes[1] == b'\n' {
                if length == 2 {
                    return "";
                } else if bytes[2] != b'\n' && bytes[2] != b'\r' {
                    &bytes[2..]
                } else {
                    bytes
                }
            } else if bytes[1] != b'\r' {
                &bytes[1..]
            } else {
                bytes
            }
        },
        _ => bytes,
    };

    let length = bytes.len();

    // from the end
    let bytes = match bytes[length - 1] {
        b'\n' => {
            if length == 1 {
                return "";
            } else if bytes[length - 2] != b'\n' && bytes[length - 2] != b'\r' {
                &bytes[..(length - 1)]
            } else {
                bytes
            }
        },
        b'\r' => {
            if length == 1 {
                return "";
            } else if bytes[length - 2] == b'\n' {
                if length == 2 {
                    return "";
                } else if bytes[length - 3] != b'\n' && bytes[length - 3] != b'\r' {
                    &bytes[..(length - 2)]
                } else {
                    bytes
                }
            } else if bytes[length - 2] != b'\r' {
                &bytes[..(length - 1)]
            } else {
                bytes
            }
        },
        _ => bytes,
    };

    unsafe { from_utf8_unchecked(bytes) }
}

#[cfg(feature = "onig")]
#[inline]
fn replace_all(regex: &Regex, pee: String, rep: &str) -> String {
    regex.replace_all(pee.as_str(), |caps: &regex::Captures| {
        let mut s = String::with_capacity(rep.len());

        let mut chars = rep.chars();

        while let Some(c) = chars.next() {
            if c == '$' {
                let index = (chars.next().unwrap() as u8 - b'0') as usize;

                s.push_str(caps.at(index).unwrap());
            } else {
                s.push(c);
            }
        }

        s
    })
}

#[cfg(not(feature = "onig"))]
#[inline]
fn replace_all(regex: &Regex, pee: String, rep: &str) -> String {
    match regex.replace_all(pee.as_str(), rep) {
        Cow::Owned(pee) => pee,
        Cow::Borrowed(_) => pee,
    }
}

#[cfg(feature = "onig")]
#[inline]
fn get<'a>(captures: &regex::Captures<'a>, index: usize) -> (&'a str, usize, usize) {
    let (start, end) = captures.pos(index).unwrap();

    (captures.at(index).unwrap(), start, end)
}

#[cfg(not(feature = "onig"))]
#[inline]
fn get<'a>(captures: &regex::Captures<'a>, index: usize) -> (&'a str, usize, usize) {
    let captures = captures.get(index).unwrap();

    (captures.as_str(), captures.start(), captures.end())
}
