use crate::app::actions::AppActions;
use crate::runtime::task::TaskHandler;
use crate::runtime::RuntimeSystem;
use apodex::client::reqwest::ReqwestClient;
use apodex::client::{ApodClient, ClientError};
use apodex::date::ApodDate;
use std::collections::VecDeque;

#[derive(Default)]
pub struct Scraper {
    client: ReqwestClient,
    fetch_task: TaskHandler<Result<(ApodDate, Option<String>), (ApodDate, ClientError)>>,
    queue: VecDeque<ApodDate>,
}

impl Scraper {
    pub fn enqueue(&mut self, date: ApodDate) {
        self.queue.push_back(date);
    }

    pub fn dequeue(&mut self) -> Option<ApodDate> {
        self.queue.pop_front()
    }

    pub fn queue_len(&self) -> usize {
        self.queue.len()
    }

    pub fn abort(&mut self) {
        self.queue.clear();
        self.fetch_task.abort();
    }

    pub fn is_busy(&self) -> bool {
        self.fetch_task.is_busy()
    }

    pub fn status(&self) -> Option<String> {
        self.fetch_task.status()
    }
}

impl RuntimeSystem for Scraper {
    fn update(
        &mut self,
        _ctx: &egui::Context,
        handle: &tokio::runtime::Handle,
        actions: &AppActions,
    ) {
        if !self.is_busy() && self.queue.is_empty() {
            return;
        }

        if !self.is_busy()
            && let Some(date) = self.dequeue()
        {
            let client = self.client.clone();
            self.fetch_task.spawn(handle, move |ctx| async move {
                ctx.set_status(format!("Fetching page for {date}..."));
                match client.fetch_page(date).await {
                    Ok(page) => Ok((date, page)),
                    Err(e) => Err((date, e)),
                }
            })
        } else {
            match self.fetch_task.poll() {
                Some(Ok((date, page))) => {
                    if let Some(html) = page {
                        actions.insert_html(date, html);
                    } else {
                        actions.toast_warning(format!("Page for {date} not found"))
                    }
                }
                Some(Err((date, e))) => {
                    actions.toast_error(format!("Failed to fetch page for {date}: {e}"));
                }
                None => {}
            }
        }
    }
}
