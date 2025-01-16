# mdbook-external-links

Open external links inside your mdBooks in a different tab.

## Installation & usage

Install through `cargo`

```bash
cargo install mdbook-external-links
```

Include it in your `book.toml`

```toml
[preprocessor.external-links]
```

That's it! 🚀

## What counts as an external link?

- [Inline](https://spec.commonmark.org/0.30/#inline-link), [reference](https://spec.commonmark.org/0.30/#full-reference-link), [shortcut](https://spec.commonmark.org/0.30/#shortcut-reference-link), and [collapsed](https://spec.commonmark.org/0.30/#collapsed-reference-link) links that start with http(s) (other protocols are missing at the moment).
- Any [autolink](https://spec.commonmark.org/0.30/#autolinks)
