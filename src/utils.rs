use std::process::Command;

use crate::models::ImageParameters;
use anyhow::{bail, Context};
use lazy_static::lazy_static;
use regex::Regex;

/// Extract parameters which were used to generate image from image metadata
pub fn extract_metadata_from_image(path: &str) -> anyhow::Result<Option<ImageParameters>> {
    let output = Command::new("exiftool")
        .args([path, "-h", "-Parameters", "-j"])
        .output()
        .context("Failed to spawn proces")?;
    if !output.status.success() {
        bail!("Process exited with non-zero status: {}", output.status);
    }

    let raw_json =
        String::from_utf8(output.stdout).context("Failed to parse command stdout to UTF-8")?;
    let json = json::parse(&raw_json[..]).context("Failed to parse stdout to json")?;
    match json[0]["Parameters"].as_str() {
        None => Ok(None),
        Some(raw) => Ok(parse_raw(raw)),
    }
}

lazy_static! {
    static ref PARAMETERS_REGEX: Regex = Regex::new(
        r#"^(?P<prompt>[\S\s]+)\nNegative prompt: (?P<negative_prompt>[\S\s]+)\nSteps: (?P<steps>\d+), Sampler: (?P<sampler>.+), CFG scale: (?P<cfg_scale>\d+), Seed: (?P<seed>-?\d+), Size: (?P<size>\d+x\d+), Model hash: (?P<model_hash>.+), Model: (?P<model>.+), Conditional mask weight: (?P<conditional_mask_weight>.+), Clip skip: (?P<clip_skip>\d+)"#,
    ).unwrap();
}

/// Parses image parameters to the structure (see [`ImageParameters`])
fn parse_raw(raw: &str) -> Option<ImageParameters> {
    if let Some(captures) = PARAMETERS_REGEX.captures(raw) {
        Some(ImageParameters {
            prompt: captures.name("prompt")?.as_str().to_owned(),
            negative_prompt: captures.name("negative_prompt")?.as_str().to_owned(),
            steps: captures.name("steps")?.as_str().parse::<u64>().ok()?,
            sampler: captures.name("sampler")?.as_str().to_owned(),
            cfg_scale: captures.name("cfg_scale")?.as_str().parse::<f64>().ok()?,
            seed: captures.name("seed")?.as_str().parse::<i64>().ok()?,
            size: parse_size(captures.name("size")?.as_str()).ok()?,
            model_hash: captures.name("model_hash")?.as_str().to_owned(),
            model: captures.name("model")?.as_str().to_owned(),
            clip_skip: captures.name("clip_skip")?.as_str().parse::<u64>().ok()?,
        })
    } else {
        None
    }
}

#[derive(Debug)]
struct ParseSizeError;

/// Parse string "123x456" to (123, 456)
fn parse_size(raw: &str) -> Result<(u64, u64), ParseSizeError> {
    let (w, h) = raw.split_once('x').ok_or(ParseSizeError)?;
    Ok((
        w.parse::<u64>().map_err(|_| ParseSizeError)?,
        h.parse::<u64>().map_err(|_| ParseSizeError)?,
    ))
}
