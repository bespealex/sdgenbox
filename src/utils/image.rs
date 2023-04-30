use std::process::Command;

use crate::models::Image;
use anyhow::{bail, Context};
use lazy_static::lazy_static;
use regex::Regex;

/// Extract parameters which were used to generate image from image metadata
pub fn extract_metadata_from_image(path: &str) -> anyhow::Result<Option<Image>> {
    let output = Command::new("exiftool")
        .args([path, "-h", "-Parameters", "-j"])
        .output()
        .context("Failed to spawn proces")?;
    if !output.status.success() {
        // File is empty
        if output.status.code() == Some(1) {
            return Ok(None);
        }

        bail!("Process exited with non-zero status: {}", output.status);
    }

    let raw_json =
        String::from_utf8(output.stdout).context("Failed to parse command stdout to UTF-8")?;
    let json = json::parse(&raw_json[..]).context("Failed to parse stdout to json")?;
    match json[0]["Parameters"].as_str() {
        // File has to Parameters metadata
        None => Ok(None),
        Some(raw) => Ok(parse_raw(raw)),
    }
}

lazy_static! {
    static ref PARAMETERS_REGEX: Regex = Regex::new(
        r#"^(?P<prompt>[\S\s]+)\nNegative prompt: (?P<negative_prompt>[\S\s]+)\nSteps: (?P<steps>\d+), Sampler: (?P<sampler>[^,]+), CFG scale: (?P<cfg_scale>\d+), Seed: (?P<seed>-?\d+), Size: (?P<size>\d+x\d+), Model hash: (?P<model_hash>[^,]+), Model: (?P<model>[^,]+)(?:, Conditional mask weight: (?P<conditional_mask_weight>[^,]+))?(?:, Clip skip: (?P<clip_skip>\d+))?"#,
    ).unwrap();
}

/// Parses image parameters to the structure (see [`ImageParameters`])
fn parse_raw(raw: &str) -> Option<Image> {
    let captures = PARAMETERS_REGEX.captures(raw)?;

    let (width, height) = parse_size(captures.name("size").unwrap().as_str()).ok()?;
    Some(Image {
        id: -1,
        prompt: captures.name("prompt").unwrap().as_str().to_owned(),
        negative_prompt: captures
            .name("negative_prompt")
            .unwrap()
            .as_str()
            .to_owned(),
        steps: captures
            .name("steps")
            .unwrap()
            .as_str()
            .parse::<i64>()
            .unwrap(),
        sampler: captures.name("sampler").unwrap().as_str().to_owned(),
        cfg_scale: captures
            .name("cfg_scale")
            .unwrap()
            .as_str()
            .parse::<f64>()
            .unwrap(),
        seed: captures
            .name("seed")
            .unwrap()
            .as_str()
            .parse::<i64>()
            .unwrap(),
        width,
        height,
        model_hash: captures.name("model_hash").unwrap().as_str().to_owned(),
        model: captures.name("model").unwrap().as_str().to_owned(),
        clip_skip: captures
            .name("clip_skip").map(|clip_skip| clip_skip.as_str().parse::<i64>().unwrap()),
        file_path: None,
        created_at: chrono::NaiveDateTime::default(),
    })
}

#[derive(Debug)]
struct ParseSizeError;

/// Parse string "123x456" to (123, 456)
fn parse_size(raw: &str) -> Result<(i64, i64), ParseSizeError> {
    let (w, h) = raw.split_once('x').ok_or(ParseSizeError)?;
    Ok((
        w.parse::<i64>().map_err(|_| ParseSizeError)?,
        h.parse::<i64>().map_err(|_| ParseSizeError)?,
    ))
}
