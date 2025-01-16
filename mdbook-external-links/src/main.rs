use clap::{Arg, ArgMatches, Command};
use exl_lib::Exl;
use mdbook::{
    book::{Book, Chapter},
    errors::Result,
    preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext},
};
use semver::{Version, VersionReq};
use std::io;
use std::process;

pub fn make_app() -> Command {
    Command::new("external-links-preprocessor")
        .about(r#"A mdbook preprocessor that adds 'target="_blank"' to anchor tags"#)
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}

fn main() {
    let matches = make_app().get_matches();

    let preprocessor = Exl::new();
    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&preprocessor, sub_args);
    } else if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<()> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args
        .get_one::<String>("renderer")
        .expect("Required argument");

    let supported = pre.supports_renderer(renderer);

    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

mod exl_lib {
    use pulldown_cmark::{CowStr, Event, LinkType, Options, Parser, Tag};
    use pulldown_cmark_to_cmark::cmark;

    use super::*;

    pub struct Exl;

    impl Exl {
        pub fn new() -> Self {
            Self
        }

        fn replace_anchors(&self, chapter: &mut Chapter) -> Result<String> {
            let mut buf = String::with_capacity(chapter.content.len());

            let events = Parser::new_ext(&chapter.content, Options::all()).map(|e| {
                let ev = match &e {
                    Event::Start(Tag::Link(..)) => "start",
                    Event::End(Tag::Link(..)) => "end",
                    _ => "other",
                };
                if ev == "other" {
                    return e;
                }
                let (lt, url, title) = match &e {
                    Event::Start(Tag::Link(lt, url, title)) => (lt, url, title),
                    Event::End(Tag::Link(lt, url, title)) => (lt, url, title),
                    _ => unreachable!(),
                };

                match lt {
                    LinkType::Shortcut
                    | LinkType::Inline
                    | LinkType::Reference
                    | LinkType::Collapsed => {
                        if url.starts_with("http") {
                            if ev == "end" {
                                Event::Html(CowStr::from("</a>"))
                            } else {
                                Event::Html(CowStr::from(format!(
                                    r#"<a href="{url}" title="{title}" target="_blank">"#
                                )))
                            }
                        } else {
                            e
                        }
                    }
                    LinkType::Email => {
                        if ev == "end" {
                            Event::Html(CowStr::from("</a>"))
                        } else {
                            Event::Html(CowStr::from(format!(r#"<a href="mailto:{url}">"#)))
                        }
                    }
                    LinkType::Autolink => {
                        if ev == "end" {
                            Event::Html(CowStr::from("</a>"))
                        } else {
                            Event::Html(CowStr::from(format!(
                                r#"<a href="{url}" target="_blank">"#
                            )))
                        }
                    }
                    LinkType::ReferenceUnknown => e,
                    LinkType::CollapsedUnknown => e,
                    LinkType::ShortcutUnknown => e,
                }
            });

            cmark(events, &mut buf)
                .map(|_| buf)
                .map_err(|err| anyhow::anyhow!("Markdown serialization failed: {err}"))
        }
    }

    impl Preprocessor for Exl {
        fn name(&self) -> &str {
            "external-links-preprocessor"
        }

        fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
            book.for_each_mut(|bi| match bi {
                mdbook::BookItem::Chapter(ref mut c) => {
                    c.content = self
                        .replace_anchors(c)
                        .expect("Error converting links for chapter");
                }
                mdbook::BookItem::Separator | mdbook::BookItem::PartTitle(_) => {}
            });
            Ok(book)
        }
    }
}
