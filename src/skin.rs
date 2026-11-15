use anyhow::{Context, Result, bail};
use image::{DynamicImage, RgbaImage};
use reqwest::StatusCode;

pub async fn download(username: impl Into<String>) -> Result<DynamicImage> {
    let username = username.into();
    if username.is_empty()
        || username.len() > 16
        || !username
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        bail!("Invalid username");
    };

    let client = reqwest::Client::new();

    // Check if the player exist
    if client
        .get(format!(
            "https://playerdb.co/api/player/minecraft/{}",
            url_encode(&username)
        ))
        .send()
        .await
        .context("Failed to check with playerdb")?
        .status()
        != StatusCode::OK
    {
        bail!("Player doesn't exist");
    }

    // Download the skin
    let response = client
        .get(format!(
            "https://mc-heads.net/skin/{}",
            url_encode(username)
        ))
        .send()
        .await
        .context("Error while downloading from mc-heads.net")?;
    if !response.status().is_success() {
        bail!(
            "mc-heads.net didn't returned a skin ({})",
            response.status()
        );
    }
    image::load_from_memory(
        &response
            .bytes()
            .await
            .context("Le skin retourné est vide.")?,
    )
    .context("Le format du skin n'est pas un PNG valide")
}

fn url_encode(string: impl Into<String>) -> String {
    string
        .into()
        .bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_' | b'-' => (b as char).to_string(),
            _ => format!("%{b:02X}"),
        })
        .collect()
}

pub fn detect_slim(skin: RgbaImage) -> bool {
    let alpha = |x: u32, y: u32| skin.get_pixel(x, y).0[3] >= 128;
    let mut occupied = false;
    for y in 20..32 {
        for x in 44..48 {
            occupied |= alpha(x, y);
        }
        for x in 36..40 {
            occupied |= alpha(x, y);
        }
    }
    !occupied
}
