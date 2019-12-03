You're probably looking for [v_htmlescape](https://crates.io/crates/v_htmlescape),
[askama_escape](https://crates.io/crates/askama_escape) or
[htmlescape](https://github.com/veddan/rust-htmlescape) instead.

# webescape

This is a minimal html escape crate which is more or less a fork of
[askama_escape](https://crates.io/crates/askama_escape).

Apparently most projects follow what
[OWASP says about how to escape HTML](https://github.com/OWASP/CheatSheetSeries/blob/master/cheatsheets/Cross_Site_Scripting_Prevention_Cheat_Sheet.md#rule-1---html-escape-before-inserting-untrusted-data-into-html-element-content).
Their [minimal encoding rules](https://www.owasp.org/index.php/XSS_Experimental_Minimal_Encoding_Rules)
seem like a much better guideline.
One could just as well follow the
[HTML spec](https://html.spec.whatwg.org/multipage/syntax.html)
since browsers implementations are good and even stuff like

```html
<div class="<script>alert(/xss/)">
```

is safe nowadays.

Therefore this crate only does minimal escaping to barely prevent cross site
scripting attacks, please use this crate with caution.
The functions in this crate won't prevent attacks that use user input on unsafe
attributes such as `href` or `style`.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
