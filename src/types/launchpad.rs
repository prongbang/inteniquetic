#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectLink {
    pub url: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Category {
    pub name: String,
    pub description: String,
    pub links: Vec<ProjectLink>,
}

impl Category {
    pub fn new(name: &str, description: &str, links: &[&str]) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            links: links
                .iter()
                .map(|url| ProjectLink {
                    url: (*url).to_string(),
                })
                .collect(),
        }
    }
}
