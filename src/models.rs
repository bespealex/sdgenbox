use sqlx::{pool::PoolConnection, Sqlite};

/// Parameters what were used to generate image
///
/// Example:
/// Example:
/// masterpiece, (Henri-Julien Dumont:1.4), 1girl, <lora:lycorisRecoil_chisatoV10:1>, blonde hair,
/// Negative prompt: (worst quality:1.4), (low quality:1.4), (monochrome:1.2), (bad_prompt:1.6), multiple penis, multiple views,, (painting by bad-artist-anime:0.9), (painting by bad-artist:0.9), watermark, text, error, blurry, jpeg artifacts, cropped, worst quality, low quality, normal quality, jpeg artifacts, signature, watermark, username, artist name, (worst quality, low quality:1.4), bad anatomy,, (worst quality:1.4), (low quality:1.4), (monochrome:1.2), (bad_prompt:1.6), multiple penis, multiple views,, (painting by bad-artist-anime:0.9), (painting by bad-artist:0.9), watermark, text, error, blurry, jpeg artifacts, cropped, worst quality, low quality, normal quality, jpeg artifacts, signature, watermark, username, artist name, (worst quality, low quality:1.4), (bad anatomy:1.5), (multiple girls:1.4), (2girls:1.4), bad-hands-5,
/// Steps: 20,
/// Sampler: DPM++ 2M Karras,
/// CFG scale: 7,
/// Seed: 2179987202,
/// Size: 768x512,
/// Model hash: 93b79e09ed,
/// Model: anything-v4.5-inpainting.inpainting,
/// Conditional mask weight: 1.0,
/// Clip skip: 2
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct Image {
    pub id: i64,
    pub prompt: String,
    pub negative_prompt: String,
    pub steps: i64,
    pub sampler: String,
    pub cfg_scale: f64,
    pub seed: i64,
    pub width: i64,
    pub height: i64,
    pub model_hash: String,
    pub model: String,
    pub clip_skip: i64,
}

pub async fn create_image(
    connection: &mut PoolConnection<Sqlite>,
    image: &mut Image,
) -> sqlx::Result<()> {
    let id = sqlx::query_scalar!(
        r#"INSERT INTO image
         (prompt, negative_prompt, steps, sampler, cfg_scale, seed, width, height, model_hash, model, clip_skip)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING id"#,
        image.prompt,
        image.negative_prompt,
        image.steps,
        image.sampler,
        image.cfg_scale,
        image.seed,
        image.width,
        image.height,
        image.model_hash,
        image.model,
        image.clip_skip
    ).fetch_one(connection).await?;

    image.id = id;
    Ok(())
}

pub async fn fetch_image_by_id(
    connection: &mut PoolConnection<Sqlite>,
    image_id: i64,
) -> sqlx::Result<Option<Image>> {
    sqlx::query_as!(Image, "SELECT * FROM image WHERE id = ?", image_id)
        .fetch_optional(connection)
        .await
}

pub async fn fetch_all_images(connection: &mut PoolConnection<Sqlite>) -> sqlx::Result<Vec<Image>> {
    sqlx::query_as!(Image, "select * from image")
        .fetch_all(connection)
        .await
}
