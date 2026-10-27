use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{DragEvent, File, HtmlInputElement};

/// Select file and drag-and-drop area.
/// When the file is read, the `on_load` callback is called with (file name, content).
#[component]
pub fn FileDrop(on_load: Callback<(String, String)>) -> impl IntoView {
    let (dragging, set_dragging) = signal(false);
    let (error, set_error) = signal(None::<String>);

    let read_file = move |file: File| {
        let name = file.name();

        spawn_local(async move {
            match JsFuture::from(file.text()).await {
                Ok(js) => {
                    set_error.set(None);
                    on_load.run((name, js.as_string().unwrap_or_default()));
                }
                Err(_) => set_error.set(Some(format!("{name} okunamadı"))),
            }
        });
    };

    let on_change = move |ev: leptos::ev::Event| {
        let input: HtmlInputElement = ev.target().unwrap().unchecked_into();

        if let Some(file) = input.files().and_then(|list| list.get(0)) {
            read_file(file);
        }
    };

    let on_drop = move |ev: DragEvent| {
        ev.prevent_default();
        set_dragging.set(false);

        if let Some(file) = ev
            .data_transfer()
            .and_then(|dt| dt.files())
            .and_then(|list| list.get(0))
        {
            read_file(file);
        }
    };

    view! {
        <label
            class="drop-zone"
            class:dragging=dragging
            on:dragover=move |ev: DragEvent| {
                ev.prevent_default();
                set_dragging.set(true);
            }
            on:dragleave=move |_| set_dragging.set(false)
            on:drop=on_drop
        >
            <input type="file" accept=".log,.txt" on:change=on_change />
            <strong>"Log dosyasını buraya sürükle"</strong>
            <span>" ya da tıklayıp seç"</span>
        </label>

        {move || error.get().map(|e| view! { <p class="error">{e}</p> })}

    }
}
