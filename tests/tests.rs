extern crate html_auto_p;

#[test]
fn basic() {
    assert_eq!("", html_auto_p::auto_p("", true, true));
    assert_eq!("", html_auto_p::auto_p("\n", true, true));
    assert_eq!("", html_auto_p::auto_p("\n\t", true, true));
    assert_eq!("", html_auto_p::auto_p("\n\n", true, true));
    assert_eq!("<p>12345</p>", html_auto_p::auto_p("12345", true, true));
    assert_eq!("<p>12345<br>\n6789</p>", html_auto_p::auto_p("12345\n6789", true, true));
    assert_eq!("<p>12345\n6789</p>", html_auto_p::auto_p("12345\n6789", false, true));
    assert_eq!("<p>12345<br>\n6789</p>", html_auto_p::auto_p("12345<br/>\n6789", true, true));
    assert_eq!("<p>12345<br/>\n6789</p>", html_auto_p::auto_p("12345<br/>\n6789", false, true));
}

#[test]
fn in_block_basic() {
    assert_eq!("<section></section>", html_auto_p::auto_p("<section></section>", true, true));
    assert_eq!("<section></section>", html_auto_p::auto_p("<section></section>", false, true));
    assert_eq!("<section>\n</section>", html_auto_p::auto_p("<section>\n</section>", true, true));
    assert_eq!("<section>\n</section>", html_auto_p::auto_p("<section>\n</section>", false, true));
    assert_eq!(
        "<section>\n\t</section>",
        html_auto_p::auto_p("<section>\n\t</section>", true, true)
    );
    assert_eq!(
        "<section>\n\t</section>",
        html_auto_p::auto_p("<section>\n\t</section>", false, true)
    );
    assert_eq!("<section>\n</section>", html_auto_p::auto_p("<section>\n\n</section>", true, true));
    assert_eq!(
        "<section>\n</section>",
        html_auto_p::auto_p("<section>\n\n</section>", false, true)
    );
    assert_eq!(
        "<section>\n</section>",
        html_auto_p::auto_p("<section>\n\n\n</section>", true, true)
    );
    assert_eq!(
        "<section>\n</section>",
        html_auto_p::auto_p("<section>\n\n\n</section>", false, true)
    );
    assert_eq!(
        "<section>\n\t12345\n</section>",
        html_auto_p::auto_p("<section>\n\t12345\n</section>", true, true)
    );
    assert_eq!(
        "<section>\n\t12345<br>\n\t6789\n</section>",
        html_auto_p::auto_p("<section>\n\t12345\n\t6789\n</section>", true, true)
    );
    assert_eq!(
        "<section>\n\t12345\n\t6789\n</section>",
        html_auto_p::auto_p("<section>\n\t12345\n\t6789\n</section>", false, true)
    );
    assert_eq!(
        "<section>\n\t12345<br>\n\t6789\n</section>",
        html_auto_p::auto_p("<section>\n\t12345<br/>\n\t6789\n</section>", true, true)
    );
    assert_eq!(
        "<section>\n\t12345<br/>\n\t6789\n</section>",
        html_auto_p::auto_p("<section>\n\t12345<br/>\n\t6789\n</section>", false, true)
    );
}

#[test]
fn reserve_newline_in_tag() {
    assert_eq!(
        "<section data=\"Hello, world!\n\nHiHiHi!\" ></section>",
        html_auto_p::auto_p("<section data=\"Hello, world!\n\nHiHiHi!\" ></section>", true, true)
    );
}

#[test]
fn pre() {
    assert_eq!(
        "<pre>\t\t 123\n\t\t\n\t\t456<br><br>789  \n\t</pre>",
        html_auto_p::auto_p("<pre>\t\t 123\n\t\t\n\t\t456<br><br>789  \n\t</pre>", true, false)
    );
    assert_eq!(
        "<pre>\t\t 123\n\t\t\n\t\t456&lt;br&gt;&lt;br&gt;789  \n\t</pre>",
        html_auto_p::auto_p("<pre>\t\t 123\n\t\t\n\t\t456<br><br>789  \n\t</pre>", true, true)
    );
}

#[test]
fn in_block_pre() {
    assert_eq!(
        "<section><pre>\t\t 123\n\t\t\n\t\t456<br><br>789  \n\t</pre></section>",
        html_auto_p::auto_p(
            "<section><pre>\t\t 123\n\t\t\n\t\t456<br><br>789  \n\t</pre></section>",
            true,
            false
        )
    );
    assert_eq!(
        "<section><pre>\t\t 123\n\t\t\n\t\t456&lt;br&gt;&lt;br&gt;789  \n\t</pre></section>",
        html_auto_p::auto_p(
            "<section><pre>\t\t 123\n\t\t\n\t\t456<br><br>789  \n\t</pre></section>",
            true,
            true
        )
    );
}

#[test]
fn script() {
    assert_eq!(
        "<script>alert('Hello');\n\nalert('World');</script>",
        html_auto_p::auto_p("<script>alert('Hello');\n\nalert('World');</script>", true, false)
    );
}

#[test]
fn in_block_script() {
    assert_eq!(
        "<section><script>alert('Hello');\n\nalert('World');</script></section>",
        html_auto_p::auto_p(
            "<section><script>alert('Hello');\n\nalert('World');</script></section>",
            true,
            false
        )
    );
}
