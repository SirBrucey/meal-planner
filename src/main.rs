use leptos::*;

mod state;

use state::State;

#[component]
fn App(cx: Scope) -> impl IntoView {
    provide_context(cx, create_rw_signal(cx, State::new()));

    view! {cx, <h1>"Hello World"</h1>}
}

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}
