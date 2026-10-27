use crate::components::{stats::StatsPanel, upload::FileDrop};
use crate::parser::{LogEntry, parse_all};
use crate::stats::summarize;
use leptos::prelude::*;

// Log file that has been loaded and parsed
#[derive(Clone)]
pub struct Dataset {
    pub name: String,
    pub entries: Vec<LogEntry>,
    pub failed: usize,
    pub parse_ms: f64,
}

fn now_ms() -> f64 {
    window().performance().map(|p| p.now()).unwrap_or(0.0)
}

#[component]
pub fn App() -> impl IntoView {
    let dataset = RwSignal::new(None::<Dataset>);
    let on_load = Callback::new(move |(name, text): (String, String)| {
        let start = now_ms();
        let result = parse_all(&text);
        dataset.set(Some(Dataset {
            name,
            entries: result.entries,
            failed: result.failed,
            parse_ms: now_ms() - start,
        }));
    });

    let summary = Memo::new(move |_| {
        dataset.with(|d| {
            d.as_ref()
                .map(|d| summarize(d.entries.iter()))
                .unwrap_or_default()
        })
    });

    view! {
        <header>
            <h1>"Log Lens"</h1>
            <p>"Nginx log'larını tarayıcında analiz et. Dosyan hiçbir sunucuya gönderilmez."</p>
        </header>
        <main>
            <FileDrop on_load=on_load />
            <Show when=move || dataset.with(|d| d.is_some())>
                {move || dataset.with(|d| d.as_ref().map(|d| view! {
                    <p class="meta">
                        <b>{d.name.clone()}</b>
                        {format!(" · {} kayıt · {} hatalı satır · {:.1} ms'de ayrıştırıldı",
                            d.entries.len(), d.failed, d.parse_ms)}
                    </p>
                }))}
                <StatsPanel summary=summary />
            </Show>
        </main>
    }
}
