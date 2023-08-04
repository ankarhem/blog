mod footer;
mod header;
// pub mod response;
mod posts;
mod routes;
use footer::*;
use header::*;

use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
      use leptos::*;

      console_error_panic_hook::set_once();

      leptos::mount_to_body(move |cx| {
          view! { cx, <App/> }
      });
    }
}
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // Fonts
        <Link rel="preconnect" href="https://fonts.googleapis.com" />
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="true" />
        <Link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600&display=swap" rel="stylesheet" />
        <Link rel="preconnect" href="https://iosevka-webfonts.github.io" />
        <Link href="https://iosevka-webfonts.github.io/iosevka/iosevka.css" rel="stylesheet" />


        // sets the document title
        <Title formatter=|title| format!("{title} | Ankarhem.dev") />

        // content for this welcome page
        <Router>
            <Header />
            <main
                class="pt-10 pb-20 min-h-[calc(100vh_-_110px)] px-4 sm:px-8"
            >
                <Routes>
                    <Route path="/" view=routes::HomePage/>
                    <Route path="/articles" view=routes::ArticlesPage/>
                    <Route path="/articles/:post_id" view=routes::ArticlePage/>
                </Routes>
            </main>
            <Footer />
        </Router>
    }
}
