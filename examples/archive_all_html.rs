use apodex::archiving::html::ArchiveHtml;
use apodex::archiving::Archive;
use apodex::client::reqwest::ReqwestClient;
use apodex::scraper::Scraper;
use chrono::NaiveDate;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let original_path: Option<PathBuf> = None;
    //let original_path = Some(PathBuf::from("./original.bin"));
    let backup_path = PathBuf::from("./output_backup.bin");
    let final_path = PathBuf::from("./output.bin");
    let backup_interval = 30;

    let client = ReqwestClient::new();
    let scraper = Scraper::new(client).with_delay(Duration::from_secs(2));

    let start = NaiveDate::from_ymd_opt(1995, 6, 16).unwrap();
    let end = NaiveDate::from_ymd_opt(2026, 1, 9).unwrap();
    let expected_days = (end - start).num_days() + 1;

    let mut archive = if let Some(path) = original_path {
        Archive::load(&path).unwrap()
    } else {
        Archive::default()
    };
    let existing_dates = archive.all_dates();

    let pb = ProgressBar::new(expected_days as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} (ETA: {eta}) | {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );
    pb.enable_steady_tick(Duration::from_millis(100));

    let mut fetched = 0u64;
    let mut existing = 0u64;
    let mut missing = 0u64;
    let mut errors = 0u64;

    let mut current_date = start;
    while current_date <= end {
        if existing_dates.contains(&current_date) {
            existing += 1;
        } else {
            match scraper.fetch_html(current_date).await {
                Ok(Some(html)) => {
                    fetched += 1;
                    archive.push(ArchiveHtml::new(current_date, html));
                }
                Ok(None) => {
                    missing += 1;
                    pb.println(format!("  ⚠ Missing: {}", current_date));
                }
                Err(err) => {
                    errors += 1;
                    pb.println(format!("  ✗ Error ({}): {}", current_date, err));
                    if let Err(err) = archive.save(&backup_path) {
                        pb.println(format!(" ✗ Failed to save archive backup: {err}"))
                    }
                }
            }
        }

        pb.set_message(format!(
            "At {current_date}, {existing} existing, {fetched} fetched, {missing} missing, {errors} errors)"
        ));

        pb.inc(1);
        let Some(next_date) = current_date.succ_opt() else {
            break;
        };
        current_date = next_date;

        let total = fetched + existing + missing + errors;
        if total.is_multiple_of(backup_interval)
            && let Err(err) = archive.save(&backup_path)
        {
            pb.println(format!(" ✗ Failed to save archive backup: {err}"))
        }
    }

    pb.set_message(format!("Compressing {} entries...", fetched));
    archive.save(&final_path).unwrap();

    pb.finish_with_message(format!(
        "Done! {fetched} fetched, {missing} missing, {errors} errors."
    ));
}
