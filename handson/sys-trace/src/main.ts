import { invoke } from "@tauri-apps/api/core";
import "@material/web/all.js";

const REFRESH_MS = 1000;

type CpuCore = { name: string; usage: number; frequency: number };
type MemorySnapshot = {
  total: number; used: number; available: number;
  swapTotal: number; swapUsed: number;
};
type DiskSnapshot = {
  name: string; mountPoint: string; fileSystem: string; kind: string;
  totalSpace: number; availableSpace: number;
};
type Metrics = {
  globalCpu: number; cpuCores: CpuCore[]; memory: MemorySnapshot;
  disks: DiskSnapshot[];
};
type HostInfo = {
  name: string; os: string; kernelVersion: string; architecture: string;
  cpuBrand: string; physicalCores: number | null; logicalCores: number;
};

const $ = (id: string) => document.getElementById(id)!;

function gib(bytes: number): string {
  return (bytes / 1024 ** 3).toFixed(1) + " GiB";
}

function band(pct: number): string {
  return pct < 50 ? "calm" : pct < 85 ? "warm" : "hot";
}

function meter(pct: number, label: string, detail: string): string {
  const clamped = Math.min(100, Math.max(0, pct));
  const state = band(clamped);
  const color = state === "calm" ? "#4caf50" : state === "warm" ? "#ff9800" : "#f44336";
  return `
    <div style="display:grid; grid-template-columns: minmax(120px, 1fr) minmax(130px, 1fr) auto; align-items:center; gap:6px; margin-bottom:4px;">
      <div style="overflow:hidden; text-overflow:ellipsis; white-space:nowrap;">${label}</div>
      <md-linear-progress value="${(clamped / 100).toFixed(3)}" style="--md-linear-progress-active-indicator-color:${color};"></md-linear-progress>
      <div style="font-variant-numeric: tabular-nums; white-space:nowrap; font-size:12px;">${detail} · ${clamped.toFixed(1)}%</div>
    </div>`;
}

function coreMeter(c: CpuCore): string {
  const clamped = Math.min(100, Math.max(0, c.usage));
  const state = band(clamped);
  const color = state === "calm" ? "#4caf50" : state === "warm" ? "#ff9800" : "#f44336";
  return `
    <div title="${c.name} · ${c.usage.toFixed(1)}% @ ${c.frequency} MHz" style="display:flex; flex-direction:column; align-items:center; width:20px;">
      <div style="height:92px; width:14px; border-radius:6px; background:#e0e0e0; display:flex; align-items:flex-end; overflow:hidden;">
        <div style="width:100%; height:${clamped}%; background:${color};"></div>
      </div>
      <div style="font-size:11px; margin-top:3px;">${c.name.replace(/^cpu/i, "").trim() || c.name}</div>
    </div>`;
}

function renderMessage(targetId: string, message: string): void {
  $(targetId).innerHTML = `<p>${message}</p>`;
}

function renderHost(info: HostInfo) {
  $("hostname").textContent = info.name;
  const cores = info.physicalCores
    ? `${info.physicalCores} cores / ${info.logicalCores} threads`
    : `${info.logicalCores} threads`;
  $("hostmeta").textContent =
    `${info.cpuBrand} · ${cores} · ${info.os} · kernel ${info.kernelVersion} · ${info.architecture}`;
}

function render(m: Metrics) {
  $("cpu-total").textContent = m.globalCpu.toFixed(1) + "%";
  $("cores").innerHTML = `
    <div style="display:flex; align-items:flex-end; gap:6px; overflow-x:auto; padding-bottom:2px;">
      ${m.cpuCores.map(coreMeter).join("")}
    </div>`;
  $("loadavg").textContent = "";

  const memPct = (m.memory.used / m.memory.total) * 100;
  $("mem-total").textContent = `${gib(m.memory.used)} / ${gib(m.memory.total)}`;
  const swapPct = m.memory.swapTotal
    ? (m.memory.swapUsed / m.memory.swapTotal) * 100
    : 0;
  $("memory").innerHTML =
    meter(memPct, "RAM", `${gib(m.memory.used)} / ${gib(m.memory.total)} · ${gib(m.memory.available)} free`) +
    (m.memory.swapTotal > 0
      ? meter(swapPct, "swap", `${gib(m.memory.swapUsed)} / ${gib(m.memory.swapTotal)}`)
      : "");

  $("disks").innerHTML = m.disks
    .map((d) => {
      const used = d.totalSpace - d.availableSpace;
      const pct = (used / d.totalSpace) * 100;
      const tag = `${d.name} · ${d.mountPoint} · ${d.fileSystem} · ${d.kind}`;
      return meter(pct, tag, `${gib(used)} / ${gib(d.totalSpace)}`);
    })
    .join("");
}

async function tick() {
  try {
    render(await invoke<Metrics>("get_metrics"));
  } catch (err) {
    $("hostmeta").textContent = `sampling failed: ${err}`;
    renderMessage("cores", "Unable to fetch processor metrics.");
    renderMessage("memory", "Unable to fetch memory metrics.");
    renderMessage("disks", "Unable to fetch storage metrics.");
  }
  setTimeout(tick, REFRESH_MS);
}

async function start() {
  try {
    renderHost(await invoke<HostInfo>("get_host_info"));
  } catch (err) {
    $("hostmeta").textContent = `could not read host info: ${err}`;
  }
  tick();
}

start();