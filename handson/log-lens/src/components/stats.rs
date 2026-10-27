use crate::stats::{Summary, human_bytes};
use leptos::prelude::*;

/// A single statistic card.
#[component]
fn StatCard(
    label: &'static str,
    #[prop(into)] value: Signal<String>,
    #[prop(optional)] tone: &'static str,
) -> impl IntoView {
    view! {
        <div class=format!("card {tone}")>
            <span class="label">{label}</span>
            <span class="value">{value}</span>
        </div>
    }
}

fn fmt_ms(v: Option<f64>) -> String {
    v.map(|v| format!("{v:.0} ms"))
        .unwrap_or_else(|| "—".into())
}

/// The panel displaying all statistics.
#[component]
pub fn StatsPanel(#[prop(into)] summary: Signal<Summary>) -> impl IntoView {
    view! {
        <section class="cards">
            <StatCard label="İstek" value=move || summary.read().total.to_string() />
            <StatCard
                label="Hata oranı"
                tone="bad"
                value=move || format!("%{:.1}", summary.read().error_rate())
            />
            <StatCard label="5xx" tone="bad" value=move || summary.read().server_errors.to_string() />
            <StatCard label="Tekil IP" value=move || summary.read().unique_ips.to_string() />
            <StatCard label="Trafik" value=move || human_bytes(summary.read().total_bytes) />
            <StatCard label="Ort. süre" value=move || fmt_ms(summary.read().avg_ms) />
            <StatCard label="p95 süre" tone="warn" value=move || fmt_ms(summary.read().p95_ms) />
        </section>
    }
}
