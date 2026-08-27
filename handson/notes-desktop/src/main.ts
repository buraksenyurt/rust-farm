import { invoke } from "@tauri-apps/api/core";
import "@material/web/all.js";

type MediaType = "Gazete" | "Dergi" | "Dijital" | "Kitap" | "Podcast" | "Medium" | "Unknown";

interface ExternalLink {
  title: string;
  url: string;
}

interface Note {
  id: number;
  title: string;
  body: string;
  publisher: string;
  author: string;
  mediaType: MediaType;
  year: number;
  month: string;
  day: number;
  externals: ExternalLink[];
  isArchived: boolean;
}

interface NoteInput {
  title: string;
  body: string;
  publisher: string;
  author: string;
  mediaType: MediaType;
  year: number;
  month: string;
  day: number;
  externals: ExternalLink[];
}

const MEDIA_TYPES: MediaType[] = ["Gazete", "Dergi", "Dijital", "Kitap", "Podcast", "Medium", "Unknown"];
const MONTHS = [
  "Ocak", "Şubat", "Mart", "Nisan", "Mayıs", "Haziran",
  "Temmuz", "Ağustos", "Eylül", "Ekim", "Kasım", "Aralık",
];

type ViewName = "home" | "add" | "list" | "detail";
type SortColumn = "title" | "author" | "id" | "date";

const $ = (id: string) => document.getElementById(id)!;

function escapeHtml(value: string): string {
  const div = document.createElement("div");
  div.textContent = value;
  return div.innerHTML;
}

// --- Backend çağrıları -----------------------------------------------------

async function fetchRandomNote(): Promise<Note | null> {
  return invoke<Note | null>("get_random_note");
}

async function fetchNotesSorted(column: SortColumn, order: "asc" | "desc"): Promise<Note[]> {
  return invoke<Note[]>("list_notes_sorted", { column, order });
}

async function fetchNote(id: number): Promise<Note | null> {
  return invoke<Note | null>("get_note", { id });
}

async function createNote(input: NoteInput): Promise<Note> {
  return invoke<Note>("add_note", { input });
}

async function archiveNoteById(id: number): Promise<void> {
  await invoke("archive_note", { id });
}

// --- Görünüm geçişleri -------------------------------------------------------

function showView(name: ViewName) {
  (["home", "add", "list", "detail"] as ViewName[]).forEach((v) => {
    $(`view-${v}`).style.display = v === name ? "block" : "none";
  });
}

// --- Not kartı render'ı (ana sayfa ve detay ekranında ortak) ----------------

function noteDateText(note: Note): string {
  if (!note.year && !note.day) {
    return "Tarih bilgisi yok";
  }
  const day = note.day > 0 ? `${note.day} ` : "";
  const month = note.month ? `${note.month} ` : "";
  return `${day}${month}${note.year || ""}`.trim();
}

function renderExternalsList(externals: ExternalLink[]): string {
  if (!externals.length) return "";
  const items = externals
    .map((e) => `<li><a href="${escapeHtml(e.url)}" target="_blank" rel="noopener">${escapeHtml(e.title)}</a></li>`)
    .join("");
  return `<h3 style="margin:12px 0 4px 0;">Dış Bağlantılar</h3><ul style="margin:0; padding-left:20px;">${items}</ul>`;
}

function noteCardHtml(note: Note, opts: { withArchive: boolean }): string {
  const archiveButton = opts.withArchive
    ? `<md-outlined-button id="archive-btn" style="margin-top:12px;">Arşivle</md-outlined-button>`
    : "";
  return `
    <md-elevated-card style="display:block; padding:16px;">
      <span style="font-size:12px; text-transform:uppercase; opacity:0.7;">${escapeHtml(note.mediaType)}</span>
      <h2 style="margin:4px 0 8px 0;">${escapeHtml(note.title)}</h2>
      <div>${note.body}</div>
      <p style="margin:12px 0 0 0; opacity:0.8;">${escapeHtml(note.author)} · ${escapeHtml(note.publisher)} · ${escapeHtml(noteDateText(note))}</p>
      ${renderExternalsList(note.externals)}
      ${archiveButton}
    </md-elevated-card>`;
}

// --- Ana sayfa ---------------------------------------------------------------

async function loadHome() {
  const container = $("home-card");
  container.innerHTML = "<p>Yükleniyor…</p>";
  try {
    const note = await fetchRandomNote();
    container.innerHTML = note
      ? noteCardHtml(note, { withArchive: false })
      : "<p>Henüz bir not bulunmuyor.</p>";
  } catch (err) {
    container.innerHTML = `<p>Not yüklenemedi: ${escapeHtml(String(err))}</p>`;
  }
}

// --- Not ekleme formu ---------------------------------------------------------

function populateSelect(id: string, options: string[]) {
  const select = $(id) as any;
  select.innerHTML = options
    .map((o) => `<md-select-option value="${o}"><div slot="headline">${o}</div></md-select-option>`)
    .join("");
  select.value = options[0];
}

function addExternalRow(title = "", url = "") {
  const rows = $("externals-rows");
  const row = document.createElement("div");
  row.style.display = "flex";
  row.style.gap = "8px";
  row.style.alignItems = "center";
  row.innerHTML = `
    <md-outlined-text-field class="ext-title" label="Bağlantı Başlığı" style="flex:1;" value="${escapeHtml(title)}"></md-outlined-text-field>
    <md-outlined-text-field class="ext-url" label="URL" style="flex:1;" value="${escapeHtml(url)}"></md-outlined-text-field>
    <md-text-button type="button" class="remove-row">Kaldır</md-text-button>`;
  row.querySelector(".remove-row")!.addEventListener("click", () => row.remove());
  rows.appendChild(row);
}

function resetAddForm() {
  ($("f-title") as any).value = "";
  ($("f-body") as any).value = "";
  ($("f-publisher") as any).value = "";
  ($("f-author") as any).value = "";
  ($("f-day") as any).value = "1";
  ($("f-year") as any).value = String(new Date().getFullYear());
  ($("f-media-type") as any).value = MEDIA_TYPES[0];
  ($("f-month") as any).value = MONTHS[0];
  $("externals-rows").innerHTML = "";
}

function setupAddForm() {
  populateSelect("f-media-type", MEDIA_TYPES);
  populateSelect("f-month", MONTHS);

  $("add-external-row").addEventListener("click", () => addExternalRow());

  $("add-form").addEventListener("submit", async (event) => {
    event.preventDefault();
    const message = $("add-message");
    message.innerHTML = "";

    const externals: ExternalLink[] = Array.from(document.querySelectorAll("#externals-rows > div"))
      .map((row) => {
        const title = (row.querySelector(".ext-title") as any).value.trim();
        const url = (row.querySelector(".ext-url") as any).value.trim();
        return { title, url };
      })
      .filter((e) => e.title && e.url);

    const input: NoteInput = {
      title: ($("f-title") as any).value.trim(),
      body: ($("f-body") as any).value.trim(),
      publisher: ($("f-publisher") as any).value.trim(),
      author: ($("f-author") as any).value.trim(),
      mediaType: ($("f-media-type") as any).value,
      year: Number(($("f-year") as any).value) || 0,
      month: ($("f-month") as any).value,
      day: Number(($("f-day") as any).value) || 0,
      externals,
    };

    if (!input.title || !input.body) {
      message.innerHTML = `<p style="color:#f44336;">Başlık ve içerik zorunludur.</p>`;
      return;
    }

    try {
      await createNote(input);
      message.innerHTML = `<p style="color:#4caf50;">Not başarıyla eklendi.</p>`;
      resetAddForm();
    } catch (err) {
      message.innerHTML = `<p style="color:#f44336;">Not eklenemedi: ${escapeHtml(String(err))}</p>`;
    }
  });
}

// --- Tüm notlar listesi --------------------------------------------------------

let sortState: { column: SortColumn; order: "asc" | "desc" } = { column: "title", order: "asc" };

async function loadList() {
  const body = $("notes-table-body");
  body.innerHTML = `<tr><td colspan="5" style="padding:8px;">Yükleniyor…</td></tr>`;
  try {
    const notes = await fetchNotesSorted(sortState.column, sortState.order);
    body.innerHTML = notes
      .map(
        (n) => `
      <tr style="border-bottom:1px solid var(--md-sys-color-outline-variant, #eee);">
        <td style="padding:8px;"><a href="#" class="note-link" data-id="${n.id}">${escapeHtml(n.title)}</a></td>
        <td style="padding:8px;">${escapeHtml(n.author)}</td>
        <td style="padding:8px;">${escapeHtml(n.publisher)}</td>
        <td style="padding:8px;">${escapeHtml(noteDateText(n))}</td>
        <td style="padding:8px;">${n.id}</td>
      </tr>`
      )
      .join("") || `<tr><td colspan="5" style="padding:8px;">Kayıt bulunamadı.</td></tr>`;

    body.querySelectorAll(".note-link").forEach((link) => {
      link.addEventListener("click", (event) => {
        event.preventDefault();
        const id = Number((link as HTMLElement).dataset.id);
        openDetail(id);
      });
    });
  } catch (err) {
    body.innerHTML = `<tr><td colspan="5" style="padding:8px;">Liste yüklenemedi: ${escapeHtml(String(err))}</td></tr>`;
  }
}

function setupSortLinks() {
  document.querySelectorAll<HTMLAnchorElement>(".sort-link").forEach((link) => {
    link.addEventListener("click", (event) => {
      event.preventDefault();
      const column = link.dataset.sort as SortColumn;
      sortState = sortState.column === column
        ? { column, order: sortState.order === "asc" ? "desc" : "asc" }
        : { column, order: "asc" };
      loadList();
    });
  });
}

// --- Not detayı --------------------------------------------------------------

async function openDetail(id: number) {
  showView("detail");
  const container = $("detail-card");
  container.innerHTML = "<p>Yükleniyor…</p>";
  try {
    const note = await fetchNote(id);
    if (!note) {
      container.innerHTML = "<p>Aranan not bilgisi bulunamadı.</p>";
      return;
    }
    container.innerHTML = noteCardHtml(note, { withArchive: true });
    $("archive-btn").addEventListener("click", async () => {
      if (!confirm("Bu notu arşivlemek istediğinize emin misiniz?")) return;
      try {
        await archiveNoteById(note.id);
        showView("list");
        await loadList();
      } catch (err) {
        alert(`Not arşivlenemedi: ${err}`);
      }
    });
  } catch (err) {
    container.innerHTML = `<p>Not yüklenemedi: ${escapeHtml(String(err))}</p>`;
  }
}

// --- Navigasyon ----------------------------------------------------------------

function setupNav() {
  $("nav-home").addEventListener("click", () => {
    showView("home");
    loadHome();
  });
  $("nav-add").addEventListener("click", () => showView("add"));
  $("nav-list").addEventListener("click", () => {
    showView("list");
    loadList();
  });
  $("home-refresh").addEventListener("click", loadHome);
}

// --- Başlangıç -----------------------------------------------------------------

function start() {
  setupNav();
  setupAddForm();
  setupSortLinks();
  showView("home");
  loadHome();
}

start();
