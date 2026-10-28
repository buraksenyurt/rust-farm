use crate::filter::Filters;
use crate::stats::{EndpointStat, HourBucket};
use leptos::prelude::*;

/// 24 hour request histogram (pure CSS bars).
#[component]
pub fn HourlyChart(#[prop(into)] buckets: Signal<[HourBucket; 24]>) -> impl IntoView {
    let max = move || {
        buckets
            .read()
            .iter()
            .map(|b| b.total)
            .max()
            .unwrap_or(0)
            .max(1)
    };
    view! {
        <div class="panel">
            <h3>"Saatlik dağılım"</h3>
            <div class="bars">
                {(0..24)
                    .map(|h| {
                        let bucket = move || buckets.read()[h];
                        let height = move || bucket().total as f64 * 100.0 / max() as f64;
                        let err_height = move || {
                            let b = bucket();
                            if b.total == 0 { 0.0 } else { b.errors as f64 * 100.0 / b.total as f64 }
                        };
                        view! {
                            <div
                                class="bar"
                                title=move || {
                                    let b = bucket();
                                    format!("{h:02}:00 · {} istek · {} adet 5xx", b.total, b.errors)
                                }
                            >
                                <div class="fill" style:height=move || format!("{}%", height())>
                                    <div class="err" style:height=move || format!("{}%", err_height())></div>
                                </div>
                                <span>{format!("{h:02}")}</span>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}

/// Most busy endpoints. Clicking a row fills the path filter.
#[component]
pub fn TopEndpoints(#[prop(into)] items: Signal<Vec<EndpointStat>>) -> impl IntoView {
    let filters = expect_context::<Filters>();
    view! {
        <div class="panel">
            <h3>"En yoğun endpoint'ler"</h3>
            <table class="compact">
                <thead>
                    <tr>
                        <th>"Path"</th>
                        <th class="num">"İstek"</th>
                        <th class="num">"5xx"</th>
                        <th class="num">"Ort."</th>
                    </tr>
                </thead>
                <tbody>
                    {move || {
                        items
                            .get()
                            .into_iter()
                            .map(|item| {
                                let path = item.path.clone();
                                view! {
                                    <tr class="clickable" on:click=move |_| filters.path.set(path.clone())>
                                        <td class="mono">{item.path}</td>
                                        <td class="num">{item.count}</td>
                                        <td class="num s5">{item.errors}</td>
                                        <td class="num">{format!("{:.0} ms", item.avg_ms)}</td>
                                    </tr>
                                }
                            })
                            .collect_view()
                    }}
                </tbody>
            </table>
        </div>
    }
}
