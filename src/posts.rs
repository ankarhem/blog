use cfg_if::cfg_if;
use leptos::ServerFnError;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct RawPost {
    pub id: String,
    pub title: String,
    pub tags: Option<Vec<String>>,
    #[serde(skip)]
    pub markdown: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub tags: Option<Vec<String>>,
    pub toc: Option<String>,
    pub content: String,
}

cfg_if! {
if #[cfg(feature = "ssr")] {

impl RawPost {
    pub fn parse_markdown(&self) -> Result<Post, ServerFnError> {
        let output = femark::process_markdown_to_html(self.markdown.clone())
            .map_err(|_| ServerFnError::ServerError("Could not parse markdown".into()))?;
        Ok(Post {
            id: self.id.clone(),
            title: self.title.clone(),
            tags: self.tags.clone(),
            toc: output.toc,
            content: output.content,
        })
    }
}

pub fn get_post(id: &str) -> leptos::error::Result<RawPost, ServerFnError> {
    let file_path = std::env::current_dir()?.join("markdown").join(format!("{}.md", id));
    let file_contents = std::fs::read_to_string(file_path)?;

    Ok(RawPost {
        id: id.into(),
        title: "TODO".into(),
        tags: None,
        markdown: file_contents,
    })
}

pub fn get_posts() -> leptos::error::Result<Vec<RawPost>, ServerFnError> {
    let file_path = std::env::current_dir()?.join("markdown");
    let files = std::fs::read_dir(file_path)?;
    let mut posts = vec![];

    files.for_each(|file| {
        if let Ok(file) = file {
            let file_stem = file.path().file_stem().unwrap_or_default().to_string_lossy().to_string();

            if let Ok(markdown) = std::fs::read_to_string(file.path()) {
                let post = RawPost {
                    id: file_stem,
                    title: "TODO".into(),
                    tags: None,
                    markdown,
                };
            posts.push(post);
            }
        } else {
            todo!()
        }
    });

    Ok(posts)
}

}
}
