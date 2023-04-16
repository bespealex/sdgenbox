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
#[derive(Debug)]
pub struct ImageParameters {
    pub prompt: String,
    pub negative_prompt: String,
    pub steps: u64,
    pub sampler: String,
    pub cfg_scale: f64,
    pub seed: i64,
    pub size: (u64, u64),
    pub model_hash: String,
    pub model: String,
    pub clip_skip: u64,
}
