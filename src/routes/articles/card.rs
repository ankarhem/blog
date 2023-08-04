use crate::posts::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Card(cx: Scope, article: RawPost) -> impl IntoView {
    let title = article.frontmatter.title.clone();
    let excerpt = article.frontmatter.excerpt.clone();
    let tags = article.frontmatter.tags.clone();

    view! { cx,
        <li class="rounded overflow-hidden shadow-lg p-4 bg-stone-100 text-stone-900">
            <h2 class="mb-4 text-lg font-medium">
                <A href=article.id>
                    {title}
                </A>
            </h2>
            <p class="text-base text-stone-700">{excerpt}</p>
            <div class="flex items-center gap-2 mt-4">
                {tags.and_then(|tags| tags.into_iter().map(|t| {
                    Some(view! { cx,
                        <span
                            class="bg-stone-200 rounded-full px-2 py-0.5 text-xs font-semibold"
                        >
                            {t}
                        </span>
                    })
                }).collect::<Option<Vec<_>>>())}
            </div>
        </li>
    }
}
