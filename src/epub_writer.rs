use epub_builder::{EpubBuilder, EpubContent, ReferenceType, ZipLibrary};
use image::*;
use crate::reader::*;
use regex::Regex;
use std::collections::*;
use std::path::Path;

pub struct EpubWriter<'a> {
    pub title: String,
    pub author: String,
    pub cover_path: &'a Path,
    pub page_content: Vec<String>,
    pub bookmark_table: HashMap<String, usize>,
    pub constants: HashMap<String, String>,
}

impl<'a> EpubWriter<'a> {
    pub fn new(title: &str, author: &str, cover_path: &'a Path) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
            cover_path,
            page_content: Vec::new(),
            bookmark_table: HashMap::new(),
            constants: HashMap::new(),
        }
    }

    pub fn process_bookmark_table(&mut self, input: &Reader) {
        let mut current_page: usize = 0;

        for line in &input.lines {
            if line.type_ == LineType::Bookmark {
                current_page += 1;

                // Remove equal characters and white spaces
                let chars_to_trim: &[char] = &['=', ' '];

                // Add the new string to the symbols
                let trimmed_string: &str = line.text.trim_matches(chars_to_trim);

                self.bookmark_table
                    .insert(trimmed_string.to_string(), current_page);
            }
        }
    }

    pub fn process_lines(&mut self, input: &Reader) {
        let mut current_page: usize = 0;

        self.process_bookmark_table(input);

        // Put an empty string to the first index of the vector
        self.page_content.push(String::new());

        for (current_line, line) in input.lines.iter().enumerate() {
            match line.type_ {
                LineType::Undefined => panic!(format!("Line {} cannot be parsed", &current_line)),
                LineType::Text => {
                    self.page_content[current_page].push_str(&format!("<p>{}</p>", &line.text));
                }
                // TODO: Implement questions
                LineType::Question => {
                    // Check between brackets
                    let re_text = Regex::new(r"\[(.*?)\]")
                        .unwrap()
                        .captures(&line.text)
                        .unwrap_or_else(|| {
                            panic!(
                                "Cannot get key of question while parsing at line {}",
                                &current_line
                            )
                        });

                    // Check after arrow
                    let re_jump = Regex::new(r"\->\s+(.*)$")
                        .unwrap()
                        .captures(&line.text)
                        .unwrap_or_else(|| {
                            panic!(
                                "Cannot get value of question while parsing at line {}",
                                &current_line
                            )
                        });

                    let title = re_text[1].to_string();
                    let page = format!(
                        "chapter_{}.xhtml",
                        self.bookmark_table[&re_jump[1].to_string()]
                    );

                    self.page_content[current_page]
                        .push_str(&format!("<p><a href=\"{}\">{}</a></p>", page, title));
                }
                LineType::Bookmark => {
                    self.page_content.push(String::new());
                    current_page += 1;
                }
                // TODO: Implement constants
                LineType::Constant => {}
                // TODO: Implement end
                LineType::End => {}
                _ => (),
            }
        }
    }

    // TODO: Handle errors
    pub fn generate(&self) -> Option<Vec<u8>> {
        let image = image::open(&self.cover_path).unwrap();

        let mut jpg = Vec::new();
        image.write_to(&mut jpg, ImageFormat::Jpeg).unwrap();

        let css = "";

        let mut epub: Vec<u8> = Vec::new();

        // Init Epub builder
        let mut builder = EpubBuilder::new(ZipLibrary::new().unwrap()).unwrap();

        // Add metadata
        builder
            .metadata("author", self.author.clone())
            .unwrap()
            .metadata("title", self.title.clone())
            .unwrap();

        // Add stylesheet
        builder.stylesheet(css.as_bytes()).unwrap();

        // Add cover image
        builder
            .add_cover_image("cover.jpg", jpg.as_slice(), "image/jpeg")
            .unwrap();

        // Add cover file
        builder
            .add_content(
                EpubContent::new(
                    "cover.xhtml",
                    self.cover_builder(image.height(), image.width()).as_bytes(),
                )
                .title("Cover")
                .reftype(ReferenceType::Cover),
            )
            .unwrap();

        // Add title file
        builder
            .add_content(
                EpubContent::new("title.xhtml", self.title_builder().as_bytes())
                    .title("Title")
                    .reftype(ReferenceType::TitlePage),
            )
            .unwrap();

        // Add page file
        for (counter, page) in self.page_content.iter().enumerate() {
            builder
                .add_content(
                    EpubContent::new(
                        format!("chapter_{}.xhtml", &counter),
                        self.page_builder(page).as_bytes(),
                    )
                    .title(format!("Chapter {}", &counter))
                    .reftype(ReferenceType::Text),
                )
                .unwrap();
        }

        builder.generate(&mut epub).unwrap();

        Some(epub)
    }

    pub fn cover_builder(&self, height: u32, width: u32) -> String {
        format!(r#"<?xml version="1.0" encoding="UTF-8" standalone="no" ?><html xmlns="http://www.w3.org/1999/xhtml">
<head>
    <title>Cover</title>
    <meta content="http://www.w3.org/1999/xhtml; charset=utf-8" http-equiv="Content-Type"/>
    <style title="override_css" type="text/css">
        @page {{ padding: 0pt; margin: 0pt }}
        body {{ text-align: center; padding:0pt; margin: 0pt; }}
    </style>
</head>
<body>
    <div>
        <svg xmlns="http://www.w3.org/2000/svg" height="100%" version="1.1" viewBox="0 0 {width} {height}" width="100%" xmlns:xlink="http://www.w3.org/1999/xlink">
            <image height="{height}" width="{width}" xlink:href="{filename}"/>
        </svg>
    </div>
</body>
</html>"#, height = height, width = width, filename = "cover.jpg")
    }

    pub fn title_builder(&self) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="no" ?>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
	<title>{title}</title>
	<meta content="http://www.w3.org/1999/xhtml; charset=utf-8" http-equiv="Content-Type" />
	<link href="stylesheet.css" rel="stylesheet" type="text/css" />
</head>
<body>
	<h1>{title}</h1>
	<p>{author}</p>
</body>
</html>"#,
            title = self.title,
            author = self.author
        )
    }

    pub fn page_builder(&self, content: &str) -> String {
        format!(r#"<?xml version="1.0" encoding="UTF-8" standalone="no" ?><html xmlns="http://www.w3.org/1999/xhtml">
<head>
    <title>Page</title>
    <meta content="http://www.w3.org/1999/xhtml; charset=utf-8" http-equiv="Content-Type"/>
    <link href="stylesheet.css" rel="stylesheet" type="text/css"/>
</head>
<body>
    {}
</body>
</html>"#, content)
    }
}
