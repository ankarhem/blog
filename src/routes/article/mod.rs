use leptos::*;
use leptos_router::*;

#[server(GetPost, "/api")]
pub async fn get_post(cx: Scope, id: String) -> Result<crate::posts::Post, ServerFnError> {
    use crate::posts::*;
    println!("get_post: {}", id);

    let post = get_post(&id)?;
    let parsed = post.parse_markdown()?;
    Ok(parsed)
}

#[component]
pub fn ArticlePage(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    // id: || -> usize
    let post_id = move || params.with(|params| params.get("post_id").cloned().unwrap_or_default());
    let post = create_resource(cx, post_id, move |id| async move { get_post(cx, id).await });

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
                _ => view! { cx, <></> }.into_view(cx)
            }}
        </Transition>
    }
}
