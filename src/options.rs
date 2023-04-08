#[derive(Default, Debug, Clone)]
/// Options for the `auto_p` function.
pub struct Options {
    /// Whether to convert remaining line-breaks to `<br>` elements.
    pub br:                             bool,
    /// Whether to escape the inner HTML in `<pre>` elements. This is useful when the inner HTML needs to be formatted and be wrapped into other non-`<pre>` elements.
    pub esc_pre:                        bool,
    /// Whether to remove useless newlines in the inner HTML of `<pre>` elements. This is useful to beautifully form code into `<pre>\n...\n</pre>` without worrying about the adjacent newlines' effects.
    pub remove_useless_newlines_in_pre: bool,
}

impl Options {
    /// Create default options. (All false)
    #[inline]
    pub const fn new() -> Self {
        Options {
            br:                             false,
            esc_pre:                        false,
            remove_useless_newlines_in_pre: false,
        }
    }

    /// Set whether to convert remaining line-breaks to `<br>` elements.
    #[inline]
    pub const fn br(mut self, br: bool) -> Self {
        self.br = br;

        self
    }

    /// Set whether to escape the inner HTML in `<pre>` elements.
    #[inline]
    pub const fn esc_pre(mut self, esc_pre: bool) -> Self {
        self.esc_pre = esc_pre;

        self
    }

    /// Set whether to remove useless newlines in the inner HTML of `<pre>` elements.
    #[inline]
    pub const fn remove_useless_newlines_in_pre(
        mut self,
        remove_useless_newlines_in_pre: bool,
    ) -> Self {
        self.remove_useless_newlines_in_pre = remove_useless_newlines_in_pre;

        self
    }
}
