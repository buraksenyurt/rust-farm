use crate::filter::Filters;
use leptos::prelude::*;

#[component]
pub fn FilterBar(#[prop(into)] matched: Signal<usize>) -> impl IntoView {
    let f = expect_context::<Filters>();
    view! {
        <section class="filters">
            <label>
                "Status"
                <select bind:value=f.status>
                    <option value="">"Tümü"</option>
                    <option value="2">"2xx"</option>
                    <option value="3">"3xx"</option>
                    <option value="4">"4xx"</option>
                    <option value="5">"5xx"</option>
                </select>
            </label>
            <label>
                "Method"
                <select bind:value=f.method>
                    <option value="">"Tümü"</option>
                    <option>"GET"</option>
                    <option>"POST"</option>
                    <option>"PUT"</option>
                    <option>"DELETE"</option>
                </select>
            </label>
            <label class="grow">
                "Path içerir"
                <input type="search" placeholder="/api/..." bind:value=f.path />
            </label>
            <label>
                "Min. süre (ms)"
                <input type="number" min="0" step="50" placeholder="0" bind:value=f.min_ms />
            </label>
            <button on:click=move |_| f.reset()>"Temizle"</button>
            <span class="matched">{move || format!("{} eşleşme", matched.get())}</span>
        </section>
    }
}
