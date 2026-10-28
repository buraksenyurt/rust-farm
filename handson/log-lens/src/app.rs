use crate::components::{
    filters::FilterBar,
    insights::{HourlyChart, TopEndpoints},
    stats::StatsPanel,
    table::RequestTable,
    upload::FileDrop,
};
use crate::filter::Filters;
use crate::parser::{LogEntry, ParseResult};
use crate::stats::{hourly, summarize, top_endpoints};
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos::task::spawn_local;

// Log file that has been loaded and parsed
#[derive(Clone)]
pub struct Dataset {
    pub id: u32,
    pub name: String,
    pub entries: Vec<LogEntry>,
    pub failed: usize,
    pub parse_ms: f64,
}

const CHUNK_LINES: usize = 50_000;

fn now_ms() -> f64 {
    window().performance().map(|p| p.now()).unwrap_or(0.0)
}

#[component]

pub fn App() -> impl IntoView {
    let dataset = RwSignal::new(None::<Dataset>);
    let load_count = StoredValue::new(0u32);
    let progress = RwSignal::new(None::<f64>);
    let on_load = Callback::new(move |(name, text): (String, String)| {
        if progress.get_untracked().is_some() {
            return; // zaten bir dosya işleniyor
        }

        progress.set(Some(0.0));
        spawn_local(async move {
            let start = now_ms();
            let total = text.len().max(1);
            let mut consumed = 0;
            let mut result = ParseResult::default();
            let mut lines = text.lines();
            loop {
                let mut count = 0;
                for line in lines.by_ref().take(CHUNK_LINES) {
                    consumed += line.len() + 1;
                    result.push_line(line);
                    count += 1;
                }
                if count < CHUNK_LINES {
                    break;
                }
                progress.set(Some(consumed as f64 / total as f64));
                TimeoutFuture::new(0).await;
            }
            load_count.update_value(|c| *c += 1);
            dataset.set(Some(Dataset {
                id: load_count.get_value(),
                name,
                entries: result.entries,
                failed: result.failed,
                parse_ms: now_ms() - start,
            }));
            progress.set(None);
        });
    });

    let filters = Filters::new();
    provide_context(filters);
    let filtered = Memo::new(move |_| {
        let criteria = filters.criteria();
        dataset.with(|d| match d {
            Some(d) => d
                .entries
                .iter()
                .enumerate()
                .filter(|(_, e)| criteria.matches(e))
                .map(|(i, _)| i)
                .collect::<Vec<usize>>(),
            None => Vec::new(),
        })
    });

    let summary = Memo::new(move |_| {
        dataset.with(|d| match d {
            Some(d) => filtered.with(|idx| summarize(idx.iter().map(|&i| &d.entries[i]))),
            None => Default::default(),
        })
    });

    let hours = Memo::new(move |_| {
        dataset.with(|d| match d {
            Some(d) => filtered.with(|idx| hourly(idx.iter().map(|&i| &d.entries[i]))),
            None => Default::default(),
        })
    });

    let endpoints = Memo::new(move |_| {
        dataset.with(|d| match d {
            Some(d) => filtered.with(|idx| top_endpoints(idx.iter().map(|&i| &d.entries[i]), 8)),
            None => Vec::new(),
        })
    });

    view! {
        <header>
            <h1>"Log Lens"</h1>
            <p>"Nginx log'larını tarayıcında analiz et. Dosyan hiçbir sunucuya gönderilmez."</p>
        </header>
        <main>
            <FileDrop on_load=on_load />
            {move || progress.get().map(|p| view! {
                <div class="progress">
                    <div class="progress-bar" style:width=format!("{:.0}%", p * 100.0)></div>
                    <span>{format!("Ayrıştırılıyor… %{:.0}", p * 100.0)}</span>
                </div>
            })}
            <Show when=move || dataset.with(|d| d.is_some())>
                {move || dataset.with(|d| d.as_ref().map(|d| view! {
                    <p class="meta">
                        <b>{d.name.clone()}</b>
                        {format!(" · {} kayıt · {} hatalı satır · {:.1} ms'de ayrıştırıldı",
                            d.entries.len(), d.failed, d.parse_ms)}
                    </p>
                }))}
                <FilterBar matched=Signal::derive(move || filtered.with(|v| v.len())) />
                <StatsPanel summary=summary />
                <section class="insights">
                    <HourlyChart buckets=hours />
                    <TopEndpoints items=endpoints />
                </section>
                <RequestTable dataset=dataset rows=filtered />
            </Show>
        </main>
    }
}
