use super::icons::{EmailIcon, GithubIcon, TwitterIcon};
use leptos::*;

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
        <footer class="border-t border-stone-500 py-4 flex justify-center">
            <address>
            <ul class="flex items-center gap-4">
                <li>
                    <a href="https://github.com/ankarhem">
                        <GithubIcon class="p-0.5 inline fill-current align-middle hover:opacity-60".into()/>
                    </a>
                </li>
                <li>
                    <a href="https://twitter.com/ankarhem">
                        <TwitterIcon class="h-8 hover:opacity-60".into()/>
                    </a>
                </li>
                <li>
                    <a href="https://github.com/ankarhem">
                        <EmailIcon class="h-8 hover:opacity-60".into()/>
                    </a>
                </li>
            </ul>
            </address>
        </footer>
    }
}
