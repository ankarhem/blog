use leptos::*;
use leptos_router::*;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! { cx,
        <header class="shadow-sm font-inter font-light">
            <nav class="container mx-auto">
                <ul class="flex items-center justify-center gap-4">
                    <li>
                        <A
                            exact=true
                            href="/"
                            class="block py-2 hover:underline aria-[current=page]:underline aria-[current=page]:decoration-stone-400 decoration-2 decoration-stone-900 hover:aria-[current=page]:decoration-stone-900"
                        >
                            "Home"
                        </A>
                    </li>
                    <li>
                        <A
                            href="/articles/component-with-as"
                            class="block py-2 hover:underline aria-[current=page]:underline aria-[current=page]:decoration-stone-400 decoration-2 decoration-stone-900 hover:aria-[current=page]:decoration-stone-900"
                        >
                            "Articles"
                        </A>
                    </li>
                    <li>
                        <A
                            exact=true
                            href="/about"
                            class="block py-2 hover:underline aria-[current=page]:underline aria-[current=page]:decoration-stone-400 decoration-2 decoration-stone-900 hover:aria-[current=page]:decoration-stone-900"
                        >
                            "About"
                        </A>
                    </li>
                </ul>
            </nav>
        </header>
    }
}
