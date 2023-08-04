use cfg_if::cfg_if;

use leptos::ServerFnError;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct PostFrontMatter {
    pub title: String,
    pub tags: Option<Vec<String>>,
    pub excerpt: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RawPost {
    pub id: String,
    pub frontmatter: PostFrontMatter,
    #[serde(skip)]
    pub markdown: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Post {
    pub id: String,
    pub frontmatter: PostFrontMatter,
    pub toc: Option<String>,
    pub content: String,
}

cfg_if! {
if #[cfg(feature = "ssr")] {

impl RawPost {
    pub fn parse_markdown(self) -> Result<Post, leptos::ServerFnError> {
        let output = femark::process_markdown_to_html(self.markdown)
            .map_err(|_| leptos::ServerFnError::ServerError("Could not parse markdown".into()))?;
        Ok(Post {
            id: self.id,
            frontmatter: self.frontmatter,
            toc: output.toc,
            content: output.content,
        })
    }
}

pub fn get_post(id: &str) -> leptos::error::Result<RawPost, leptos::ServerFnError> {
    let file_path = std::env::current_dir()?.join("markdown").join(format!("{}.md", id));
    let file_contents = std::fs::read_to_string(file_path)?;

    let (fm, markdown) = parse_frontmatter(&file_contents)?;

    Ok(RawPost {
        id: id.into(),
        frontmatter: fm,
        markdown: markdown.into(),
    })
}

pub fn get_posts() -> leptos::error::Result<Vec<RawPost>, leptos::ServerFnError> {
    let file_path = std::env::current_dir()?.join("markdown");
    let files = std::fs::read_dir(file_path)?;
    let mut posts = vec![];

    files.for_each(|file| {
        if let Ok(file) = file {
            let file_stem = file.path().file_stem().unwrap_or_default().to_string_lossy().to_string();

            if let Ok(content) = std::fs::read_to_string(file.path()) {
                if let Ok((fm, markdown)) = parse_frontmatter(&content) {
                    let post = RawPost {
                        id: file_stem,
                        frontmatter: fm,
                        markdown: markdown.to_string(),
                    };
                    posts.push(post);
                }
            }
        } else {
            todo!()
        }
    });

    Ok(posts)
}

fn find_yaml_block(text: &str) -> Option<(usize, usize, usize)> {
    match text.starts_with("---\n") {
        true => {
            let slice_after_marker = &text[4..];
            let fm_end = slice_after_marker.find("---\n")?;
            Some((4, fm_end + 4, fm_end + 2 * 4))
        }
        false => None,
    }
}

fn parse_frontmatter(text: &str) -> Result<(PostFrontMatter, &str), ServerFnError> {
    let (start, end, content_start) = find_yaml_block(text).ok_or(ServerFnError::ServerError("Could not find frontmatter".to_string()))?;
    let yaml_block = &text[start..end];
    let frontmatter = serde_yaml::from_str(yaml_block)?;
    Ok((frontmatter, &text[content_start..]))
}


}
}
