use std::path::{Path, PathBuf};

use rand::{thread_rng, Rng};
use sqlx::{Executor, QueryBuilder, Row, Sqlite, Transaction};
use tokio::fs::File;

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
#[derive(Debug, PartialEq, serde::Serialize, sqlx::FromRow)]
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
    pub clip_skip: Option<i64>,
    pub file_path: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

pub async fn create_image(
    transaction: &mut Transaction<'_, Sqlite>,
    image: &mut Image,
    image_file: &mut File,
    media_root: &Path,
) -> anyhow::Result<()> {
    let file_path: PathBuf = generate_image_path();
    let mut file = tokio::fs::File::create(&media_root.join(&file_path)).await?;
    tokio::io::copy(image_file, &mut file).await?;
    let file_path = file_path.to_string_lossy();

    let id = sqlx::query_scalar!(
        r#"INSERT INTO image
         (prompt, negative_prompt, steps, sampler, cfg_scale, seed, width, height, model_hash, model, clip_skip, file_path)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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
        image.clip_skip,
        file_path,
    ).fetch_one(&mut *transaction).await?;
    image.id = id;
    image.file_path = Some(file_path.to_string());

    Ok(())
}

pub fn generate_image_path() -> PathBuf {
    let random_file_id = format!("{:16x}", thread_rng().gen::<u64>());
    let mut image_path = Path::new("images").join(random_file_id.as_str());
    image_path.set_extension("png");
    image_path
}

pub async fn fetch_image_by_id(
    executor: impl Executor<'_, Database = Sqlite>,
    image_id: i64,
) -> sqlx::Result<Option<Image>> {
    sqlx::query_as!(Image, r#"SELECT id, prompt, negative_prompt, steps, sampler, cfg_scale, seed, width, height, model_hash, model, clip_skip, file_path, created_at as "created_at: _" FROM image WHERE id = ?"#, image_id)
        .fetch_optional(executor)
        .await
}

pub struct Limits {
    offset: u32,
    limit: u32,
}

impl Limits {
    pub fn from_page(page: u32, page_size: u32) -> Self {
        Limits {
            offset: page_size * page,
            limit: page_size,
        }
    }
}

fn add_filter_to_query(query: &mut QueryBuilder<Sqlite>, search: &str) {
    query
        .push(" WHERE cast(id as text) LIKE ")
        .push_bind(format!("%{}%", search.to_uppercase()))
        .push(" OR upper(prompt) LIKE ")
        .push_bind(format!("%{}%", search.to_uppercase()))
        .push(" OR upper(negative_prompt) LIKE ")
        .push_bind(format!("%{}%", search.to_uppercase()))
        .push(" OR upper(sampler) LIKE ")
        .push_bind(format!("%{}%", search.to_uppercase()))
        .push(" OR upper(model_hash) LIKE ")
        .push_bind(format!("%{}%", search.to_uppercase()))
        .push(" OR upper(model) LIKE ")
        .push_bind(format!("%{}%", search.to_uppercase()));
}

pub async fn fetch_images_count(
    executor: impl Executor<'_, Database = Sqlite>,
    search: Option<&str>,
) -> sqlx::Result<u32> {
    // Empty search is the same as no search
    let search = match search {
        Some("") => None,
        other => other,
    };

    let mut query = sqlx::QueryBuilder::new("SELECT count(*) FROM image");
    if let Some(search) = search {
        add_filter_to_query(&mut query, search);
    }
    let size = query.build().fetch_one(executor).await?.try_get(0)?;

    Ok(size)
}

pub async fn fetch_images(
    executor: impl Executor<'_, Database = Sqlite>,
    search: Option<&str>,
    limits: &Limits,
) -> sqlx::Result<Vec<Image>> {
    // Empty search is the same as no search
    let search = match search {
        Some("") => None,
        other => other,
    };

    let mut images_query = sqlx::QueryBuilder::new("SELECT id, prompt, negative_prompt, steps, sampler, cfg_scale, seed, width, height, model_hash, model, clip_skip, file_path, created_at FROM image");
    if let Some(search) = search {
        add_filter_to_query(&mut images_query, search);
    }
    images_query.push(" ORDER BY created_at DESC");
    images_query
        .push(" LIMIT ")
        .push_bind(limits.limit)
        .push(" OFFSET ")
        .push_bind(limits.offset);

    let images = images_query
        .build()
        .fetch_all(executor)
        .await?
        .into_iter()
        .map(|row| sqlx::FromRow::from_row(&row).unwrap())
        .collect();

    Ok(images)
}

#[cfg(test)]
mod test {
    use std::fs::create_dir;

    use chrono::NaiveDate;
    use sqlx::{migrate, pool::PoolConnection, Acquire, Sqlite};
    use tempfile::{NamedTempFile, TempDir};

    use super::{create_image, fetch_image_by_id, Image};

    fn new_test_image() -> Image {
        Image {
            id: 0,
            prompt: "prompt".to_string(),
            negative_prompt: "negative prompt".to_string(),
            steps: 42,
            sampler: "sampler".to_string(),
            cfg_scale: 4.2,
            seed: 1234,
            width: 400,
            height: 600,
            model_hash: "modelhash".to_string(),
            model: "model".to_string(),
            clip_skip: Some(1),
            file_path: None,
            created_at: NaiveDate::from_ymd_opt(2023, 4, 24)
                .unwrap()
                .and_hms_opt(11, 22, 33)
                .unwrap(),
        }
    }

    async fn new_connection() -> PoolConnection<Sqlite> {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect(":memory:")
            .await
            .unwrap();
        migrate!().run(&pool).await.unwrap();
        pool.acquire().await.unwrap()
    }

    fn prepare_media() -> TempDir {
        let media_root = TempDir::new().unwrap();
        create_dir(media_root.path().join("images")).unwrap();
        media_root
    }

    #[actix_web::test]
    async fn test_create_image() {
        let mut connection = new_connection().await;
        let mut transaction = connection.begin().await.unwrap();
        let media_root = prepare_media();

        let mut image = new_test_image();
        let original_file = NamedTempFile::new().unwrap();
        create_image(
            &mut transaction,
            &mut image,
            &mut tokio::fs::File::from_std(original_file.into_file()),
            media_root.path(),
        )
        .await
        .unwrap();

        assert_ne!(image.id, 0);
        let image_file = image.file_path.unwrap();
        assert!(media_root.path().join(image_file).exists());
    }

    #[actix_web::test]
    async fn test_image_get_by_id() {
        let mut connection = new_connection().await;
        let mut transaction = connection.begin().await.unwrap();
        let media_root = prepare_media();

        let mut image = new_test_image();
        let original_file = NamedTempFile::new().unwrap();
        create_image(
            &mut transaction,
            &mut image,
            &mut tokio::fs::File::from_std(original_file.into_file()),
            media_root.path(),
        )
        .await
        .unwrap();

        let fetched_image = fetch_image_by_id(&mut transaction, image.id).await.unwrap();
        let fetched_image = fetched_image.unwrap();

        assert_eq!(
            fetched_image,
            Image {
                // autofilled by DB
                created_at: fetched_image.created_at,
                ..image
            }
        )
    }
}
