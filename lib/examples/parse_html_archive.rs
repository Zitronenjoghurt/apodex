use apodex::archiving::html::ArchiveHtml;
use apodex::archiving::Archive;
use apodex::parsing::quality_control::{quality_control, QualityWarning};
use apodex::parsing::{parse_html, ParseError};
use apodex::ApodEntry;
use chrono::NaiveDate;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

enum EntryResult {
    Ok {
        entry: ApodEntry,
        warnings: HashSet<QualityWarning>,
    },
    Err(ParseError),
}

struct AnalyzedEntry {
    date: NaiveDate,
    result: EntryResult,
}

impl AnalyzedEntry {
    pub fn link(&self) -> String {
        format!(
            "https://apod.nasa.gov/apod/ap{}.html",
            self.date.format("%y%m%d")
        )
    }
}

fn main() {
    let archive_path = PathBuf::from("../../html_archive.bin");
    let archive: Archive<ArchiveHtml> = Archive::load(&archive_path).unwrap();

    let mut entries: Vec<_> = archive
        .iter()
        .map(|(date, html_entry)| AnalyzedEntry {
            date: *date,
            result: match parse_html(*date, &html_entry.html) {
                Ok(entry) => EntryResult::Ok {
                    warnings: quality_control(&entry),
                    entry,
                },
                Err(e) => EntryResult::Err(e),
            },
        })
        .collect();

    entries.sort_by_key(|e| e.date);

    generate_html_report(&entries);
    println!("Generated report.html");
}

fn generate_html_report(entries: &[AnalyzedEntry]) {
    let total = entries.len();
    let errors = entries
        .iter()
        .filter(|e| matches!(e.result, EntryResult::Err(_)))
        .count();
    let warnings = entries
        .iter()
        .filter(|e| matches!(&e.result, EntryResult::Ok { warnings, .. } if !warnings.is_empty()))
        .count();
    let ok = total - errors - warnings;

    let rows: String = entries
        .iter()
        .map(|e| match &e.result {
            EntryResult::Ok { entry, warnings } => {
                let class = if warnings.is_empty() { "ok" } else { "warning" };
                let badges: String = warnings
                    .iter()
                    .map(|w| format!(r#"<span class="badge badge-warn">{}</span>"#, html_escape(&format!("{:?}", w))))
                    .collect();
                format!(
                    r#"<tr class="{}" data-status="{}"><td><a href="{}">{}</a></td><td class="title">{}</td><td>{}</td></tr>"#,
                    class, class, e.link(), e.date, html_escape(&entry.title), if badges.is_empty() { "✓".into() } else { badges }
                )
            }
            EntryResult::Err(err) => {
                format!(
                    r#"<tr class="error" data-status="error"><td>{}</td><td>—</td><td><span class="badge badge-err">{}</span></td></tr>"#,
                    e.date, err
                )
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <style>
        body {{ font-family: system-ui; max-width: 1400px; margin: 0 auto; padding: 20px; }}
        table {{ width: 100%; border-collapse: collapse; margin-top: 16px; }}
        th, td {{ border: 1px solid #ddd; padding: 8px; text-align: left; }}
        tr:hover {{ background: #f5f5f5; }}
        .ok {{ background: #d4edda; }}
        .warning {{ background: #fff3cd; }}
        .error {{ background: #f8d7da; }}
        .title {{ font-family: monospace; white-space: pre-wrap; }}
        .badge {{ padding: 2px 6px; border-radius: 4px; font-size: 12px; margin-right: 4px; }}
        .badge-warn {{ background: #ffc107; }}
        .badge-err {{ background: #dc3545; color: white; }}
        .controls {{ margin-bottom: 16px; display: flex; gap: 16px; align-items: center; flex-wrap: wrap; }}
        input[type="text"] {{ padding: 8px; min-width: 300px; }}
        .stats {{ color: #666; }}
        .stats span {{ margin-right: 16px; }}
        .count-ok {{ color: #28a745; }}
        .count-warn {{ color: #856404; }}
        .count-err {{ color: #dc3545; }}
    </style>
</head>
<body>
    <h1>APOD Quality Report</h1>

    <div class="controls">
        <input type="text" id="filter" placeholder="Filter by date or title..." onkeyup="applyFilters()">
        <select id="statusFilter" onchange="applyFilters()">
            <option value="all">All</option>
            <option value="ok">OK only</option>
            <option value="warning">Warnings only</option>
            <option value="error">Errors only</option>
            <option value="issues">Warnings + Errors</option>
        </select>
    </div>

    <p class="stats">
        <span>Total: {}</span>
        <span class="count-ok">OK: {}</span>
        <span class="count-warn">Warnings: {}</span>
        <span class="count-err">Errors: {}</span>
    </p>

    <table>
        <thead><tr><th style="width: 120px">Date</th><th>Title</th><th style="width: 300px">Status</th></tr></thead>
        <tbody>
{}
        </tbody>
    </table>

    <script>
        function applyFilters() {{
            const text = document.getElementById('filter').value.toLowerCase();
            const status = document.getElementById('statusFilter').value;
            document.querySelectorAll('tbody tr').forEach(row => {{
                const matchesText = row.textContent.toLowerCase().includes(text);
                const rowStatus = row.dataset.status;
                const matchesStatus = status === 'all'
                    || status === rowStatus
                    || (status === 'issues' && (rowStatus === 'warning' || rowStatus === 'error'));
                row.style.display = matchesText && matchesStatus ? '' : 'none';
            }});
        }}
    </script>
</body>
</html>"#,
        total, ok, warnings, errors, rows
    );

    File::create("../../report.html")
        .unwrap()
        .write_all(html.as_bytes())
        .unwrap();
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
