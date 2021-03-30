use log::{debug, error, info};
use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use toml::map::Map;
use toml::value::Table;
use toml::value::Value;

enum LineType {
    H1,
    H2,
    H3,
    P,
    IMG,
}

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
        if line.starts_with("# ") {
            LineType::H1
        } else if line.starts_with("## ") {
            LineType::H2
        } else if line.starts_with("### ") {
            LineType::H3
        } else {
            LineType::P
        }
    }

    fn run_chapter(&self, chapter: &Chapter, config: &Table) -> Result<String, Error> {
        let content = chapter
            .content
            .lines()
            .map(|line| match self.get_line_type(&line) {});
        debug!("Content: {:?}", content);
        for key in chapter_config.keys() {
            debug!("Key: {}", key);
            match key.as_str() {
                "h1" => {}
                "h2" => {}
                "h3" => {}
                _ => error!("Key is not a valid html element name"),
            }
        }
        Ok(chapter.content)
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
                            let new_content = self.run_chapter(&chapter, &chapter_config)?;
                        }
                        _ => error!("Error: Invalid config. Chapter names should be a Toml Table"),
                    },
                    None => {
                        error!("Error: Invalid config. Chapter names should be in a Toml Table")
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
