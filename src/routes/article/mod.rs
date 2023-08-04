mod skeleton;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use skeleton::*;

#[server(GetPost, "/api")]
pub async fn get_post(cx: Scope, id: String) -> Result<crate::posts::Post, ServerFnError> {
    use crate::posts::*;
    println!("get_post: {}", id);

    let post = get_post(&id)?;
    let parsed = post.parse_markdown()?;

    // let ten_millis = std::time::Duration::from_millis(10000);
    // std::thread::sleep(ten_millis);

    Ok(parsed)
}

#[component]
pub fn ArticlePage(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    // id: || -> usize
    let post_id = move || params.with(|params| params.get("post_id").cloned().unwrap_or_default());
    let post =
        create_blocking_resource(cx, post_id, move |id| async move { get_post(cx, id).await });

    view! { cx,
        <Transition
            fallback=move || view! { cx, <SkeletonArticle /> }
        >
            {move || match post.read(cx) {
                Some(Ok(data)) => view! { cx,
                    <>
                        <Title text=data.frontmatter.title.clone() />
                        <article
                            class="prose prose-stone prose-lg mx-auto max-w-[unset] [&>*:not(.markdown-content)]:max-w-[50rem] [&>*]:mx-auto"
                        >
                            <div>
                                <h1 class="break-words hyphens-auto mb-8">
                                    {data.frontmatter.title.clone()}
                                </h1>
                                {data.frontmatter.tags.map(|tags| {
                                    view! { cx,
                                        <div class="flex items-center gap-2">
                                            {tags.into_iter().map(|t| {
                                                Some(view! { cx,
                                                        <span
                                                            class="bg-stone-200 rounded-full px-2 py-0.5 text-xs font-semibold"
                                                        >
                                                            {t}
                                                        </span>
                                                })
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    }
                                })}
                            </div>
                            {data.toc.map(|toc| view! { cx,
                                <div
                                    class="not-prose text-stone-900 max-w-[50rem] mx-auto p-4 mt-10 mb-8 bg-stone-200 rounded"
                                    inner_html=toc
                                />
                            })}
                            <div
                                class="markdown-content [&>*:not(.code-block)]:max-w-[50rem] [&>*]:mx-auto"
                                inner_html=data.content
                            />
                        </article>
                    </>
                }.into_view(cx),
                _ => view! { cx, <></> }.into_view(cx)
            }}
        </Transition>
    }
}
