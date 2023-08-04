use leptos::*;
use leptos_meta::*;
mod card;
use card::*;

#[server(GetPosts, "/api")]
pub async fn get_articles(cx: Scope) -> Result<Vec<crate::posts::RawPost>, ServerFnError> {
    use crate::posts::*;
    println!("get_articles");
    let posts = get_posts()?;

    // let ten_millis = std::time::Duration::from_millis(10000);
    // std::thread::sleep(ten_millis);

    Ok(posts)
}

#[component]
fn SkeletonArticles(cx: Scope) -> impl IntoView {
    view! { cx,
        <ul class="grid gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
            {(1..=4).map(|_| view! { cx,
                <SkeletonCard />
            }).collect::<Vec<_>>()}
        </ul>
    }
}

#[component]
pub fn ArticlesPage(cx: Scope) -> impl IntoView {
    let articles = create_blocking_resource(
        cx,
        move || (),
        move |_| async move { get_articles(cx).await },
    );

    view! { cx,
        <div class="container mx-auto max-w-5xl">
        <Suspense
            fallback=move || view! { cx, <SkeletonArticles /> }
        >
            {move || match articles.read(cx) {
                Some(Ok(data)) => view! { cx,
                    <Title text="Articles" />
                    <ul class="grid gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
                        {data.into_iter().map(|article| view! { cx,
                            <Card article=article />
                        }).collect::<Vec<_>>()}
                    </ul>
                }.into_view(cx),
                _ => view! { cx, <></> }.into_view(cx)
            }}
        </Suspense>
        </div>
    }
}
