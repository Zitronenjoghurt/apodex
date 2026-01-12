use crate::app::actions::AppActions;
use crate::directories::heed_cache_dir;
use crate::runtime::task::TaskHandler;
use crate::runtime::RuntimeSystem;
use apodex::client::reqwest::ReqwestClient;
use apodex::client::{ApodClient, ClientError};
use apodex::date::ApodDate;
use apodex::media::heed::HeedMediaCache;
use apodex::media::{MediaCache, MediaEntry};
use apodex::ApodEntry;
use egui::{ColorImage, Context, TextureHandle, TextureId, TextureOptions, Vec2};
use lru::LruCache;
use std::collections::VecDeque;
use std::num::NonZeroUsize;
use tokio::runtime::Handle;

pub struct ApodMedia {
    heed_cache: HeedMediaCache,
    texture_cache: LruCache<ApodDate, TextureHandle>,
    client: ReqwestClient,
    fetch_task: TaskHandler<Result<Option<MediaEntry>, ClientError>>,
    current_fetch: Option<ApodEntry>,
    queue: VecDeque<ApodEntry>,
}

impl Default for ApodMedia {
    fn default() -> Self {
        let heed_cache = HeedMediaCache::new("media", heed_cache_dir(), 2048).unwrap();
        let texture_cache = LruCache::new(NonZeroUsize::new(100).unwrap());

        Self {
            heed_cache,
            texture_cache,
            client: Default::default(),
            fetch_task: Default::default(),
            current_fetch: None,
            queue: Default::default(),
        }
    }
}

impl ApodMedia {
    pub fn show_image(&mut self, ui: &mut egui::Ui, entry: &ApodEntry) {
        if let Some((id, aspect)) = self.get_texture(ui.ctx(), entry) {
            let size = fit_to_bounds(ui.available_size(), aspect);
            ui.image((id, size));
        } else if let Some(status) = self.status() {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label(status);
            });
        } else {
            ui.small("No media found");
        }
    }

    pub fn get_texture(&mut self, ctx: &Context, entry: &ApodEntry) -> Option<(TextureId, f32)> {
        if let Some(handle) = self.texture_cache.get(&entry.date) {
            return Some((handle.id(), handle.aspect_ratio()));
        }

        if let Some(media_entry) = self.heed_cache.get(entry.date).ok().flatten() {
            let texture = Self::create_texture(ctx, entry.date, &media_entry.data)?;
            let id = texture.id();
            let aspect = texture.aspect_ratio();
            self.texture_cache.put(entry.date, texture);
            return Some((id, aspect));
        }

        if !self.queue.contains(entry) && self.current_fetch.as_ref() != Some(entry) {
            self.queue.push_back(entry.clone());
        }

        None
    }

    pub fn is_busy(&self) -> bool {
        self.fetch_task.is_busy()
    }

    pub fn status(&self) -> Option<String> {
        self.fetch_task.status()
    }

    fn create_texture(ctx: &Context, date: ApodDate, data: &[u8]) -> Option<TextureHandle> {
        let img = image::load_from_memory(data)
            .ok()
            .map(|img| img.to_rgba8())?;
        let size = [img.width() as usize, img.height() as usize];
        let pixels = img.into_raw();
        Some(ctx.load_texture(
            format!("apod-{}", date),
            ColorImage::from_rgba_unmultiplied(size, &pixels),
            TextureOptions::LINEAR,
        ))
    }
}

impl RuntimeSystem for ApodMedia {
    fn update(&mut self, ctx: &Context, handle: &Handle, actions: &AppActions) {
        if !self.is_busy() && self.queue.is_empty() {
            return;
        }

        if !self.is_busy()
            && let Some(entry) = self.queue.pop_front()
        {
            let client = self.client.clone();
            self.current_fetch = Some(entry.clone());
            self.fetch_task.spawn(handle, move |ctx| async move {
                ctx.set_status(format!("Fetching media for {}", entry.date));
                client.fetch_media(&entry).await
            });
        } else {
            let Some(entry) = self.current_fetch.as_ref() else {
                return;
            };

            match self.fetch_task.poll() {
                Some(Ok(media)) => {
                    if let Some(media) = media {
                        if let Err(err) = self.heed_cache.store(
                            entry.date,
                            media.data.as_slice(),
                            media.media_type,
                        ) {
                            actions.toast_error(format!(
                                "Failed to cache media for {}: {err}",
                                entry.date
                            ));
                        }
                        let Some(texture) = Self::create_texture(ctx, entry.date, &media.data)
                        else {
                            actions.toast_warning(format!(
                                "Failed to decode media for {}",
                                entry.date
                            ));
                            self.current_fetch = None;
                            return;
                        };
                        self.texture_cache.put(entry.date, texture);
                    } else {
                        actions.toast_warning(format!("Media for {} not found", entry.date))
                    }
                    self.current_fetch = None;
                }
                Some(Err(err)) => {
                    actions.toast_error(format!("Failed to fetch media for {}: {err}", entry.date));
                    self.current_fetch = None;
                }
                None => {}
            }
        }
    }
}

fn fit_to_bounds(bounds: Vec2, aspect: f32) -> Vec2 {
    let width = bounds.x.min(bounds.y * aspect);
    let height = width / aspect;
    Vec2::new(width, height)
}
