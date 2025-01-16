# ndsdoc

## Setup

You need Python, Rust and mdBook.

```sh
# clone the repo
git clone --recurse-submodules git@github.com:gbadev-org/ndsdoc.git
cd ndsdoc

cargo install mdbook

# run the development server
mdbook serve --open
```

The book will be live at http://localhost:3000