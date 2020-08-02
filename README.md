HTML Auto `<p>`
====================

[![Build Status](https://travis-ci.org/magiclen/html-auto-p.svg?branch=master)](https://travis-ci.org/magiclen/html-auto-p)

This library provides a function like `wpautop` in Wordpress. It uses a group of regex replaces used to identify text formatted with newlines and replace double line-breaks with HTML paragraph tags.

Someone who familiars with HTML would prefer directly writing plain HTML instead of using an editor like TinyMCE or Gutenberg. However, it takes time to manually add newlines and paragraphs in HTML. Wordpress provides a handy function called `wpautop` which can replace double line-breaks with paragraph elements (`<p>`) and convert remaining line-breaks to `<br>` elements.

The `auto_p` function in this library can be used like `wpautop`.

```rust
extern crate html_auto_p;

use html_auto_p::auto_p;

assert_eq!("<p>Hello world!</p>", auto_p("Hello world!", false, false));
assert_eq!("<p>Line 1<br>\nLine 2</p>", auto_p("Line 1\nLine 2", true, false));
assert_eq!("<p>Line 1<br>\nLine 2</p>", auto_p("Line 1<br>\nLine 2", true, false));
assert_eq!("<p>Paragraph 1</p>\n<p>Paragraph 2</p>", auto_p("Paragraph 1\n\nParagraph 2", false, false));
assert_eq!("<pre>Line 1<br>\nLine 2</pre>", auto_p("<pre>Line 1<br>\nLine 2</pre>", true, false));
assert_eq!("<pre>Line 1&lt;br&gt;\nLine 2</pre>", auto_p("<pre>Line 1<br>\nLine 2</pre>", true, true));
```

* The first parameter is the input HTML.
* The second parameter is to control whether to convert remaining line-breaks to `<br>` elements.
* The third parameter is to control whether to escape the inner HTML in `<pre>` elements. This is useful when the inner HTML needs to be formatted and be wrapped into other non-`<pre>` elements.

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