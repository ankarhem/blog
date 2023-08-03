use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Post {
    toc: Option<String>,
    content: String,
}

#[cfg(feature = "ssr")]
impl From<femark::HTMLOutput> for Post {
    fn from(output: femark::HTMLOutput) -> Self {
        Self {
            toc: output.toc,
            content: output.content,
        }
    }
}

#[server(GetPost, "/api")]
pub async fn get_post(id: String) -> Result<Post, ServerFnError> {
    println!("get_post: {}", id);

    let file_path = std::env::current_dir()?
        .join("markdown")
        .join(format!("{}.md", id));
    println!("file_path: {:?}", file_path);
    let file_contents = std::fs::read_to_string(file_path)?;

    let markdown = femark::process_markdown_to_html(file_contents)
        .map_err(|_| ServerFnError::ServerError("could not parse markdown".into()))?;

    Ok(markdown.into())
}

#[component]
pub fn PostPage(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    // id: || -> usize
    let post_id = move || params.with(|params| params.get("post_id").cloned().unwrap_or_default());
    let post = create_resource(cx, post_id, |id| async move { get_post(id).await });

    view! { cx,
        <Transition
            fallback=move || view! { cx, <p>"Loading..."</p> }
        >
            {move || match post.read(cx) {
                Some(Ok(data)) => view! { cx,
                    <>
                        {data.toc.map(|toc| view! { cx,
                            <div
                                class="max-w-[50rem] mx-auto p-4 mb-8 bg-stone-200 rounded"
                                inner_html=toc
                            />
                        })}
                        <article
                            class="prose prose-lg mx-auto max-w-[unset] [&>*:not(.code-block)]:max-w-[50rem] [&>*]:mx-auto"
                            inner_html=data.content
                        />
                    </>
                }.into_view(cx),
                _ => view! { cx, <p>"Error"</p> }.into_view(cx),
            }}
        </Transition>
    }
}
