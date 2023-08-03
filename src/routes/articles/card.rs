use crate::posts::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Card(cx: Scope, article: RawPost) -> impl IntoView {
    view! { cx,
        <li class="rounded overflow-hidden shadow-lg p-4 bg-stone-100">
            <h2>
                <A href=article.id>
                    {article.title}
                </A>
            </h2>
            <p class="text-base">Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.</p>
        </li>
    }
}
