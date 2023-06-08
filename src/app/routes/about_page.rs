use leptos::*;

#[component]
pub fn AboutPage(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <article class="prose prose-xl">
            <header class="font-sans">
                <h1 class="mb-0">"Jakob"<br />"Ankarhem"</h1>
                <p class="mt-1 text-stone-400">"Frontend Developer"</p>
            </header>
            <p>
                "I like to build things for the web. Doing frontend development for a living, and in my spare time I'm a big fan of Rust."
            </p>
            <p>
                "I have a passion for programming, and I always have my hands in a few different projects. Building new things either because they don't exist yet, to learn new tech or simply because it's fun."
            </p>
            <p>
                "You can find me on GitHub, Twitter and Email."
            </p>
        </article>
    }
}
