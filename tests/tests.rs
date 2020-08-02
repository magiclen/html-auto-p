extern crate html_auto_p;

use html_auto_p::*;

#[test]
fn basic() {
    assert_eq!("", auto_p("", Options::new().br(true).esc_pre(true)));
    assert_eq!("", auto_p("\n", Options::new().br(true).esc_pre(true)));
    assert_eq!("", auto_p("\n\t", Options::new().br(true).esc_pre(true)));
    assert_eq!("", auto_p("\n\n", Options::new().br(true).esc_pre(true)));
    assert_eq!("<p>12345</p>", auto_p("12345", Options::new().br(true).esc_pre(true)));
    assert_eq!(
        "<p>12345<br>\n6789</p>",
        auto_p("12345\n6789", Options::new().br(true).esc_pre(true))
    );
    assert_eq!("<p>12345\n6789</p>", auto_p("12345\n6789", Options::new().br(false).esc_pre(true)));
    assert_eq!(
        "<p>12345<br>\n6789</p>",
        auto_p("12345<br/>\n6789", Options::new().br(true).esc_pre(true))
    );
    assert_eq!(
        "<p>12345<br/>\n6789</p>",
        auto_p("12345<br/>\n6789", Options::new().br(false).esc_pre(true))
    );
}

#[test]
fn in_block_basic() {
    assert_eq!(
        "<section></section>",
        auto_p("<section></section>", Options::new().br(true).esc_pre(true))
    );
    assert_eq!(
        "<section></section>",
        auto_p("<section></section>", Options::new().br(false).esc_pre(true))
    );
    assert_eq!(
        "<section>\n</section>",
        auto_p("<section>\n</section>", Options::new().br(true).esc_pre(true))
    );
    assert_eq!(
        "<section>\n</section>",
        auto_p("<section>\n</section>", Options::new().br(false).esc_pre(true))
    );
    assert_eq!(
        "<section>\n\t</section>",
        auto_p("<section>\n\t</section>", Options::new().br(true).esc_pre(true))
    );
    assert_eq!(
        "<section>\n\t</section>",
        auto_p("<section>\n\t</section>", Options::new().br(false).esc_pre(true))
    );
    assert_eq!(
        "<section>\n</section>",
        auto_p("<section>\n\n</section>", Options::new().br(true).esc_pre(true))
    );
    assert_eq!(
        "<section>\n</section>",
        auto_p("<section>\n\n</section>", Options::new().br(false).esc_pre(true))
    );
    assert_eq!(
        "<section>\n</section>",
        auto_p("<section>\n\n\n</section>", Options::new().br(true).esc_pre(true))
    );
    assert_eq!(
        "<section>\n</section>",
        auto_p("<section>\n\n\n</section>", Options::new().br(false).esc_pre(true))
    );
    assert_eq!(
        "<section>\n\t12345\n</section>",
        auto_p("<section>\n\t12345\n</section>", Options::new().br(true).esc_pre(true))
    );
    assert_eq!(
        "<section>\n\t12345<br>\n\t6789\n</section>",
        auto_p("<section>\n\t12345\n\t6789\n</section>", Options::new().br(true).esc_pre(true))
    );
    assert_eq!(
        "<section>\n\t12345\n\t6789\n</section>",
        auto_p("<section>\n\t12345\n\t6789\n</section>", Options::new().br(false).esc_pre(true))
    );
    assert_eq!(
        "<section>\n\t12345<br>\n\t6789\n</section>",
        auto_p(
            "<section>\n\t12345<br/>\n\t6789\n</section>",
            Options::new().br(true).esc_pre(true)
        )
    );
    assert_eq!(
        "<section>\n\t12345<br/>\n\t6789\n</section>",
        auto_p(
            "<section>\n\t12345<br/>\n\t6789\n</section>",
            Options::new().br(false).esc_pre(true)
        )
    );
}

#[test]
fn reserve_newline_in_tag() {
    assert_eq!(
        "<section data=\"Hello, world!\n\nHiHiHi!\" ></section>",
        auto_p(
            "<section data=\"Hello, world!\n\nHiHiHi!\" ></section>",
            Options::new().br(true).esc_pre(true)
        )
    );
}

#[test]
fn pre() {
    assert_eq!(
        "<pre>\t\t 123\n\t\t\n\t\t456<br><br>789  \n\t</pre>",
        auto_p(
            "<pre>\t\t 123\n\t\t\n\t\t456<br><br>789  \n\t</pre>",
            Options::new().br(true).esc_pre(false)
        )
    );
    assert_eq!(
        "<pre>\t\t 123\n\t\t\n\t\t456&lt;br&gt;&lt;br&gt;789  \n\t</pre>",
        auto_p(
            "<pre>\t\t 123\n\t\t\n\t\t456<br><br>789  \n\t</pre>",
            Options::new().br(true).esc_pre(true)
        )
    );
    assert_eq!(
        "<pre></pre>",
        auto_p("<pre>\n</pre>", Options::new().remove_useless_newlines_in_pre(true))
    );
    assert_eq!(
        "<pre>\n\n</pre>",
        auto_p("<pre>\n\n</pre>", Options::new().remove_useless_newlines_in_pre(true))
    );
    assert_eq!(
        "<pre>1</pre>",
        auto_p("<pre>\n1\n</pre>", Options::new().remove_useless_newlines_in_pre(true))
    );
    assert_eq!(
        "<pre>\n\n1\n\n</pre>",
        auto_p("<pre>\n\n1\n\n</pre>", Options::new().remove_useless_newlines_in_pre(true))
    );
}

#[test]
fn in_block_pre() {
    assert_eq!(
        "<section><pre>\t\t 123\n\t\t\n\t\t456<br><br>789  \n\t</pre></section>",
        auto_p(
            "<section><pre>\t\t 123\n\t\t\n\t\t456<br><br>789  \n\t</pre></section>",
            Options::new().br(true).esc_pre(false)
        )
    );
    assert_eq!(
        "<section><pre>\t\t 123\n\t\t\n\t\t456&lt;br&gt;&lt;br&gt;789  \n\t</pre></section>",
        auto_p(
            "<section><pre>\t\t 123\n\t\t\n\t\t456<br><br>789  \n\t</pre></section>",
            Options::new().br(true).esc_pre(true)
        )
    );
}

#[test]
fn script() {
    assert_eq!(
        "<script>alert('Hello');\n\nalert('World');</script>",
        auto_p(
            "<script>alert('Hello');\n\nalert('World');</script>",
            Options::new().br(true).esc_pre(false)
        )
    );
}

#[test]
fn in_block_script() {
    assert_eq!(
        "<section><script>alert('Hello');\n\nalert('World');</script></section>",
        auto_p(
            "<section><script>alert('Hello');\n\nalert('World');</script></section>",
            Options::new().br(true).esc_pre(false)
        )
    );
}
