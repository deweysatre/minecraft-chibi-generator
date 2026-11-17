mod cli;
mod positions;
mod renderer;
mod skin;

use crate::renderer::trim_transparent_border;
use anyhow::Context;
use clap::Parser;
use cli::{Args, View};
use image::imageops::{FilterType, resize};
use std::path::{Path, PathBuf};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let scale = args.scale.unwrap_or(1);
    let output = args.output;
    let all_views = args.all_views;
    let view = match args.view {
        Some(view) => view,
        None if all_views.is_some() => View::All,
        None => {
            anyhow::bail!("You must provide 'view' or 'all_views'")
        }
    };
    let skin = if let Some(username) = args.username {
        skin::download(username)
            .await
            .context("Error while downloading skin")?
    } else if let Some(input) = args.input {
        image::open(&input)
            .with_context(|| format!("Unable to load skin from {}", input.display()))?
    } else {
        anyhow::bail!("You must provide either 'username' or 'input'");
    };
    let is_slim = skin::detect_slim(skin.clone());

    match view {
        View::All => {
            let output = all_views.expect("View::All requires an output directory");
            std::fs::create_dir_all(&output).with_context(|| {
                format!("Unable to create output directory {}", output.display())
            })?;
            for (view, filename) in [
                (View::Right34, "right_34.png"),
                (View::BackRight34, "right_34_back.png"),
                (View::Right, "right.png"),
                (View::Left34, "left_34.png"),
                (View::BackLeft34, "left_34_back.png"),
                (View::Left, "left.png"),
                (View::Front, "front.png"),
                (View::Back, "back.png"),
                (View::Top, "top.png"),
                (View::Bottom, "bottom.png"),
            ] {
                save_view(
                    &skin,
                    is_slim,
                    view,
                    scale,
                    &output.join(filename),
                    args.no_trim,
                )?;
            }
        }
        view => {
            let output = output.unwrap_or_else(|| PathBuf::from("chibi.png"));
            let parent_folder = output.parent().unwrap();
            std::fs::create_dir_all(parent_folder).with_context(|| {
                format!("Unable to create output directory {}", output.display())
            })?;
            save_view(&skin, is_slim, view, scale, &output, args.no_trim)?;
        }
    }

    Ok(())
}

fn save_view(
    skin: &image::DynamicImage,
    is_slim: bool,
    view: View,
    scale: u32,
    output: &Path,
    no_trim: bool,
) -> anyhow::Result<()> {
    let skin = &skin.to_rgba8();
    let image = renderer::compose_view(skin, is_slim, view)?;
    let mut image = resize(&image, 24 * scale, 33 * scale, FilterType::Nearest);
    if !no_trim {
        image = trim_transparent_border(&image);
    }
    image
        .save(output)
        .with_context(|| format!("Unable to save image to {}", output.display()))?;
    Ok(())
}
