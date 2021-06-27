#[derive(Debug)]
pub struct ChapterData {
    pub html: String,
    tags: Vec<String>,
}

impl ChapterData {
    pub fn new() -> ChapterData {
        let html = String::from("");
        let tags: Vec<String> = Vec::new();
        ChapterData {
            html,
            tags
        }
    }
    pub fn fetch_tag_from_line(&mut self, line: &str) {
        for word in line.split_whitespace() {
            match &word.find("{{") {
                Some(lb) => {
                    match &word.find("}}") {
                        Some(rb) => {
                            let l = lb.to_owned();
                            let r = rb.to_owned();
                            let tag = &word[l+2..r];
                            self.tags.push(String::from(tag));
                        },
                        None => {
                            continue;
                        }
                    }
                },
                None => {
                    continue;
                }
            }
        }

    }

    pub fn print_info(&self) {
        println!("[ HTML CONTENT ] {}", &self.html);
        println!("[ TAGS ]");
        for tag in &self.tags {
            println!("{}", tag);
        }
    }
}
