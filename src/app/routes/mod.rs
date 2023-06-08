mod about_page;
mod home_page;

use super::footer::Footer;
use super::header::Header;
use about_page::AboutPage;
use home_page::HomePage;

use leptos::*;
use leptos_router::*;

#[component]
pub fn AppRouter(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <Router>
            <Header/>
            <main class="max-w-prose mx-auto mx-auto mt-10 mb-20 px-4 sm:px-8">
            <Routes>
                    <Route path="/" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="/about" view=|cx| view! { cx, <AboutPage/> }/>
            </Routes>
            </main>
            <Footer/>
        </Router>
    }
}
