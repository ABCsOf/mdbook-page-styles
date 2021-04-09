use log::{debug, error, info};
use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use toml::map::Map;
use toml::value::Table;
use toml::value::Value;

enum LineType {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    P,
}

const H1_PREFIX: &'static str = "# ";
const H2_PREFIX: &'static str = "## ";
const H3_PREFIX: &'static str = "### ";
const H4_PREFIX: &'static str = "#### ";
const H5_PREFIX: &'static str = "##### ";
const H6_PREFIX: &'static str = "###### ";

pub struct PageStyler {
    default_config: Map<String, Value>,
}

impl PageStyler {
    pub fn new() -> Self {
        Self {
            default_config: Table::new(),
        }
    }

    fn get_line_type(&self, line: &str) -> LineType {
        if line.starts_with(H1_PREFIX) {
            LineType::H1
        } else if line.starts_with(H2_PREFIX) {
            LineType::H2
        } else if line.starts_with(H3_PREFIX) {
            LineType::H3
        } else if line.starts_with(H4_PREFIX) {
            LineType::H4
        } else if line.starts_with(H5_PREFIX) {
            LineType::H5
        } else if line.starts_with(H6_PREFIX) {
            LineType::H6
        } else {
            LineType::P
        }
    }

    fn get_class_for<'a>(config: &'a Table, key: &str) -> Option<&'a String> {
        match config.get(key) {
            Some(val) => match val {
                Value::Table(table) => match table.get("class") {
                    Some(val) => match val {
                        Value::String(s) => Some(&s),
                        _ => None,
                    },
                    None => None,
                },
                _ => None,
            },
            None => None,
        }
    }

    fn wrap_with_html(
        line: &str,
        tag_name: &str,
        class: &str,
        strip_prefix: &'static str,
    ) -> String {
        format!(
            "<{0} class=\"{1}\">{2}</{0}>",
            tag_name,
            class,
            line.strip_prefix(strip_prefix).unwrap()
        )
    }

    fn run_chapter(&self, chapter: &Chapter, config: &Table) -> Result<String, Error> {
        let content = chapter
            .content
            .lines()
            .map(|line| match self.get_line_type(&line) {
                LineType::H1 => {
                    if let Some(class) = PageStyler::get_class_for(config, "h1") {
                        PageStyler::wrap_with_html(line, "h1", class, H1_PREFIX)
                    } else {
                        line.into()
                    }
                }
                LineType::H2 => {
                    if let Some(class) = PageStyler::get_class_for(config, "h2") {
                        PageStyler::wrap_with_html(line, "h2", class, H2_PREFIX)
                    } else {
                        line.into()
                    }
                }
                LineType::H3 => {
                    if let Some(class) = PageStyler::get_class_for(config, "h3") {
                        PageStyler::wrap_with_html(line, "h3", class, H3_PREFIX)
                    } else {
                        line.into()
                    }
                }
                LineType::H4 => {
                    if let Some(class) = PageStyler::get_class_for(config, "h4") {
                        PageStyler::wrap_with_html(line, "h4", class, H4_PREFIX)
                    } else {
                        line.into()
                    }
                }
                LineType::H5 => {
                    if let Some(class) = PageStyler::get_class_for(config, "h5") {
                        PageStyler::wrap_with_html(line, "h5", class, H5_PREFIX)
                    } else {
                        line.into()
                    }
                }
                LineType::H6 => {
                    if let Some(class) = PageStyler::get_class_for(config, "h6") {
                        PageStyler::wrap_with_html(line, "h6", class, H6_PREFIX)
                    } else {
                        line.into()
                    }
                }
                _ => line.into(),
            })
            .collect();
        debug!("Content: {:?}", content);
        Ok(content)
    }
}

impl Preprocessor for PageStyler {
    fn name(&self) -> &str {
        "page-styles"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        info!("Context: {:?}", ctx.config);
        let context = ctx
            .config
            .get_preprocessor(self.name())
            .unwrap_or(&self.default_config);
        info!("Preprocessor context: {:?}", context);
        book.for_each_mut(|item| match item {
            BookItem::Chapter(chapter) => {
                info!("Chapter: {:?}", chapter);
                match context.get(&chapter.name) {
                    Some(value) => match value {
                        Value::Table(chapter_config) => {
                            let new_content = self.run_chapter(&chapter, &chapter_config).unwrap();
                            chapter.content = new_content;
                        }
                        _ => error!("Error: Invalid config. Chapter names should be a Toml Table"),
                    },
                    None => {
                        info!("Skipping chapter {}: No config values.", chapter.name)
                    }
                };
            }
            _ => debug!("Not a chapter"),
        });
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        info!("Supports '{}'?", renderer);
        match renderer {
            "html" => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger;
    use mdbook::book;

    #[test]
    fn it_checks_supported_renderer() {
        let processor = PageStyler::new();

        assert!(processor.supports_renderer("html"));
        assert_ne!(processor.supports_renderer("pdf"), true);
    }

    #[test]
    fn it_runs() {
        env_logger::init();
        let processor = PageStyler::new();
        let mut mdbook = book::MDBook::load("./test-book").unwrap();
        mdbook.with_preprocessor(processor);
        mdbook.build().unwrap();
        assert!(true);
    }
}
