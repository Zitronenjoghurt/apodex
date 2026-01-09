use apodex::archiving::html::ArchiveHtml;
use apodex::archiving::Archive;
use apodex::client::reqwest::ReqwestClient;
use apodex::scraper::Scraper;
use chrono::NaiveDate;
use futures::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let client = ReqwestClient::new();
    let scraper = Scraper::new(client).with_delay(Duration::from_millis(200));

    let start = NaiveDate::from_ymd_opt(1995, 6, 16).unwrap();
    let end = NaiveDate::from_ymd_opt(2026, 1, 9).unwrap();
    let expected_days = (end - start).num_days() + 1;

    let pb = ProgressBar::new(expected_days as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} (ETA: {eta}) | {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );
    pb.enable_steady_tick(Duration::from_millis(100));

    let mut archive = Archive::default();
    let mut stream = scraper.iter_html(start, end);
    let mut count = 0u64;
    let mut missing = 0u64;
    let mut errors = 0u64;
    let mut expected_date = start;

    while let Some(result) = stream.next().await {
        match result {
            Ok((date, html)) => {
                while expected_date < date {
                    missing += 1;
                    pb.println(format!("  ⚠ Missing: {}", expected_date));
                    pb.inc(1);
                    expected_date = expected_date.succ_opt().unwrap();
                }

                count += 1;
                pb.set_message(format!(
                    "At {date}, {count} fetched, {missing} missing, {errors} errors)"
                ));
                pb.inc(1);
                archive.push(ArchiveHtml::new(date, html));
                expected_date = date.succ_opt().unwrap();
            }
            Err((date, error)) => {
                errors += 1;
                pb.println(format!("  ✗ Error ({}): {}", date, error));
                pb.inc(1);
                expected_date = date.succ_opt().unwrap();
            }
        }
    }

    while expected_date <= end {
        missing += 1;
        pb.println(format!("  ⚠ Missing: {}", expected_date));
        pb.inc(1);
        expected_date = expected_date.succ_opt().unwrap();
    }

    pb.set_message(format!("Compressing {} entries...", count));
    archive.save(&PathBuf::from("./output.bin")).unwrap();

    pb.finish_with_message(format!(
        "Done! {count} fetched, {missing} missing, {errors} errors."
    ));
}
