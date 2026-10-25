use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <button
            on:click=move |_| {
                 *set_count.write() += 1;
            }
             // the class: syntax reactively updates a single class
            // here, we'll set the `red` class when `count` is odd
            class:red=move || count.get() % 2 == 1
        >
            "Press the button: "
            {count}
        </button>
        <p>
            "Double count: "
            {move || count.get() * 2}
        </p>
    }
}
