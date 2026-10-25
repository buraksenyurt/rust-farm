use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <h1>"Log Lens"</h1>
        <p>"Log dosyaları tarayıcı dışına çıkartılmadan analiz edilir."</p>
    }
}
