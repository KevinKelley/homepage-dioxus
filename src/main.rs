use leptos::*;

mod navbar;
mod blog;
mod todo;
mod calc;
mod about;

use navbar::NavBar;
use calc::Calc;
use about::About;



#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let double_count = move || count() * 2;

    view! {
        <NavBar/>

        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me"
        </button>
        <br/>
    }
}

fn main() {
    leptos::mount_to_body(App)
}