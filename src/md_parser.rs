pub struct Metadata {
    pub name: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
}

impl Metadata {
    pub fn new() -> Self {
        Metadata {
            name: None,
            author: None,
            description: None,
        }
    }

    pub fn add(&mut self, name: String, value: String) -> Result<(), String> {
        match name.as_str() {
            "name" => Ok(self.name = Some(value)),
            "author" => Ok(self.author = Some(value)),
            "description" => Ok(self.description = Some(value)),
            _ => Err(format!("Unknown metadata key: {}", name)),
        }
    }
}

pub fn parse_md(content: String) -> Option<Metadata> {
    let lines = content
        .split('\n')
        .map(|c| c.to_string())
        .collect::<Vec<_>>();

    if lines[0].trim() == "---" {
        let mut i = 1_usize;
        let mut metadata = Metadata::new();

        while i < lines.len() {
            if lines[i].trim() == "---" {
                break;
            }

            let splits = lines[i].split(':').collect::<Vec<_>>();

            let (key, value) = (splits[0], splits[1]);
            match metadata.add(key.trim().to_string(), value.trim().to_string()) {
                Err(e) => println!("\x1b[31m\x1b[1merror\x1b[0m{}\x1b[0m", e),
                _ => (),
            }

            i += 1;
        }

        Some(metadata)
    } else {
        None
    }
}
