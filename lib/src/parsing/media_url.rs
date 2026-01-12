use crate::parsing::ParseError;
use crate::APOD_BASE_URL;
use regex::Regex;
use scraper::{Html, Selector};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MediaUrl {
    pub url: Option<String>,
    pub hd_url: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MediaUrlKind {
    ImagePNG,
    ImageJPG,
    ImageGIF,
    VideoMP4,
    YoutubeVideo,
}

impl MediaUrl {
    pub fn highest_quality(&self) -> Option<&str> {
        self.hd_url.as_deref().or(self.url.as_deref())
    }

    pub fn kind(&self) -> Option<MediaUrlKind> {
        let url = self.url.as_deref()?;

        if url.starts_with("https://www.youtube.com/embed/") {
            return Some(MediaUrlKind::YoutubeVideo);
        }

        let path = url.split('?').next().unwrap_or(url);
        let ext = path.rsplit_once('.')?.1.to_lowercase();

        match ext.as_str() {
            "png" => Some(MediaUrlKind::ImagePNG),
            "jpg" | "jpeg" => Some(MediaUrlKind::ImageJPG),
            "gif" => Some(MediaUrlKind::ImageGIF),
            "mp4" => Some(MediaUrlKind::VideoMP4),
            _ => None,
        }
    }
}

pub fn parse_media(doc: &Html) -> Result<MediaUrl, ParseError> {
    if let Some((url, hd_url)) = extract_image_urls(doc) {
        return Ok(MediaUrl {
            url: Some(get_last_url(&url)),
            hd_url: Some(get_last_url(&hd_url)),
        });
    }

    if let Some(url) = extract_video_url(doc) {
        return Ok(MediaUrl {
            url: Some(get_last_url(&url)),
            hd_url: None,
        });
    }

    Ok(MediaUrl {
        url: None,
        hd_url: None,
    })
}

fn extract_image_urls(doc: &Html) -> Option<(String, String)> {
    let img_sel = Selector::parse("img").unwrap();
    let a_sel = Selector::parse("a[href]").unwrap();

    let img = doc.select(&img_sel).next()?;
    let src = img.value().attr("src")?;
    let url = format!("{}/{}", APOD_BASE_URL, src);

    let mut hd_url = url.clone();

    for link in doc.select(&a_sel) {
        if let Some(href) = link.value().attr("href")
            && href.starts_with("image")
        {
            hd_url = format!("{}/{}", APOD_BASE_URL, href);
            break;
        }
    }

    Some((url, hd_url))
}

fn extract_video_url(doc: &Html) -> Option<String> {
    let iframe_sel = Selector::parse("iframe").unwrap();
    if let Some(iframe) = doc.select(&iframe_sel).next()
        && let Some(src) = iframe.value().attr("src")
    {
        return Some(src.to_string());
    }

    let video_sel = Selector::parse("video").unwrap();
    if let Some(video) = doc.select(&video_sel).next() {
        let source_sel = Selector::parse("source").unwrap();
        if let Some(source) = video.select(&source_sel).next()
            && let Some(src) = source.value().attr("src")
        {
            return Some(format!("{}/{}", APOD_BASE_URL, src));
        }
        if let Some(src) = video.value().attr("src") {
            return Some(format!("{}/{}", APOD_BASE_URL, src));
        }
    }

    None
}

fn get_last_url(data: &str) -> String {
    let re = Regex::new(r"https?://\S+$").unwrap();

    if let Some(mat) = re.find_iter(data).last() {
        mat.as_str().to_string()
    } else {
        data.to_string()
    }
}
