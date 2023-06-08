mod footer;
mod header;
mod icons;
mod routes;

use leptos::*;
use leptos_meta::*;
use routes::AppRouter;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/blog_leptos.css"/>

        // sets the document title
        <Title text="Ankarhem.dev"/>

        <AppRouter />
    }
}
