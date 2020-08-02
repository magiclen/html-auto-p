HTML Auto `<p>`
====================

[![Build Status](https://travis-ci.org/magiclen/html-auto-p.svg?branch=master)](https://travis-ci.org/magiclen/html-auto-p)

This library provides a function like `wpautop` in Wordpress. It uses a group of regex replaces used to identify text formatted with newlines and replace double line-breaks with HTML paragraph tags.

Someone who familiars with HTML would prefer directly writing plain HTML instead of using an editor like TinyMCE or Gutenberg. However, it takes time to manually add newlines and paragraphs in HTML. Wordpress provides a handy function called `wpautop` which can replace double line-breaks with paragraph elements (`<p>`) and convert remaining line-breaks to `<br>` elements.

The `auto_p` function in this library can be used like `wpautop`.

```rust
extern crate html_auto_p;

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

## Crates.io

https://crates.io/crates/html-auto-p

## Documentation

https://docs.rs/html-auto-p

## License

[MIT](LICENSE)