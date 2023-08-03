use leptos::*;
mod card;
use card::*;

#[server(GetPosts, "/api")]
pub async fn get_articles(cx: Scope) -> Result<Vec<crate::posts::RawPost>, ServerFnError> {
    use crate::posts::*;
    println!("get_articles");
    let posts = get_posts()?;
    Ok(posts)
}

#[component]
pub fn ArticlesPage(cx: Scope) -> impl IntoView {
    let articles = create_resource(
        cx,
        move || (),
        move |_| async move { get_articles(cx).await },
    );
    view! { cx,
        <div class="container mx-auto max-w-5xl">
        <Transition
            fallback=move || view! { cx, <p>"Loading..."</p> }
        >
            {move || match articles.read(cx) {
                Some(Ok(data)) => view! { cx,
                    <ul class="grid gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
                        {data.into_iter().map(|article| view! { cx,
                            <Card article=article />
                        }).collect::<Vec<_>>()}
                    </ul>
                }.into_view(cx),
                _ => view! { cx, <></> }.into_view(cx)
            }}
        </Transition>
        </div>
    }
}
