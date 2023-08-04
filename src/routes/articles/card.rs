use crate::posts::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn SkeletonCard(cx: Scope) -> impl IntoView {
    view! { cx,
        <li class="rounded overflow-hidden shadow-lg p-4 bg-stone-100">
            <span class="block mb-6 animate-pulse bg-stone-200 rounded w-full h-6">
            </span>
            <div class="flex flex-col gap-2">
                <span class="text-base bg-stone-200 animate-pulse rounded w-11/12 h-4"></span>
                <span class="text-base bg-stone-200 animate-pulse rounded w-full h-4"></span>
                <span class="text-base bg-stone-200 animate-pulse rounded w-full h-4"></span>
                <span class="text-base bg-stone-200 animate-pulse rounded w-10/12 h-4"></span>
                <span class="text-base bg-stone-200 animate-pulse rounded w-1/2 h-4"></span>
            </div>
            <div class="flex items-center gap-2 mt-4">
                <span class="text-base bg-stone-200 animate-pulse rounded-full w-10 h-4"></span>
                <span class="text-base bg-stone-200 animate-pulse rounded-full w-10 h-4"></span>
            </div>
        </li>
    }
}

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
