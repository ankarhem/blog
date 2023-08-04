use leptos::*;

#[component]
pub fn SkeletonArticle(cx: Scope) -> impl IntoView {
    view! { cx,
        <article class="max-w-[50rem] mx-auto">
            <span class="block animate-pulse w-full h-12 bg-stone-200 rounded mb-8"></span>
            <div class="flex items-center gap-2">
                <span class="text-base bg-stone-200 animate-pulse rounded-full w-10 h-4"></span>
                <span class="text-base bg-stone-200 animate-pulse rounded-full w-16 h-4"></span>
            </div>
            <div class="w-full bg-stone-200 rounded mt-10 mb-8 p-4">
                <ul class="table-of-contents flex flex-col gap-4">
                    <span class="block toc-entry level-2 text-base bg-stone-300 animate-pulse rounded w-20 h-5"></span>
                    <span class="block toc-entry level-2 text-base bg-stone-300 animate-pulse rounded w-10 h-5"></span>
                    <span class="block toc-entry level-3 text-base bg-stone-300 animate-pulse rounded w-20 h-5"></span>
                    <span class="block toc-entry level-3 text-base bg-stone-300 animate-pulse rounded w-16 h-5"></span>
                </ul>
            </div>
        </article>
    }
}
