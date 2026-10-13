#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use image::{DynamicImage, ImageFormat, RgbaImage};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlemishSpot {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct FaceRegion {
    pub center_x: f32,
    pub center_y: f32,
    pub face_rx: f32,
    pub face_ry: f32,
    pub left_eye_x: f32,
    pub left_eye_y: f32,
    pub right_eye_x: f32,
    pub right_eye_y: f32,
    pub mouth_x: f32,
    pub mouth_y: f32,
    pub nose_x: f32,
    pub nose_y: f32,
    // Exact 478-point anatomical anchor coordinates
    pub chin_x: f32,
    pub chin_y: f32,
    pub forehead_x: f32,
    pub forehead_y: f32,
    pub jaw_left_x: f32,
    pub jaw_left_y: f32,
    pub jaw_right_x: f32,
    pub jaw_right_y: f32,
    pub cheek_left_x: f32,
    pub cheek_left_y: f32,
    pub cheek_right_x: f32,
    pub cheek_right_y: f32,
    pub mouth_left_x: f32,
    pub mouth_left_y: f32,
    pub mouth_right_x: f32,
    pub mouth_right_y: f32,
}

impl Default for FaceRegion {
    fn default() -> Self {
        Self {
            center_x: 0.5,
            center_y: 0.42,
            face_rx: 0.18,
            face_ry: 0.24,
            left_eye_x: 0.43,
            left_eye_y: 0.38,
            right_eye_x: 0.57,
            right_eye_y: 0.38,
            mouth_x: 0.50,
            mouth_y: 0.51,
            nose_x: 0.50,
            nose_y: 0.44,
            chin_x: 0.50,
            chin_y: 0.65,
            forehead_x: 0.50,
            forehead_y: 0.20,
            jaw_left_x: 0.35,
            jaw_left_y: 0.55,
            jaw_right_x: 0.65,
            jaw_right_y: 0.55,
            cheek_left_x: 0.34,
            cheek_left_y: 0.43,
            cheek_right_x: 0.66,
            cheek_right_y: 0.43,
            mouth_left_x: 0.45,
            mouth_left_y: 0.51,
            mouth_right_x: 0.55,
            mouth_right_y: 0.51,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportedPhotoPayload {
    pub path: String,
    pub name: String,
    pub data_url: String,
    pub width: u32,
    pub height: u32,
    pub detected_face: FaceRegion,
}

fn default_treatment() -> String {
    "color".to_string()
}
fn default_wb_mode() -> String {
    "asShot".to_string()
}
fn default_100() -> f32 {
    100.0
}
fn default_80() -> f32 {
    80.0
}
fn default_75() -> f32 {
    75.0
}
fn default_50() -> f32 {
    50.0
}
fn default_40() -> f32 {
    40.0
}
fn default_25() -> f32 {
    25.0
}
fn default_1() -> f32 {
    1.0
}
fn default_120() -> f32 {
    120.0
}
fn default_45() -> f32 {
    45.0
}
fn default_lip_tint() -> String {
    "rose".to_string()
}
fn default_none_str() -> String {
    "none".to_string()
}
fn default_original_str() -> String {
    "original".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RetouchParams {
    // --- MODULE 2: PORTRAIT RETOUCHING ---
    // 1. Blemish Removal
    pub freckle_acne: f32,
    pub face_mole: f32,
    pub face_shine: f32,
    pub dark_circles: f32,
    pub eye_bags: f32,
    pub forehead_wrinkles: f32,
    pub nasolabial_folds: f32,
    pub neck_wrinkles: f32,
    pub glasses_glare: f32,
    pub double_chin: f32,
    pub body_blemish: f32,

    // 2. Skin Retouching
    pub dodge_burn_even: f32,
    pub dodge_burn_sculpt: f32,
    pub freq_sep_low: f32,
    #[serde(default = "default_80")]
    pub freq_sep_high: f32,
    pub facial_skin_smooth: f32,
    pub body_skin_smooth: f32,
    pub skin_radiance: f32,
    pub skin_rosy: f32,
    pub skin_warmth: f32,
    pub unify_complexion: f32,

    // 3. Face Reshape (3D Liquify)
    pub face_width: f32,
    pub jawline_slim: f32,
    pub cheekbone_lift: f32,
    pub chin_length: f32,
    pub forehead_height: f32,
    pub nose_width: f32,
    pub nose_tip_lift: f32,
    pub mouth_size: f32,
    pub lip_plump: f32,
    pub smile_enhance: f32,

    // 4. Eyes & Eyebrows
    pub eye_size: f32,
    #[serde(default)]
    pub eye_distance: f32,
    #[serde(default)]
    pub eye_tilt: f32,
    pub eye_brighten: f32,
    pub red_veins: f32,
    pub catchlight_boost: f32,
    pub eyebrow_thickness: f32,
    pub eyebrow_lift: f32,
    #[serde(default)]
    pub eyebrow_tilt: f32,

    // 5. Teeth
    pub teeth_whiten: f32,
    pub teeth_desaturate: f32,
    pub teeth_brighten: f32,

    // 6. Makeup Studio
    pub makeup_highlight: f32,
    pub makeup_contour: f32,
    pub makeup_blush: f32,
    pub makeup_eyeshadow: f32,
    pub makeup_lipstick: f32,
    #[serde(default = "default_lip_tint")]
    pub makeup_lip_tint: String,

    // 7. Hair
    pub stray_hair: f32,
    pub hair_volume: f32,
    pub hair_part_line: f32,
    pub hair_shine: f32,
    pub hair_dye_strength: f32,
    #[serde(default = "default_none_str")]
    pub hair_dye_color: String,

    // 8. Body Reshape
    pub neck_slim: f32,
    pub shoulder_sculpt: f32,
    pub waist_slim: f32,

    // --- MODULE 1: COLOR ADJUSTMENT (LIGHTCRAFT + RETOUCH) ---
    #[serde(default = "default_treatment")]
    pub treatment: String,
    #[serde(default = "default_100")]
    pub profile_amount: f32,
    #[serde(default = "default_wb_mode")]
    pub wb_mode: String,
    pub temperature: f32,
    pub tint: f32,
    pub exposure: f32,
    pub contrast: f32,
    pub highlights: f32,
    pub shadows: f32,
    pub whites: f32,
    pub blacks: f32,
    pub clarity: f32,
    pub texture: f32,
    pub dehaze: f32,
    pub vibrance: f32,
    pub saturation: f32,

    // Parametric + Point Tone Curve (Master + R/G/B)
    pub curve_shadows: f32,
    pub curve_darks: f32,
    pub curve_lights: f32,
    pub curve_highlights: f32,
    #[serde(default = "default_25")]
    pub curve_split_shadows: f32,
    #[serde(default = "default_50")]
    pub curve_split_mid: f32,
    #[serde(default = "default_75")]
    pub curve_split_highlights: f32,
    #[serde(default = "default_100")]
    pub curve_refine_saturation: f32,
    #[serde(default)]
    pub curve_lut: Vec<u8>,
    #[serde(default)]
    pub curve_lut_red: Vec<u8>,
    #[serde(default)]
    pub curve_lut_green: Vec<u8>,
    #[serde(default)]
    pub curve_lut_blue: Vec<u8>,

    // 8-Band HSL Color Mixer
    pub hsl_red_hue: f32,
    pub hsl_red_sat: f32,
    pub hsl_red_lum: f32,
    pub hsl_orange_hue: f32,
    pub hsl_orange_sat: f32,
    pub hsl_orange_lum: f32,
    pub hsl_yellow_hue: f32,
    pub hsl_yellow_sat: f32,
    pub hsl_yellow_lum: f32,
    pub hsl_green_hue: f32,
    pub hsl_green_sat: f32,
    pub hsl_green_lum: f32,
    #[serde(default)]
    pub hsl_aqua_hue: f32,
    #[serde(default)]
    pub hsl_aqua_sat: f32,
    #[serde(default)]
    pub hsl_aqua_lum: f32,
    pub hsl_blue_hue: f32,
    pub hsl_blue_sat: f32,
    pub hsl_blue_lum: f32,
    #[serde(default)]
    pub hsl_purple_hue: f32,
    #[serde(default)]
    pub hsl_purple_sat: f32,
    #[serde(default)]
    pub hsl_purple_lum: f32,
    #[serde(default)]
    pub hsl_magenta_hue: f32,
    #[serde(default)]
    pub hsl_magenta_sat: f32,
    #[serde(default)]
    pub hsl_magenta_lum: f32,

    // 8-Band B&W Mixer (Lightcraft BwMix)
    #[serde(default)]
    pub bw_red: f32,
    #[serde(default)]
    pub bw_orange: f32,
    #[serde(default)]
    pub bw_yellow: f32,
    #[serde(default)]
    pub bw_green: f32,
    #[serde(default)]
    pub bw_aqua: f32,
    #[serde(default)]
    pub bw_blue: f32,
    #[serde(default)]
    pub bw_purple: f32,
    #[serde(default)]
    pub bw_magenta: f32,

    // Reference Color Match
    #[serde(default)]
    pub color_match_strength: f32,
    #[serde(default)]
    pub color_match_stats: Vec<f32>,

    // 4-Way Color Grading (Shadows, Midtones, Highlights, Global + Blending & Balance)
    pub grade_shadow_hue: f32,
    pub grade_shadow_sat: f32,
    #[serde(default)]
    pub grade_shadow_lum: f32,
    #[serde(default = "default_120")]
    pub grade_mid_hue: f32,
    #[serde(default)]
    pub grade_mid_sat: f32,
    #[serde(default)]
    pub grade_mid_lum: f32,
    pub grade_high_hue: f32,
    pub grade_high_sat: f32,
    #[serde(default)]
    pub grade_high_lum: f32,
    #[serde(default = "default_45")]
    pub grade_global_hue: f32,
    #[serde(default)]
    pub grade_global_sat: f32,
    #[serde(default)]
    pub grade_global_lum: f32,
    #[serde(default = "default_50")]
    pub grade_blending: f32,
    #[serde(default)]
    pub grade_balance: f32,

    // Detail, Sharpening & Noise Reduction (Lightcraft Detail)
    #[serde(default)]
    pub ai_denoise: f32,
    pub sharpen_amount: f32,
    #[serde(default = "default_1")]
    pub sharpen_radius: f32,
    #[serde(default = "default_25")]
    pub sharpen_detail: f32,
    #[serde(default)]
    pub sharpen_masking: f32,
    pub noise_reduction: f32,
    #[serde(default = "default_50")]
    pub nr_detail: f32,
    #[serde(default)]
    pub nr_contrast: f32,
    #[serde(default)]
    pub nr_color: f32,
    #[serde(default = "default_50")]
    pub nr_color_detail: f32,
    #[serde(default = "default_50")]
    pub nr_color_smoothness: f32,

    // Effects: Film Grain & Post-Crop Vignette (Lightcraft Finish)
    pub film_grain: f32,
    #[serde(default = "default_25")]
    pub grain_size: f32,
    #[serde(default = "default_50")]
    pub grain_roughness: f32,
    pub vignette: f32,
    #[serde(default = "default_50")]
    pub vignette_midpoint: f32,
    #[serde(default)]
    pub vignette_roundness: f32,
    #[serde(default = "default_50")]
    pub vignette_feather: f32,
    #[serde(default)]
    pub vignette_highlights: f32,

    // Camera Calibration (Lightcraft Calibration)
    #[serde(default)]
    pub cal_shadows_tint: f32,
    #[serde(default)]
    pub cal_red_hue: f32,
    #[serde(default)]
    pub cal_red_sat: f32,
    #[serde(default)]
    pub cal_green_hue: f32,
    #[serde(default)]
    pub cal_green_sat: f32,
    #[serde(default)]
    pub cal_blue_hue: f32,
    #[serde(default)]
    pub cal_blue_sat: f32,

    #[serde(default = "default_none_str")]
    pub lut_preset: String,

    // --- MODULE 3: BACKGROUND, LENS BLUR & CLOTHING ---
    pub clothing_dewrinkle: f32,
    pub clothing_lint_clean: f32,
    pub backdrop_clean: f32,
    pub backdrop_banding_fix: f32,
    pub sky_enhance: f32,
    pub bg_blur_bokeh: f32,
    #[serde(default = "default_50")]
    pub lens_blur_focal_distance: f32,
    #[serde(default = "default_40")]
    pub lens_blur_focal_range: f32,
    #[serde(default = "default_original_str")]
    pub bg_replace_mode: String,

    // --- MODULE 4: OPTICS, GEOMETRY & TRANSFORM (LIGHTCRAFT OPTICS + UPRIGHT) ---
    #[serde(default)]
    pub optics_distortion: f32,
    #[serde(default)]
    pub optics_vignetting: f32,
    #[serde(default = "default_50")]
    pub optics_vignetting_midpoint: f32,
    #[serde(default)]
    pub optics_ca_red: f32,
    #[serde(default)]
    pub optics_ca_blue: f32,
    #[serde(default)]
    pub optics_defringe_purple: f32,
    #[serde(default)]
    pub optics_defringe_green: f32,

    pub rotate_angle: f32,
    pub keystone_vertical: f32,
    pub keystone_horizontal: f32,
    #[serde(default)]
    pub geo_aspect: f32,
    #[serde(default = "default_100")]
    pub geo_scale: f32,
    #[serde(default)]
    pub geo_offset_x: f32,
    #[serde(default)]
    pub geo_offset_y: f32,
    pub flip_horizontal: bool,
    #[serde(default)]
    pub flip_vertical: bool,

    #[serde(default)]
    pub blemish_spots: Vec<BlemishSpot>,
    #[serde(default)]
    pub face: FaceRegion,
}

impl Default for RetouchParams {
    fn default() -> Self {
        Self {
            freckle_acne: 0.0,
            face_mole: 0.0,
            face_shine: 0.0,
            dark_circles: 0.0,
            eye_bags: 0.0,
            forehead_wrinkles: 0.0,
            nasolabial_folds: 0.0,
            neck_wrinkles: 0.0,
            glasses_glare: 0.0,
            double_chin: 0.0,
            body_blemish: 0.0,
            dodge_burn_even: 0.0,
            dodge_burn_sculpt: 0.0,
            freq_sep_low: 0.0,
            freq_sep_high: 80.0,
            facial_skin_smooth: 0.0,
            body_skin_smooth: 0.0,
            skin_radiance: 0.0,
            skin_rosy: 0.0,
            skin_warmth: 0.0,
            unify_complexion: 0.0,
            face_width: 0.0,
            jawline_slim: 0.0,
            cheekbone_lift: 0.0,
            chin_length: 0.0,
            forehead_height: 0.0,
            nose_width: 0.0,
            nose_tip_lift: 0.0,
            mouth_size: 0.0,
            lip_plump: 0.0,
            smile_enhance: 0.0,
            eye_size: 0.0,
            eye_distance: 0.0,
            eye_tilt: 0.0,
            eye_brighten: 0.0,
            red_veins: 0.0,
            catchlight_boost: 0.0,
            eyebrow_thickness: 0.0,
            eyebrow_lift: 0.0,
            eyebrow_tilt: 0.0,
            teeth_whiten: 0.0,
            teeth_desaturate: 0.0,
            teeth_brighten: 0.0,
            makeup_highlight: 0.0,
            makeup_contour: 0.0,
            makeup_blush: 0.0,
            makeup_eyeshadow: 0.0,
            makeup_lipstick: 0.0,
            makeup_lip_tint: "rose".to_string(),
            stray_hair: 0.0,
            hair_volume: 0.0,
            hair_part_line: 0.0,
            hair_shine: 0.0,
            hair_dye_strength: 0.0,
            hair_dye_color: "none".to_string(),
            neck_slim: 0.0,
            shoulder_sculpt: 0.0,
            waist_slim: 0.0,
            treatment: "color".to_string(),
            profile_amount: 100.0,
            wb_mode: "asShot".to_string(),
            temperature: 0.0,
            tint: 0.0,
            exposure: 0.0,
            contrast: 0.0,
            highlights: 0.0,
            shadows: 0.0,
            whites: 0.0,
            blacks: 0.0,
            clarity: 0.0,
            texture: 0.0,
            dehaze: 0.0,
            vibrance: 0.0,
            saturation: 0.0,
            curve_shadows: 0.0,
            curve_darks: 0.0,
            curve_lights: 0.0,
            curve_highlights: 0.0,
            curve_split_shadows: 25.0,
            curve_split_mid: 50.0,
            curve_split_highlights: 75.0,
            curve_refine_saturation: 100.0,
            curve_lut: Vec::new(),
            curve_lut_red: Vec::new(),
            curve_lut_green: Vec::new(),
            curve_lut_blue: Vec::new(),
            hsl_red_hue: 0.0,
            hsl_red_sat: 0.0,
            hsl_red_lum: 0.0,
            hsl_orange_hue: 0.0,
            hsl_orange_sat: 0.0,
            hsl_orange_lum: 0.0,
            hsl_yellow_hue: 0.0,
            hsl_yellow_sat: 0.0,
            hsl_yellow_lum: 0.0,
            hsl_green_hue: 0.0,
            hsl_green_sat: 0.0,
            hsl_green_lum: 0.0,
            hsl_aqua_hue: 0.0,
            hsl_aqua_sat: 0.0,
            hsl_aqua_lum: 0.0,
            hsl_blue_hue: 0.0,
            hsl_blue_sat: 0.0,
            hsl_blue_lum: 0.0,
            hsl_purple_hue: 0.0,
            hsl_purple_sat: 0.0,
            hsl_purple_lum: 0.0,
            hsl_magenta_hue: 0.0,
            hsl_magenta_sat: 0.0,
            hsl_magenta_lum: 0.0,
            bw_red: 0.0,
            bw_orange: 0.0,
            bw_yellow: 0.0,
            bw_green: 0.0,
            bw_aqua: 0.0,
            bw_blue: 0.0,
            bw_purple: 0.0,
            bw_magenta: 0.0,
            color_match_strength: 0.0,
            color_match_stats: Vec::new(),
            grade_shadow_hue: 215.0,
            grade_shadow_sat: 0.0,
            grade_shadow_lum: 0.0,
            grade_mid_hue: 120.0,
            grade_mid_sat: 0.0,
            grade_mid_lum: 0.0,
            grade_high_hue: 38.0,
            grade_high_sat: 0.0,
            grade_high_lum: 0.0,
            grade_global_hue: 45.0,
            grade_global_sat: 0.0,
            grade_global_lum: 0.0,
            grade_blending: 50.0,
            grade_balance: 0.0,
            ai_denoise: 0.0,
            sharpen_amount: 0.0,
            sharpen_radius: 1.0,
            sharpen_detail: 25.0,
            sharpen_masking: 0.0,
            noise_reduction: 0.0,
            nr_detail: 50.0,
            nr_contrast: 0.0,
            nr_color: 0.0,
            nr_color_detail: 50.0,
            nr_color_smoothness: 50.0,
            film_grain: 0.0,
            grain_size: 25.0,
            grain_roughness: 50.0,
            vignette: 0.0,
            vignette_midpoint: 50.0,
            vignette_roundness: 0.0,
            vignette_feather: 50.0,
            vignette_highlights: 0.0,
            cal_shadows_tint: 0.0,
            cal_red_hue: 0.0,
            cal_red_sat: 0.0,
            cal_green_hue: 0.0,
            cal_green_sat: 0.0,
            cal_blue_hue: 0.0,
            cal_blue_sat: 0.0,
            lut_preset: "none".to_string(),
            clothing_dewrinkle: 0.0,
            clothing_lint_clean: 0.0,
            backdrop_clean: 0.0,
            backdrop_banding_fix: 0.0,
            sky_enhance: 0.0,
            bg_blur_bokeh: 0.0,
            lens_blur_focal_distance: 50.0,
            lens_blur_focal_range: 40.0,
            bg_replace_mode: "original".to_string(),
            optics_distortion: 0.0,
            optics_vignetting: 0.0,
            optics_vignetting_midpoint: 50.0,
            optics_ca_red: 0.0,
            optics_ca_blue: 0.0,
            optics_defringe_purple: 0.0,
            optics_defringe_green: 0.0,
            rotate_angle: 0.0,
            keystone_vertical: 0.0,
            keystone_horizontal: 0.0,
            geo_aspect: 0.0,
            geo_scale: 100.0,
            geo_offset_x: 0.0,
            geo_offset_y: 0.0,
            flip_horizontal: false,
            flip_vertical: false,
            blemish_spots: Vec::new(),
            face: FaceRegion::default(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessResult {
    pub data_url: String,
    pub width: u32,
    pub height: u32,
    pub elapsed_ms: u128,
    pub detected_face: FaceRegion,
}

/// Pre-filtered 512x512 Semantic Anatomical Weights (zero label crosstalk, 100% eye/lip/brow protection)
#[derive(Copy, Clone, Default)]
struct SemanticWeights {
    skin: f32,        // Face & body skin, STRICTLY 0.0 on eyes, eyelashes, eyebrows, nostrils, lips, teeth
    forehead: f32,    // Forehead zone (G == 30)
    under_eye: f32,   // Under-eye dark circles / eye bags crescent (G == 60), STRICTLY 0.0 on eyeball
    nasolabial: f32,  // Smile lines / nasolabial folds (G == 85)
    eyebrow: f32,     // Eyebrows (G == 110)
    eyeshadow: f32,   // Upper eyelid eyeshadow zone (G == 125)
    cheek: f32,       // Left & right cheek blush zones (G == 135)
    eye: f32,         // Sclera + Iris eye opening (G == 160)
    lips_outer: f32,  // Lipstick zone (G == 205), STRICTLY 0.0 on inner mouth teeth
    teeth_inner: f32, // Inner mouth cavity for teeth whitening (G == 245)
    hair: f32,        // Neural hair segmentation mask (B channel)
    clothing: f32,    // Neural clothing segmentation mask (A == 128)
    background: f32,  // Neural background segmentation mask (A == 0)
}

const MASK_DIM: usize = 512;

#[inline]
fn compute_geometric_weights(nx: f32, ny: f32, face: &FaceRegion) -> SemanticWeights {
    let fdx = (nx - face.center_x) / face.face_rx.max(0.05);
    let fdy = (ny - face.center_y) / face.face_ry.max(0.05);
    let f_d2 = fdx * fdx + fdy * fdy;

    let le_d2 = ((nx - face.left_eye_x) / (face.face_rx * 0.20)).powi(2)
        + ((ny - face.left_eye_y) / (face.face_ry * 0.09)).powi(2);
    let re_d2 = ((nx - face.right_eye_x) / (face.face_rx * 0.20)).powi(2)
        + ((ny - face.right_eye_y) / (face.face_ry * 0.09)).powi(2);
    let eye = if le_d2 < 1.0 || re_d2 < 1.0 { 1.0 } else { 0.0 };

    let lb_d2 = ((nx - face.left_eye_x) / (face.face_rx * 0.24)).powi(2)
        + ((ny - (face.left_eye_y - face.face_ry * 0.12)) / (face.face_ry * 0.06)).powi(2);
    let rb_d2 = ((nx - face.right_eye_x) / (face.face_rx * 0.24)).powi(2)
        + ((ny - (face.right_eye_y - face.face_ry * 0.12)) / (face.face_ry * 0.06)).powi(2);
    let eyebrow = if lb_d2 < 1.0 || rb_d2 < 1.0 { 1.0 } else { 0.0 };

    let les_d2 = ((nx - face.left_eye_x) / (face.face_rx * 0.22)).powi(2)
        + ((ny - (face.left_eye_y - face.face_ry * 0.065)) / (face.face_ry * 0.048)).powi(2);
    let res_d2 = ((nx - face.right_eye_x) / (face.face_rx * 0.22)).powi(2)
        + ((ny - (face.right_eye_y - face.face_ry * 0.065)) / (face.face_ry * 0.048)).powi(2);
    let eyeshadow = if (les_d2 < 1.0 || res_d2 < 1.0) && eye == 0.0 && eyebrow == 0.0 {
        1.0
    } else {
        0.0
    };

    let m_outer_d2 = ((nx - face.mouth_x) / (face.face_rx * 0.28)).powi(2)
        + ((ny - face.mouth_y) / (face.face_ry * 0.10)).powi(2);
    let m_inner_d2 = ((nx - face.mouth_x) / (face.face_rx * 0.19)).powi(2)
        + ((ny - face.mouth_y) / (face.face_ry * 0.048)).powi(2);
    let teeth_inner = if m_inner_d2 < 1.0 { 1.0 } else { 0.0 };
    let lips_outer = if m_outer_d2 < 1.0 && m_inner_d2 >= 1.0 { 1.0 } else { 0.0 };

    let lu_d2 = ((nx - face.left_eye_x) / (face.face_rx * 0.22)).powi(2)
        + ((ny - (face.left_eye_y + face.face_ry * 0.11)) / (face.face_ry * 0.075)).powi(2);
    let ru_d2 = ((nx - face.right_eye_x) / (face.face_rx * 0.22)).powi(2)
        + ((ny - (face.right_eye_y + face.face_ry * 0.11)) / (face.face_ry * 0.075)).powi(2);
    let under_eye = if (lu_d2 < 1.0 || ru_d2 < 1.0) && eye == 0.0 { 1.0 } else { 0.0 };

    let l_cheek_d2 = ((nx - (face.center_x - face.face_rx * 0.50)) / (face.face_rx * 0.28)).powi(2)
        + ((ny - (face.center_y + face.face_ry * 0.08)) / (face.face_ry * 0.18)).powi(2);
    let r_cheek_d2 = ((nx - (face.center_x + face.face_rx * 0.50)) / (face.face_rx * 0.28)).powi(2)
        + ((ny - (face.center_y + face.face_ry * 0.08)) / (face.face_ry * 0.18)).powi(2);
    let cheek = if l_cheek_d2 < 1.0 || r_cheek_d2 < 1.0 { 1.0 } else { 0.0 };

    let l_nf_d2 = ((nx - (face.nose_x - face.face_rx * 0.24)) / (face.face_rx * 0.14)).powi(2)
        + ((ny - (face.nose_y + face.face_ry * 0.16)) / (face.face_ry * 0.16)).powi(2);
    let r_nf_d2 = ((nx - (face.nose_x + face.face_rx * 0.24)) / (face.face_rx * 0.14)).powi(2)
        + ((ny - (face.nose_y + face.face_ry * 0.16)) / (face.face_ry * 0.16)).powi(2);
    let nasolabial = if (l_nf_d2 < 1.0 || r_nf_d2 < 1.0) && m_outer_d2 >= 1.0 {
        1.0
    } else {
        0.0
    };

    let fh_d2 = ((nx - face.center_x) / (face.face_rx * 0.75)).powi(2)
        + ((ny - (face.center_y - face.face_ry * 0.55)) / (face.face_ry * 0.26)).powi(2);
    let forehead = if fh_d2 < 1.0 && eyebrow == 0.0 { 1.0 } else { 0.0 };

    let neck_d2 = ((nx - face.center_x) / (face.face_rx * 0.65)).powi(2)
        + ((ny - (face.chin_y + face.face_ry * 0.28)) / (face.face_ry * 0.35)).powi(2);
    let skin = if (f_d2 < 1.05 || (neck_d2 < 1.0 && ny > face.chin_y * 0.92))
        && eye == 0.0
        && eyebrow == 0.0
        && m_outer_d2 >= 1.05
    {
        if f_d2 < 1.05 {
            (1.0 - f_d2 * 0.25).clamp(0.2, 1.0)
        } else {
            (1.0 - neck_d2 * 0.35).clamp(0.2, 1.0)
        }
    } else {
        0.0
    };

    let head_d2 = ((nx - face.center_x) / (face.face_rx * 1.55)).powi(2)
        + ((ny - (face.center_y - face.face_ry * 0.10)) / (face.face_ry * 1.48)).powi(2);
    let torso_d2 = ((nx - face.center_x) / (face.face_rx * 2.45)).powi(2)
        + ((ny - (face.center_y + face.face_ry * 2.05)) / (face.face_ry * 1.75)).powi(2);

    SemanticWeights {
        skin,
        forehead,
        under_eye,
        nasolabial,
        eyebrow,
        eyeshadow,
        cheek,
        eye,
        lips_outer,
        teeth_inner,
        hair: if head_d2 >= 0.72 && head_d2 < 1.65 && ny < face.mouth_y + 0.04 {
            1.0
        } else {
            0.0
        },
        clothing: if torso_d2 < 1.05 && ny > face.chin_y + face.face_ry * 0.18 {
            1.0
        } else {
            0.0
        },
        background: if head_d2 > 1.42 && torso_d2 > 1.05 {
            1.0
        } else {
            0.0
        },
    }
}

fn build_semantic_grid(mask_img: Option<&RgbaImage>, face: &FaceRegion) -> Vec<SemanticWeights> {
    let mut raw_grid = vec![SemanticWeights::default(); MASK_DIM * MASK_DIM];

    if let Some(m_img) = mask_img {
        let mw = m_img.width() as usize;
        let mh = m_img.height() as usize;
        raw_grid
            .par_chunks_mut(MASK_DIM)
            .enumerate()
            .for_each(|(y, row)| {
                let sy = ((y * mh) / MASK_DIM).min(mh.saturating_sub(1)) as u32;
                let ny = y as f32 / MASK_DIM as f32;
                for x in 0..MASK_DIM {
                    let sx = ((x * mw) / MASK_DIM).min(mw.saturating_sub(1)) as u32;
                    let nx = x as f32 / MASK_DIM as f32;
                    let p = m_img.get_pixel(sx, sy);
                    let r_skin = p[0] as f32 / 255.0;
                    let g_code = p[1];
                    let b_hair = p[2] as f32 / 255.0;
                    let a_scene = p[3];
                    let geo = compute_geometric_weights(nx, ny, face);

                    row[x] = SemanticWeights {
                        skin: r_skin.max(geo.skin * 0.35),
                        forehead: if g_code.abs_diff(30) <= 6 { 1.0 } else { geo.forehead * 0.5 },
                        under_eye: if g_code.abs_diff(60) <= 6 { 1.0 } else { geo.under_eye * 0.5 },
                        nasolabial: if g_code.abs_diff(85) <= 6 { 1.0 } else { geo.nasolabial * 0.6 },
                        eyebrow: if g_code.abs_diff(110) <= 6 { 1.0 } else { geo.eyebrow * 0.5 },
                        eyeshadow: if g_code.abs_diff(125) <= 6 { 1.0 } else { geo.eyeshadow * 0.6 },
                        cheek: if g_code.abs_diff(135) <= 6 { 1.0 } else { geo.cheek * 0.6 },
                        eye: if g_code.abs_diff(160) <= 6 { 1.0 } else { geo.eye * 0.5 },
                        lips_outer: if g_code.abs_diff(205) <= 6 { 1.0 } else { geo.lips_outer * 0.5 },
                        teeth_inner: if g_code.abs_diff(245) <= 6 { 1.0 } else { geo.teeth_inner * 0.5 },
                        hair: b_hair.max(geo.hair * 0.45),
                        clothing: if a_scene.abs_diff(128) <= 28 { 1.0 } else { geo.clothing * 0.45 },
                        background: if a_scene < 40 { 1.0 } else { geo.background * 0.45 },
                    };
                }
            });
    } else {
        raw_grid
            .par_chunks_mut(MASK_DIM)
            .enumerate()
            .for_each(|(y, row)| {
                let ny = y as f32 / MASK_DIM as f32;
                for x in 0..MASK_DIM {
                    let nx = x as f32 / MASK_DIM as f32;
                    row[x] = compute_geometric_weights(nx, ny, face);
                }
            });
    }

    // Separable 7-tap box blur on the 512x512 SemanticWeights grid for silky-smooth feathering without label crosstalk
    let rad: isize = 3;
    let mut temp = vec![SemanticWeights::default(); MASK_DIM * MASK_DIM];
    temp.par_chunks_mut(MASK_DIM)
        .enumerate()
        .for_each(|(y, row_out)| {
            let row_in = &raw_grid[y * MASK_DIM..(y + 1) * MASK_DIM];
            for x in 0..MASK_DIM {
                let x0 = (x as isize - rad).max(0) as usize;
                let x1 = (x as isize + rad).min(MASK_DIM as isize - 1) as usize;
                let inv = 1.0 / ((x1 - x0 + 1) as f32);
                let mut acc = SemanticWeights::default();
                for ix in x0..=x1 {
                    let w = row_in[ix];
                    acc.skin += w.skin;
                    acc.forehead += w.forehead;
                    acc.under_eye += w.under_eye;
                    acc.nasolabial += w.nasolabial;
                    acc.eyebrow += w.eyebrow;
                    acc.eyeshadow += w.eyeshadow;
                    acc.cheek += w.cheek;
                    acc.eye += w.eye;
                    acc.lips_outer += w.lips_outer;
                    acc.teeth_inner += w.teeth_inner;
                    acc.hair += w.hair;
                    acc.clothing += w.clothing;
                    acc.background += w.background;
                }
                row_out[x] = SemanticWeights {
                    skin: acc.skin * inv,
                    forehead: acc.forehead * inv,
                    under_eye: acc.under_eye * inv,
                    nasolabial: acc.nasolabial * inv,
                    eyebrow: acc.eyebrow * inv,
                    eyeshadow: acc.eyeshadow * inv,
                    cheek: acc.cheek * inv,
                    eye: acc.eye * inv,
                    lips_outer: acc.lips_outer * inv,
                    teeth_inner: acc.teeth_inner * inv,
                    hair: acc.hair * inv,
                    clothing: acc.clothing * inv,
                    background: acc.background * inv,
                };
            }
        });

    let mut final_grid = vec![SemanticWeights::default(); MASK_DIM * MASK_DIM];
    final_grid
        .par_chunks_mut(MASK_DIM)
        .enumerate()
        .for_each(|(y, row_out)| {
            let y0 = (y as isize - rad).max(0) as usize;
            let y1 = (y as isize + rad).min(MASK_DIM as isize - 1) as usize;
            let inv = 1.0 / ((y1 - y0 + 1) as f32);
            for x in 0..MASK_DIM {
                let mut acc = SemanticWeights::default();
                for iy in y0..=y1 {
                    let w = temp[iy * MASK_DIM + x];
                    acc.skin += w.skin;
                    acc.forehead += w.forehead;
                    acc.under_eye += w.under_eye;
                    acc.nasolabial += w.nasolabial;
                    acc.eyebrow += w.eyebrow;
                    acc.eyeshadow += w.eyeshadow;
                    acc.cheek += w.cheek;
                    acc.eye += w.eye;
                    acc.lips_outer += w.lips_outer;
                    acc.teeth_inner += w.teeth_inner;
                    acc.hair += w.hair;
                    acc.clothing += w.clothing;
                    acc.background += w.background;
                }
                let eye = acc.eye * inv;
                let eyebrow = acc.eyebrow * inv;
                let lips_outer = (acc.lips_outer * inv * (1.0 - (acc.teeth_inner * inv * 1.5).min(1.0))).clamp(0.0, 1.0);
                let teeth_inner = acc.teeth_inner * inv;

                // STRICT ISOLATION: Skin, Under-Eye, and Eyeshadow masks are forced to 0.0 on Eyes, Eyebrows, Lips, and Teeth!
                let feature_guard = (eye * 2.2 + eyebrow * 1.8 + lips_outer * 2.2 + teeth_inner * 2.5).clamp(0.0, 1.0);
                let skin = (acc.skin * inv * (1.0 - feature_guard)).clamp(0.0, 1.0);
                let under_eye = (acc.under_eye * inv * (1.0 - (eye * 2.5).min(1.0))).clamp(0.0, 1.0);
                let eyeshadow = (acc.eyeshadow * inv * (1.0 - (eye * 2.5).min(1.0)) * (1.0 - (eyebrow * 1.8).min(1.0))).clamp(0.0, 1.0);

                row_out[x] = SemanticWeights {
                    skin,
                    forehead: (acc.forehead * inv * (1.0 - (eyebrow * 1.8).min(1.0))).clamp(0.0, 1.0),
                    under_eye,
                    nasolabial: acc.nasolabial * inv * (1.0 - (lips_outer * 1.8).min(1.0)),
                    eyebrow,
                    eyeshadow,
                    cheek: acc.cheek * inv * skin.max(0.25),
                    eye,
                    lips_outer,
                    teeth_inner,
                    hair: acc.hair * inv * (1.0 - skin * 0.8),
                    clothing: acc.clothing * inv * (1.0 - skin * 0.6),
                    background: acc.background * inv * (1.0 - skin),
                };
            }
        });

    final_grid
}

#[inline]
fn sample_semantic_grid(grid: &[SemanticWeights], nx: f32, ny: f32) -> SemanticWeights {
    let fx = (nx * (MASK_DIM - 1) as f32).clamp(0.0, (MASK_DIM - 1) as f32);
    let fy = (ny * (MASK_DIM - 1) as f32).clamp(0.0, (MASK_DIM - 1) as f32);
    let x0 = fx.floor() as usize;
    let y0 = fy.floor() as usize;
    let x1 = (x0 + 1).min(MASK_DIM - 1);
    let y1 = (y0 + 1).min(MASK_DIM - 1);
    let dx = fx - x0 as f32;
    let dy = fy - y0 as f32;

    let w00 = (1.0 - dx) * (1.0 - dy);
    let w10 = dx * (1.0 - dy);
    let w01 = (1.0 - dx) * dy;
    let w11 = dx * dy;

    let a = grid[y0 * MASK_DIM + x0];
    let b = grid[y0 * MASK_DIM + x1];
    let c = grid[y1 * MASK_DIM + x0];
    let d = grid[y1 * MASK_DIM + x1];

    SemanticWeights {
        skin: a.skin * w00 + b.skin * w10 + c.skin * w01 + d.skin * w11,
        forehead: a.forehead * w00 + b.forehead * w10 + c.forehead * w01 + d.forehead * w11,
        under_eye: a.under_eye * w00 + b.under_eye * w10 + c.under_eye * w01 + d.under_eye * w11,
        nasolabial: a.nasolabial * w00 + b.nasolabial * w10 + c.nasolabial * w01 + d.nasolabial * w11,
        eyebrow: a.eyebrow * w00 + b.eyebrow * w10 + c.eyebrow * w01 + d.eyebrow * w11,
        eyeshadow: a.eyeshadow * w00 + b.eyeshadow * w10 + c.eyeshadow * w01 + d.eyeshadow * w11,
        cheek: a.cheek * w00 + b.cheek * w10 + c.cheek * w01 + d.cheek * w11,
        eye: a.eye * w00 + b.eye * w10 + c.eye * w01 + d.eye * w11,
        lips_outer: a.lips_outer * w00 + b.lips_outer * w10 + c.lips_outer * w01 + d.lips_outer * w11,
        teeth_inner: a.teeth_inner * w00 + b.teeth_inner * w10 + c.teeth_inner * w01 + d.teeth_inner * w11,
        hair: a.hair * w00 + b.hair * w10 + c.hair * w01 + d.hair * w11,
        clothing: a.clothing * w00 + b.clothing * w10 + c.clothing * w01 + d.clothing * w11,
        background: a.background * w00 + b.background * w10 + c.background * w01 + d.background * w11,
    }
}

#[inline]
fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;
    let h = if delta < 1e-5 {
        0.0
    } else if (max - r).abs() < 1e-5 {
        60.0 * (((g - b) / delta).rem_euclid(6.0))
    } else if (max - g).abs() < 1e-5 {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };
    let s = if max < 1e-5 { 0.0 } else { delta / max };
    (h, s, max)
}

#[inline]
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let c = v * s;
    let hh = (h.rem_euclid(360.0)) / 60.0;
    let x = c * (1.0 - (hh.rem_euclid(2.0) - 1.0).abs());
    let m = v - c;
    let (r1, g1, b1) = if hh < 1.0 {
        (c, x, 0.0)
    } else if hh < 2.0 {
        (x, c, 0.0)
    } else if hh < 3.0 {
        (0.0, c, x)
    } else if hh < 4.0 {
        (0.0, x, c)
    } else if hh < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    (r1 + m, g1 + m, b1 + m)
}

// =========================================================================
// LIGHTCRAFT COLOR SCIENCE: OkLab / OkLCh, 8-Band Partition, Calibration,
// Parametric + Point Curves, 4-Way Color Grading, Grain & Vignette
// =========================================================================
#[inline]
fn srgb_to_oklab(r: f32, g: f32, b: f32) -> [f32; 3] {
    let rl = r.max(0.0);
    let gl = g.max(0.0);
    let bl = b.max(0.0);
    let l = (0.41222146 * rl + 0.53633255 * gl + 0.051445995 * bl).cbrt();
    let m = (0.2119035 * rl + 0.6806995 * gl + 0.10739696 * bl).cbrt();
    let s = (0.08830246 * rl + 0.28171885 * gl + 0.6299787 * bl).cbrt();
    [
        0.21045426 * l + 0.7936178 * m - 0.004072047 * s,
        1.9779985 * l - 2.4285922 * m + 0.4505937 * s,
        0.025904037 * l + 0.78277177 * m - 0.80867577 * s,
    ]
}

#[inline]
fn oklab_to_srgb(lab: [f32; 3]) -> (f32, f32, f32) {
    let [l_, a, b] = lab;
    let l = (l_ + 0.39633778 * a + 0.21580376 * b).powi(3);
    let m = (l_ - 0.105561346 * a - 0.06385417 * b).powi(3);
    let s = (l_ - 0.08948418 * a - 1.2914855 * b).powi(3);
    (
        (4.0767417 * l - 3.3077116 * m + 0.23096994 * s).max(0.0),
        (-1.268438 * l + 2.6097574 * m - 0.34131938 * s).max(0.0),
        (-0.0041960863 * l - 0.7034186 * m + 1.7076147 * s).max(0.0),
    )
}

#[inline]
fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    if (e1 - e0).abs() < 1e-6 {
        return if x >= e1 { 1.0 } else { 0.0 };
    }
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

const LC_HUE_CENTRES: [f32; 8] = [22.0, 52.0, 92.0, 142.0, 195.0, 250.0, 298.0, 342.0];

#[inline]
fn lc_band_weights(h_deg: f32) -> [f32; 8] {
    let h = h_deg.rem_euclid(360.0);
    let mut i0 = 7usize;
    let mut i1 = 0usize;
    for i in 0..8 {
        let j = (i + 1) % 8;
        let c0 = LC_HUE_CENTRES[i];
        let c1 = if j == 0 { LC_HUE_CENTRES[0] + 360.0 } else { LC_HUE_CENTRES[j] };
        let hh = if j == 0 && h < LC_HUE_CENTRES[0] { h + 360.0 } else { h };
        if hh >= c0 && hh < c1 {
            i0 = i;
            i1 = j;
            break;
        }
    }
    let c0 = LC_HUE_CENTRES[i0];
    let c1 = if i1 == 0 { LC_HUE_CENTRES[0] + 360.0 } else { LC_HUE_CENTRES[i1] };
    let hh = if i1 == 0 && h < LC_HUE_CENTRES[0] { h + 360.0 } else { h };
    let u = ((hh - c0) / (c1 - c0)).clamp(0.0, 1.0);
    let w1 = 0.5 - 0.5 * (u * std::f32::consts::PI).cos();
    let mut out = [0.0f32; 8];
    out[i0] = 1.0 - w1;
    out[i1] = w1;
    out
}

fn build_calibration_matrix(params: &RetouchParams) -> Option<[[f32; 3]; 3]> {
    let active = params.cal_red_hue.abs() > 0.2
        || params.cal_red_sat.abs() > 0.2
        || params.cal_green_hue.abs() > 0.2
        || params.cal_green_sat.abs() > 0.2
        || params.cal_blue_hue.abs() > 0.2
        || params.cal_blue_sat.abs() > 0.2;
    if !active {
        return None;
    }
    let rh = params.cal_red_hue / 100.0 * 0.22;
    let rs = 1.0 + params.cal_red_sat / 100.0 * 0.35;
    let gh = params.cal_green_hue / 100.0 * 0.22;
    let gs = 1.0 + params.cal_green_sat / 100.0 * 0.35;
    let bh = params.cal_blue_hue / 100.0 * 0.22;
    let bs = 1.0 + params.cal_blue_sat / 100.0 * 0.35;

    let mut m = [
        [rs, -gh * gs * 0.5, bh * bs * 0.5],
        [rh * rs * 0.6, gs, -bh * bs * 0.6],
        [-rh * rs * 0.4, gh * gs * 0.6, bs],
    ];
    for row in &mut m {
        let sum: f32 = row.iter().sum();
        if sum.abs() > 1e-5 {
            for v in row.iter_mut() {
                *v /= sum;
            }
        }
    }
    Some(m)
}

#[inline]
fn eval_parametric_curve(x: f32, params: &RetouchParams) -> f32 {
    if params.curve_shadows.abs() < 0.2
        && params.curve_darks.abs() < 0.2
        && params.curve_lights.abs() < 0.2
        && params.curve_highlights.abs() < 0.2
    {
        return x;
    }
    let s0 = (params.curve_split_shadows / 100.0).clamp(0.08, 0.45);
    let s1 = (params.curve_split_mid / 100.0).clamp(s0 + 0.08, 0.72);
    let s2 = (params.curve_split_highlights / 100.0).clamp(s1 + 0.08, 0.92);

    let bell = |c: f32, w: f32| -> f32 {
        let d = ((x - c) / w).abs();
        if d >= 1.0 {
            0.0
        } else {
            let u = 1.0 - d * d;
            u * u
        }
    };
    let end_taper = (4.0 * x * (1.0 - x)).clamp(0.0, 1.0).sqrt();
    let dy = (params.curve_shadows / 100.0) * 0.22 * bell(s0 * 0.55, s0 * 0.95)
        + (params.curve_darks / 100.0) * 0.22 * bell((s0 + s1) * 0.5, (s1 - s0) * 1.1)
        + (params.curve_lights / 100.0) * 0.22 * bell((s1 + s2) * 0.5, (s2 - s1) * 1.1)
        + (params.curve_highlights / 100.0) * 0.22 * bell((s2 + 1.0) * 0.5, (1.0 - s2) * 0.95);
    (x + dy * end_taper).clamp(0.0, 1.0)
}

#[inline]
fn sample_curve_lut(lut: &[u8], v: f32) -> f32 {
    if lut.len() != 256 {
        return v;
    }
    let scaled = (v.clamp(0.0, 1.0) * 255.0).clamp(0.0, 255.0);
    let i0 = scaled.floor() as usize;
    let i1 = (i0 + 1).min(255);
    let frac = scaled - i0 as f32;
    let v0 = lut[i0] as f32 / 255.0;
    let v1 = lut[i1] as f32 / 255.0;
    v0 * (1.0 - frac) + v1 * frac
}

#[inline]
fn hash_u32(mut x: u32) -> u32 {
    x ^= x >> 16;
    x = x.wrapping_mul(0x7feb_352d);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846c_a68b);
    x ^= x >> 16;
    x
}

#[inline]
fn hash2(ix: i32, iy: i32, seed: u32) -> f32 {
    let h = hash_u32((ix as u32).wrapping_mul(374_761_393) ^ (iy as u32).wrapping_mul(668_265_263) ^ seed);
    (h as f32) * (1.0 / u32::MAX as f32) - 0.5
}

#[inline]
fn value_noise(x: f32, y: f32, seed: u32) -> f32 {
    let ix = x.floor() as i32;
    let iy = y.floor() as i32;
    let fx = x - ix as f32;
    let fy = y - iy as f32;
    let ux = fx * fx * (3.0 - 2.0 * fx);
    let uy = fy * fy * (3.0 - 2.0 * fy);
    let a = hash2(ix, iy, seed);
    let b = hash2(ix + 1, iy, seed);
    let c = hash2(ix, iy + 1, seed);
    let d = hash2(ix + 1, iy + 1, seed);
    let ab = a + (b - a) * ux;
    let cd = c + (d - c) * ux;
    ab + (cd - ab) * uy
}

#[inline]
fn lc_grain_noise(px: f32, py: f32, amount: f32, size: f32, roughness: f32) -> f32 {
    let freq = 1.0 / (0.6 + (size / 100.0).clamp(0.0, 1.0) * 2.6);
    let n1 = value_noise(px * freq, py * freq, 0xA341_316C);
    let n2 = value_noise(px * freq * 2.13, py * freq * 2.13, 0xC801_3EA4);
    let rough = (roughness / 100.0).clamp(0.0, 1.0);
    let n = n1 * (1.0 - 0.45 * rough) + n2 * (0.65 * rough);
    n * (amount / 100.0) * 0.22
}

#[inline]
fn skin_weight(r: f32, g: f32, b: f32) -> f32 {
    let r255 = r * 255.0;
    let g255 = g * 255.0;
    let b255 = b * 255.0;
    let cb = 128.0 - 0.168736 * r255 - 0.331264 * g255 + 0.5 * b255;
    let cr = 128.0 + 0.5 * r255 - 0.418688 * g255 - 0.081312 * b255;
    let cb_dist = ((cb - 112.0) / 26.0).abs();
    let cr_dist = ((cr - 152.0) / 28.0).abs();
    let chroma_score = (1.0 - (cb_dist * cb_dist + cr_dist * cr_dist).sqrt()).clamp(0.0, 1.0);
    let (h, s, v) = rgb_to_hsv(r, g, b);
    let hue_ok = if h <= 54.0 || h >= 330.0 { 1.0 } else { 0.0 };
    let sat_ok = if s >= 0.06 && s <= 0.82 && v >= 0.10 { 1.0 } else { 0.25 };
    (chroma_score * hue_ok * sat_ok).clamp(0.0, 1.0)
}

pub fn detect_face_region(img: &RgbaImage) -> FaceRegion {
    let (w, h) = img.dimensions();
    if w == 0 || h == 0 {
        return FaceRegion::default();
    }
    let mut sum_x = 0.0f64;
    let mut sum_y = 0.0f64;
    let mut sum_w = 0.0f64;
    let step_x = (w / 120).max(1);
    let step_y = (h / 120).max(1);

    for y in (0..h).step_by(step_y as usize) {
        let ny = y as f32 / h as f32;
        if ny < 0.06 || ny > 0.80 {
            continue;
        }
        for x in (0..w).step_by(step_x as usize) {
            let nx = x as f32 / w as f32;
            if nx < 0.10 || nx > 0.90 {
                continue;
            }
            let p = img.get_pixel(x, y);
            let sw = skin_weight(p[0] as f32 / 255.0, p[1] as f32 / 255.0, p[2] as f32 / 255.0);
            let prior = (1.0 - ((nx - 0.5) * 1.35).powi(2)).max(0.1)
                * (1.0 - ((ny - 0.38) * 1.55).powi(2)).max(0.1);
            let weight = (sw * prior) as f64;
            if weight > 0.16 {
                sum_x += nx as f64 * weight;
                sum_y += ny as f64 * weight;
                sum_w += weight;
            }
        }
    }

    if sum_w < 3.0 {
        return FaceRegion::default();
    }
    let cx = (sum_x / sum_w) as f32;
    let cy = (sum_y / sum_w) as f32;

    let mut var_x = 0.0f64;
    let mut var_y = 0.0f64;
    for y in (0..h).step_by(step_y as usize) {
        let ny = y as f32 / h as f32;
        if ny < 0.06 || ny > 0.80 {
            continue;
        }
        for x in (0..w).step_by(step_x as usize) {
            let nx = x as f32 / w as f32;
            let p = img.get_pixel(x, y);
            let sw = skin_weight(p[0] as f32 / 255.0, p[1] as f32 / 255.0, p[2] as f32 / 255.0);
            if sw > 0.30 && (nx - cx).abs() < 0.35 && (ny - cy).abs() < 0.38 {
                var_x += ((nx - cx) as f64).powi(2) * sw as f64;
                var_y += ((ny - cy) as f64).powi(2) * sw as f64;
            }
        }
    }

    let rx = ((var_x / sum_w).sqrt() as f32 * 1.75).clamp(0.10, 0.28);
    let ry = ((var_y / sum_w).sqrt() as f32 * 1.85).clamp(0.14, 0.35);

    FaceRegion {
        center_x: cx,
        center_y: cy,
        face_rx: rx,
        face_ry: ry,
        left_eye_x: (cx - rx * 0.36).clamp(0.05, 0.95),
        left_eye_y: (cy - ry * 0.16).clamp(0.05, 0.95),
        right_eye_x: (cx + rx * 0.36).clamp(0.05, 0.95),
        right_eye_y: (cy - ry * 0.16).clamp(0.05, 0.95),
        nose_x: cx,
        nose_y: (cy + ry * 0.06).clamp(0.05, 0.95),
        mouth_x: cx,
        mouth_y: (cy + ry * 0.38).clamp(0.05, 0.95),
        chin_x: cx,
        chin_y: (cy + ry * 0.95).clamp(0.05, 0.98),
        forehead_x: cx,
        forehead_y: (cy - ry * 0.92).clamp(0.02, 0.95),
        jaw_left_x: (cx - rx * 0.82).clamp(0.05, 0.95),
        jaw_left_y: (cy + ry * 0.52).clamp(0.05, 0.95),
        jaw_right_x: (cx + rx * 0.82).clamp(0.05, 0.95),
        jaw_right_y: (cy + ry * 0.52).clamp(0.05, 0.95),
        cheek_left_x: (cx - rx * 0.88).clamp(0.05, 0.95),
        cheek_left_y: (cy + ry * 0.06).clamp(0.05, 0.95),
        cheek_right_x: (cx + rx * 0.88).clamp(0.05, 0.95),
        cheek_right_y: (cy + ry * 0.06).clamp(0.05, 0.95),
        mouth_left_x: (cx - rx * 0.28).clamp(0.05, 0.95),
        mouth_left_y: (cy + ry * 0.38).clamp(0.05, 0.95),
        mouth_right_x: (cx + rx * 0.28).clamp(0.05, 0.95),
        mouth_right_y: (cy + ry * 0.38).clamp(0.05, 0.95),
    }
}

fn separable_blur(src: &[f32], w: usize, h: usize, radius: usize) -> Vec<f32> {
    if radius == 0 || w == 0 || h == 0 {
        return src.to_vec();
    }
    let mut temp = vec![0.0f32; w * h * 3];
    let mut dst = vec![0.0f32; w * h * 3];
    let r = radius as isize;

    temp.par_chunks_mut(w * 3)
        .enumerate()
        .for_each(|(y, row_out)| {
            let row_in = &src[y * w * 3..(y + 1) * w * 3];
            for x in 0..w {
                let mut sr = 0.0;
                let mut sg = 0.0;
                let mut sb = 0.0;
                let x0 = (x as isize - r).max(0) as usize;
                let x1 = (x as isize + r).min(w as isize - 1) as usize;
                let count = (x1 - x0 + 1) as f32;
                for ix in x0..=x1 {
                    let idx = ix * 3;
                    sr += row_in[idx];
                    sg += row_in[idx + 1];
                    sb += row_in[idx + 2];
                }
                let out_idx = x * 3;
                row_out[out_idx] = sr / count;
                row_out[out_idx + 1] = sg / count;
                row_out[out_idx + 2] = sb / count;
            }
        });

    dst.par_chunks_mut(w * 3)
        .enumerate()
        .for_each(|(y, row_out)| {
            let y0 = (y as isize - r).max(0) as usize;
            let y1 = (y as isize + r).min(h as isize - 1) as usize;
            let count = (y1 - y0 + 1) as f32;
            for x in 0..w {
                let mut sr = 0.0;
                let mut sg = 0.0;
                let mut sb = 0.0;
                for iy in y0..=y1 {
                    let idx = (iy * w + x) * 3;
                    sr += temp[idx];
                    sg += temp[idx + 1];
                    sb += temp[idx + 2];
                }
                let out_idx = x * 3;
                row_out[out_idx] = sr / count;
                row_out[out_idx + 1] = sg / count;
                row_out[out_idx + 2] = sb / count;
            }
        });

    dst
}

#[inline]
fn sample_bilinear(buf: &[f32], w: usize, h: usize, fx: f32, fy: f32) -> (f32, f32, f32) {
    let x = fx.clamp(0.0, (w - 1) as f32);
    let y = fy.clamp(0.0, (h - 1) as f32);
    let x0 = x.floor() as usize;
    let y0 = y.floor() as usize;
    let x1 = (x0 + 1).min(w - 1);
    let y1 = (y0 + 1).min(h - 1);
    let dx = x - x0 as f32;
    let dy = y - y0 as f32;

    let i00 = (y0 * w + x0) * 3;
    let i10 = (y0 * w + x1) * 3;
    let i01 = (y1 * w + x0) * 3;
    let i11 = (y1 * w + x1) * 3;

    let mut out = [0.0f32; 3];
    for c in 0..3 {
        let top = buf[i00 + c] * (1.0 - dx) + buf[i10 + c] * dx;
        let bot = buf[i01 + c] * (1.0 - dx) + buf[i11 + c] * dx;
        out[c] = top * (1.0 - dy) + bot * dy;
    }
    (out[0], out[1], out[2])
}

/// Adaptive denoiser for RAW and JPG: estimates the noise level of the photo itself,
/// then applies non-local means (patch matching, edge preserving) and chroma smoothing.
fn adaptive_denoise_rgb(src: &[f32], w: usize, h: usize, strength: f32, detail: f32) -> Vec<f32> {
    let s = (strength / 100.0).clamp(0.0, 1.0);
    if s < 0.005 || w < 8 || h < 8 {
        return src.to_vec();
    }
    let luma: Vec<f32> = (0..w * h)
        .map(|i| 0.299 * src[i * 3] + 0.587 * src[i * 3 + 1] + 0.114 * src[i * 3 + 2])
        .collect();

    // Noise estimate: robust MAD of (pixel - local 4-neighbour mean) on a sparse grid
    let mut devs: Vec<f32> = Vec::new();
    let step = ((w * h) / 20000).max(1);
    let mut i = w + 1;
    while i < w * h - w - 1 {
        if i % w != 0 && i % w != w - 1 {
            let m = (luma[i - 1] + luma[i + 1] + luma[i - w] + luma[i + w]) * 0.25;
            devs.push((luma[i] - m).abs());
        }
        i += step;
    }
    devs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mad = if devs.is_empty() { 0.01 } else { devs[devs.len() / 2] };
    // 4-neighbour residual has variance sigma^2 * 1.25 -> sigma = MAD*1.4826/sqrt(1.25)
    let sigma = (mad * 1.4826 / 1.118).clamp(0.002, 0.25);
    let h_filter = (sigma * (0.6 + 1.6 * s)).max(1e-4);
    let h2 = h_filter * h_filter;
    let noise_floor = 2.0 * sigma * sigma;

    const R: i32 = 3; // search radius (7x7 window)
    let at = |x: i32, y: i32| -> usize {
        (y.clamp(0, h as i32 - 1) as usize) * w + x.clamp(0, w as i32 - 1) as usize
    };

    let mut out = vec![0.0f32; w * h * 3];
    out.par_chunks_mut(w * 3).enumerate().for_each(|(y, row)| {
        let yi = y as i32;
        for x in 0..w {
            let xi = x as i32;
            let mut acc = [0.0f32; 3];
            let mut wsum = 0.0f32;
            for dy in -R..=R {
                for dx in -R..=R {
                    let mut d2 = 0.0f32;
                    for py in -1..=1 {
                        for px in -1..=1 {
                            let a = luma[at(xi + px, yi + py)];
                            let b = luma[at(xi + dx + px, yi + dy + py)];
                            d2 += (a - b) * (a - b);
                        }
                    }
                    d2 /= 9.0;
                    let wgt = (-((d2 - noise_floor).max(0.0)) / h2).exp();
                    let j = at(xi + dx, yi + dy) * 3;
                    acc[0] += src[j] * wgt;
                    acc[1] += src[j + 1] * wgt;
                    acc[2] += src[j + 2] * wgt;
                    wsum += wgt;
                }
            }
            let o = x * 3;
            let idx = (y * w + x) * 3;
            // restore a share of original fine detail (Detail slider)
            let keep = (detail / 100.0).clamp(0.0, 1.0) * 0.30 * (1.0 - s * 0.5);
            for c in 0..3 {
                let nlm = acc[c] / wsum.max(1e-6);
                row[o + c] = nlm + (src[idx + c] - nlm) * keep;
            }
        }
    });

    // Chroma smoothing: colour blotches are removed more aggressively than luminance
    let blurred = separable_blur(&out, w, h, 2 + (s * 4.0) as usize);
    let k = (s * 0.85).clamp(0.0, 0.85);
    out.par_chunks_mut(3).enumerate().for_each(|(i, px)| {
        let l = 0.299 * px[0] + 0.587 * px[1] + 0.114 * px[2];
        let lb = 0.299 * blurred[i * 3] + 0.587 * blurred[i * 3 + 1] + 0.114 * blurred[i * 3 + 2];
        for c in 0..3 {
            let chroma = px[c] - l;
            let chroma_b = blurred[i * 3 + c] - lb;
            px[c] = l + chroma * (1.0 - k) + chroma_b * k;
        }
    });
    out
}

pub fn apply_retouch_pipeline(
    img: &RgbaImage,
    params: &RetouchParams,
    mask_img: Option<&RgbaImage>,
) -> RgbaImage {
    let (w_u32, h_u32) = img.dimensions();
    let w = w_u32 as usize;
    let h = h_u32 as usize;

    // Build the 512x512 pre-feathered Semantic Anatomical Mask Grid (aligned 1:1 with unwarped original image!)
    let semantic_grid = build_semantic_grid(mask_img, &params.face);

    // Load original image into normalized RGB float buffer (in original coordinates so masks align 1:1!)
    let mut base_rgb = vec![0.0f32; w * h * 3];
    base_rgb
        .par_chunks_mut(w * 3)
        .enumerate()
        .for_each(|(y, row)| {
            for x in 0..w {
                let p = img.get_pixel(x as u32, y as u32);
                let idx = x * 3;
                row[idx] = p[0] as f32 / 255.0;
                row[idx + 1] = p[1] as f32 / 255.0;
                row[idx + 2] = p[2] as f32 / 255.0;
            }
        });

    // Interactive Manual Spot Healing (in original image coordinates)
    if !params.blemish_spots.is_empty() {
        let snapshot = base_rgb.clone();
        for spot in &params.blemish_spots {
            let cx = spot.x * w as f32;
            let cy = spot.y * h as f32;
            let rad = (spot.radius * w.min(h) as f32).max(4.0);
            let x0 = ((cx - rad).floor().max(0.0)) as usize;
            let x1 = ((cx + rad).ceil().min((w - 1) as f32)) as usize;
            let y0 = ((cy - rad).floor().max(0.0)) as usize;
            let y1 = ((cy + rad).ceil().min((h - 1) as f32)) as usize;

            let mut ring_r = 0.0f32;
            let mut ring_g = 0.0f32;
            let mut ring_b = 0.0f32;
            let samples = 16;
            for i in 0..samples {
                let angle = (i as f32 / samples as f32) * std::f32::consts::TAU;
                let sx = cx + angle.cos() * (rad * 1.35);
                let sy = cy + angle.sin() * (rad * 1.35);
                let (sr, sg, sb) = sample_bilinear(&snapshot, w, h, sx, sy);
                ring_r += sr;
                ring_g += sg;
                ring_b += sb;
            }
            ring_r /= samples as f32;
            ring_g /= samples as f32;
            ring_b /= samples as f32;

            for y in y0..=y1 {
                for x in x0..=x1 {
                    let dx = x as f32 - cx;
                    let dy = y as f32 - cy;
                    let dist = (dx * dx + dy * dy).sqrt();
                    if dist < rad {
                        let t = (1.0 - (dist / rad).powi(2)).clamp(0.0, 1.0);
                        let idx = (y * w + x) * 3;
                        base_rgb[idx] = base_rgb[idx] * (1.0 - t) + ring_r * t;
                        base_rgb[idx + 1] = base_rgb[idx + 1] * (1.0 - t) + ring_g * t;
                        base_rgb[idx + 2] = base_rgb[idx + 2] * (1.0 - t) + ring_b * t;
                    }
                }
            }
        }
    }

    // AI Denoise: runs first on the decoded pixels (RAW and JPG take the same path)
    if params.ai_denoise > 0.5 {
        base_rgb = adaptive_denoise_rgb(&base_rgb, w, h, params.ai_denoise, params.nr_detail);
    }

    // Multi-scale Frequency Separation Blurs on base_rgb
    let fine_radius = ((w.min(h) as f32 * 0.0045).round() as usize).clamp(2, 10);
    let med_radius = ((w.min(h) as f32 * 0.014).round() as usize).clamp(5, 24);
    let bokeh_radius = if params.bg_blur_bokeh > 0.5 || params.stray_hair > 0.5 || params.backdrop_banding_fix > 0.5 {
        ((w.min(h) as f32 * 0.026 * (params.bg_blur_bokeh.max(35.0) / 100.0)).round() as usize).clamp(6, 44)
    } else {
        0
    };

    let blur_fine = separable_blur(&base_rgb, w, h, fine_radius);
    let blur_med_p1 = separable_blur(&base_rgb, w, h, med_radius);
    let blur_med = separable_blur(&blur_med_p1, w, h, med_radius / 2 + 1);
    let blur_bokeh = if bokeh_radius > 0 {
        let p1 = separable_blur(&base_rgb, w, h, bokeh_radius);
        separable_blur(&p1, w, h, bokeh_radius / 2 + 1)
    } else {
        Vec::new()
    };

    let face = &params.face;

    // Precompute Color & Retouching scalars (each slider strictly separated!)
    let exp_mult = 2.0f32.powf(params.exposure / 50.0);
    let contrast_k = (params.contrast / 100.0) * 0.72;
    let temp_shift = (params.temperature / 100.0) * 0.16;
    let tint_shift = (params.tint / 100.0) * 0.12;
    let sat_mult = 1.0 + (params.saturation / 100.0) * 0.90;
    let vib_strength = (params.vibrance / 100.0) * 0.85;
    let clarity_str = (params.clarity / 100.0) * 0.95;
    let texture_str = (params.texture / 100.0) * 1.05;
    let dehaze_str = (params.dehaze / 100.0) * 0.50;
    let high_adj = (params.highlights / 100.0) * 0.42;
    let shad_adj = (params.shadows / 100.0) * 0.46;
    let whites_adj = (params.whites / 100.0) * 0.28;
    let blacks_adj = (params.blacks / 100.0) * 0.28;

    // Separated Portrait Retouching Strengths
    let freckle_acne_str = (params.freckle_acne / 100.0).clamp(0.0, 1.0);
    let face_mole_str = (params.face_mole / 100.0).clamp(0.0, 1.0);
    let body_blemish_str = (params.body_blemish / 100.0).clamp(0.0, 1.0);
    let face_shine_str = (params.face_shine / 100.0).clamp(0.0, 1.0);
    let glasses_glare_str = (params.glasses_glare / 100.0).clamp(0.0, 1.0);
    let dark_circles_str = (params.dark_circles / 100.0).clamp(0.0, 1.0);
    let eye_bags_str = (params.eye_bags / 100.0).clamp(0.0, 1.0);
    let forehead_wrinkles_str = (params.forehead_wrinkles / 100.0).clamp(0.0, 1.0);
    let nasolabial_str = (params.nasolabial_folds / 100.0).clamp(0.0, 1.0);
    let neck_wrinkles_str = (params.neck_wrinkles / 100.0).clamp(0.0, 1.0);
    let double_chin_str = (params.double_chin / 100.0).clamp(0.0, 1.0);

    let dodge_burn_even_str = (params.dodge_burn_even / 100.0).clamp(0.0, 1.0);
    let unify_complexion_str = (params.unify_complexion / 100.0).clamp(0.0, 1.0);
    let db_sculpt_str = (params.dodge_burn_sculpt / 100.0).clamp(0.0, 1.0);
    let freq_sep_low_str = (params.freq_sep_low / 100.0).clamp(0.0, 1.0);
    let pore_scale = (params.freq_sep_high / 80.0).clamp(0.0, 1.35);
    let facial_smooth_str = (params.facial_skin_smooth / 100.0).clamp(0.0, 1.0);
    let body_smooth_str = (params.body_skin_smooth / 100.0).clamp(0.0, 1.0);
    let rad_str = (params.skin_radiance / 100.0).clamp(0.0, 1.0);
    let rosy_str = (params.skin_rosy / 100.0).clamp(-1.0, 1.0);
    let warmth_str = (params.skin_warmth / 100.0).clamp(-1.0, 1.0);

    let eye_bright_str = (params.eye_brighten / 100.0).clamp(0.0, 1.0);
    let catchlight_str = (params.catchlight_boost / 100.0).clamp(0.0, 1.0);
    let red_vein_str = (params.red_veins / 100.0).clamp(0.0, 1.0);
    let brow_thick_str = (params.eyebrow_thickness / 100.0).clamp(-1.0, 1.0);

    let teeth_whiten_str = (params.teeth_whiten / 100.0).clamp(0.0, 1.0);
    let teeth_desat_str = (params.teeth_desaturate / 100.0).clamp(0.0, 1.0);
    let teeth_bright_str = (params.teeth_brighten / 100.0).clamp(0.0, 1.0);

    let makeup_high_str = (params.makeup_highlight / 100.0).clamp(0.0, 1.0);
    let makeup_cont_str = (params.makeup_contour / 100.0).clamp(0.0, 1.0);
    let makeup_blush_str = (params.makeup_blush / 100.0).clamp(0.0, 1.0);
    let makeup_eyeshadow_str = (params.makeup_eyeshadow / 100.0).clamp(0.0, 1.0);
    let lip_str = (params.makeup_lipstick / 100.0).clamp(0.0, 1.0);

    // Precompute source image RGB mean & std for Color Match (Histogram / Moment Transfer)
    let color_match_str = (params.color_match_strength / 100.0).clamp(0.0, 1.0);
    let src_stats: [f32; 6] = if color_match_str > 0.005 && params.color_match_stats.len() == 6 {
        let step = (w * h / 4096).max(1);
        let mut sr = 0.0f64;
        let mut sg = 0.0f64;
        let mut sb = 0.0f64;
        let mut cnt = 0.0f64;
        for i in (0..(w * h)).step_by(step) {
            sr += base_rgb[i * 3] as f64;
            sg += base_rgb[i * 3 + 1] as f64;
            sb += base_rgb[i * 3 + 2] as f64;
            cnt += 1.0;
        }
        let mr = (sr / cnt.max(1.0)) as f32;
        let mg = (sg / cnt.max(1.0)) as f32;
        let mb = (sb / cnt.max(1.0)) as f32;
        let mut vr = 0.0f64;
        let mut vg = 0.0f64;
        let mut vb = 0.0f64;
        for i in (0..(w * h)).step_by(step) {
            vr += ((base_rgb[i * 3] - mr) as f64).powi(2);
            vg += ((base_rgb[i * 3 + 1] - mg) as f64).powi(2);
            vb += ((base_rgb[i * 3 + 2] - mb) as f64).powi(2);
        }
        [
            mr,
            ((vr / cnt.max(1.0)).sqrt() as f32).max(0.02),
            mg,
            ((vg / cnt.max(1.0)).sqrt() as f32).max(0.02),
            mb,
            ((vb / cnt.max(1.0)).sqrt() as f32).max(0.02),
        ]
    } else {
        [0.5, 0.2, 0.5, 0.2, 0.5, 0.2]
    };

    // Stage 1: Apply all Semantic Mask-Guided Retouching + Color Grading in unwarped image space
    let mut retouched_rgb = vec![0.0f32; w * h * 3];
    retouched_rgb
        .par_chunks_mut(w * 3)
        .enumerate()
        .for_each(|(y, row_out)| {
            let ny = y as f32 / h as f32;
            for x in 0..w {
                let nx = x as f32 / w as f32;
                let idx = (y * w + x) * 3;

                let mut r = base_rgb[idx];
                let mut g = base_rgb[idx + 1];
                let mut b = base_rgb[idx + 2];

                let rf = blur_fine[idx];
                let gf = blur_fine[idx + 1];
                let bf = blur_fine[idx + 2];

                let rm = blur_med[idx];
                let gm = blur_med[idx + 1];
                let bm = blur_med[idx + 2];

                let high_r = r - rf;
                let high_g = g - gf;
                let high_b = b - bf;

                let sem = sample_semantic_grid(&semantic_grid, nx, ny);

                // Combine neural polygon skin mask with subtle color gate so non-skin objects are never smoothed
                let color_sw = skin_weight(r, g, b);
                let sw = (sem.skin * (0.65 + 0.35 * color_sw)).clamp(0.0, 1.0);
                let is_body_zone = ((ny - face.chin_y) / (face.face_ry * 0.25).max(0.04)).clamp(0.0, 1.0);
                let face_skin_w = sw * (1.0 - is_body_zone);
                let body_skin_w = sw * is_body_zone;

                // =========================================================
                // 1. BLEMISHES, SHINE, WRINKLES & DARK CIRCLES / EYE BAGS
                // =========================================================
                if sw > 0.01 {
                    let lum_curr = 0.299 * r + 0.587 * g + 0.114 * b;
                    let lum_fine = 0.299 * rf + 0.587 * gf + 0.114 * bf;
                    let lum_med = 0.299 * rm + 0.587 * gm + 0.114 * bm;

                    // Local edge guard so structural boundaries (jaw shadow, nose edge) are not flattened
                    let med_diff = ((rf - rm).abs() + (gf - gm).abs() + (bf - bm).abs()) / 3.0;
                    let local_flatness = (1.0 - (med_diff / 0.16)).clamp(0.15, 1.0);

                    // 1a. Freckles & Acne (Face only): targets localized reddish/warm spots & small pigment dips
                    if freckle_acne_str > 0.005 && face_skin_w > 0.01 {
                        let spot_dip = (lum_med - lum_fine).clamp(0.0, 0.12);
                        let red_spot = ((rf - gf) - (rm - gm)).clamp(0.0, 0.12);
                        let heal_a = ((spot_dip * 7.5 + red_spot * 6.5) * freckle_acne_str * face_skin_w * local_flatness).clamp(0.0, 0.92);
                        r = r * (1.0 - heal_a) + (rm + high_r * 0.45) * heal_a;
                        g = g * (1.0 - heal_a) + (gm + high_g * 0.45) * heal_a;
                        b = b * (1.0 - heal_a) + (bm + high_b * 0.45) * heal_a;
                    }

                    // 1b. Face Moles: targets compact dark spots (lum_curr significantly darker than lum_med)
                    if face_mole_str > 0.005 && face_skin_w > 0.01 {
                        let mole_dip = (lum_med - lum_curr).clamp(0.0, 0.22);
                        if mole_dip > 0.025 {
                            let mole_a = ((mole_dip - 0.02) * 8.5 * face_mole_str * face_skin_w * local_flatness).clamp(0.0, 0.94);
                            r = r * (1.0 - mole_a) + (rm + high_r * 0.30) * mole_a;
                            g = g * (1.0 - mole_a) + (gm + high_g * 0.30) * mole_a;
                            b = b * (1.0 - mole_a) + (bm + high_b * 0.30) * mole_a;
                        }
                    }

                    // 1c. Body Blemishes (Neck & Body skin only!)
                    if body_blemish_str > 0.005 && body_skin_w > 0.01 {
                        let b_dip = (lum_med - lum_curr).clamp(0.0, 0.15);
                        let b_red = ((r - g) - (rm - gm)).clamp(0.0, 0.12);
                        let body_a = ((b_dip * 8.0 + b_red * 6.5) * body_blemish_str * body_skin_w).clamp(0.0, 0.92);
                        r = r * (1.0 - body_a) + (rm + high_r * 0.40) * body_a;
                        g = g * (1.0 - body_a) + (gm + high_g * 0.40) * body_a;
                        b = b * (1.0 - body_a) + (bm + high_b * 0.40) * body_a;
                    }

                    // 1d. Forehead Wrinkles (Strictly inside Forehead Polygon sem.forehead!)
                    if forehead_wrinkles_str > 0.005 && sem.forehead > 0.01 {
                        let crease_dip = (lum_med - lum_curr).clamp(0.0, 0.14);
                        let fw_a = (crease_dip * 9.5 * forehead_wrinkles_str * sem.forehead * face_skin_w).clamp(0.0, 0.92);
                        r = r * (1.0 - fw_a) + (rm + high_r * 0.45) * fw_a;
                        g = g * (1.0 - fw_a) + (gm + high_g * 0.45) * fw_a;
                        b = b * (1.0 - fw_a) + (bm + high_b * 0.45) * fw_a;
                    }

                    // 1e. Nasolabial Folds / Smile Lines (Strictly inside Nasolabial Polygon sem.nasolabial!)
                    if nasolabial_str > 0.005 && sem.nasolabial > 0.01 {
                        let fold_dip = (lum_med - lum_curr).clamp(0.0, 0.16);
                        let nf_a = (fold_dip * 8.5 * nasolabial_str * sem.nasolabial * face_skin_w).clamp(0.0, 0.90);
                        r = r * (1.0 - nf_a) + (rm + high_r * 0.55) * nf_a;
                        g = g * (1.0 - nf_a) + (gm + high_g * 0.55) * nf_a;
                        b = b * (1.0 - nf_a) + (bm + high_b * 0.55) * nf_a;
                    }

                    // 1f. Neck Wrinkles (Strictly on Neck skin below chin!)
                    if neck_wrinkles_str > 0.005 && body_skin_w > 0.01 {
                        let neck_dip = (lum_med - lum_curr).clamp(0.0, 0.16);
                        let nw_a = (neck_dip * 9.0 * neck_wrinkles_str * body_skin_w).clamp(0.0, 0.90);
                        r = r * (1.0 - nw_a) + (rm + high_r * 0.45) * nw_a;
                        g = g * (1.0 - nw_a) + (gm + high_g * 0.45) * nw_a;
                        b = b * (1.0 - nw_a) + (bm + high_b * 0.45) * nw_a;
                    }

                    // 1g. Face Shine (Matte specular highlight compression toward natural local skin color!)
                    if face_shine_str > 0.005 && face_skin_w > 0.01 {
                        let lum_now = 0.299 * r + 0.587 * g + 0.114 * b;
                        if lum_now > lum_med + 0.018 && lum_now > 0.55 {
                            let excess = ((lum_now - lum_med) * 3.2 * ((lum_now - 0.52) / 0.48).clamp(0.0, 1.0)).clamp(0.0, 0.88);
                            let shine_a = excess * face_shine_str * face_skin_w;
                            r = r * (1.0 - shine_a) + (rm + high_r * 0.60) * shine_a;
                            g = g * (1.0 - shine_a) + (gm + high_g * 0.60) * shine_a;
                            b = b * (1.0 - shine_a) + (bm + high_b * 0.60) * shine_a;
                        }
                    }

                    // =========================================================
                    // 2. SKIN RETOUCHING (FREQUENCY SEPARATION & DODGE/BURN)
                    // =========================================================
                    // 2a. Frequency Separation Low + Pore Texture Control (Face & Body)
                    if freq_sep_low_str > 0.005 || (params.freq_sep_high - 80.0).abs() > 1.0 {
                        let fs_a = (freq_sep_low_str * sw * 0.90).clamp(0.0, 0.92);
                        let low_r = rf * (1.0 - fs_a) + rm * fs_a;
                        let low_g = gf * (1.0 - fs_a) + gm * fs_a;
                        let low_b = bf * (1.0 - fs_a) + bm * fs_a;
                        let active_pore = if freq_sep_low_str > 0.005 {
                            pore_scale
                        } else {
                            1.0 + (pore_scale - 1.0) * sw
                        };
                        r = low_r + high_r * active_pore;
                        g = low_g + high_g * active_pore;
                        b = low_b + high_b * active_pore;
                    }

                    // 2b. Facial Skin Smooth (Dedicated Face-only bilateral-like smoothing)
                    if facial_smooth_str > 0.005 && face_skin_w > 0.01 {
                        let sm_a = (facial_smooth_str * face_skin_w * local_flatness * 0.88).clamp(0.0, 0.92);
                        r = r * (1.0 - sm_a) + (rm + high_r * 0.35 * pore_scale) * sm_a;
                        g = g * (1.0 - sm_a) + (gm + high_g * 0.35 * pore_scale) * sm_a;
                        b = b * (1.0 - sm_a) + (bm + high_b * 0.35 * pore_scale) * sm_a;
                    }

                    // 2c. Body Skin Smooth (Dedicated Neck & Body-only smoothing — never touches face!)
                    if body_smooth_str > 0.005 && body_skin_w > 0.01 {
                        let bsm_a = (body_smooth_str * body_skin_w * local_flatness * 0.90).clamp(0.0, 0.94);
                        r = r * (1.0 - bsm_a) + (rm + high_r * 0.30) * bsm_a;
                        g = g * (1.0 - bsm_a) + (gm + high_g * 0.30) * bsm_a;
                        b = b * (1.0 - bsm_a) + (bm + high_b * 0.30) * bsm_a;
                    }

                    // 2d. Dodge & Burn Even (Evens out micro-luminance blotches while keeping chrominance & pores)
                    if dodge_burn_even_str > 0.005 {
                        let lum_now = (0.299 * r + 0.587 * g + 0.114 * b).max(1e-4);
                        let lum_f = 0.299 * rf + 0.587 * gf + 0.114 * bf;
                        let lum_m = 0.299 * rm + 0.587 * gm + 0.114 * bm;
                        let target_lum = lum_f * (1.0 - dodge_burn_even_str * sw * 0.82)
                            + lum_m * (dodge_burn_even_str * sw * 0.82)
                            + (lum_now - lum_f);
                        let scale = (target_lum / lum_now).clamp(0.75, 1.30);
                        r *= scale;
                        g *= scale;
                        b *= scale;
                    }

                    // 2e. Unify Complexion (Evens out redness/color blotches while keeping luminance & pores!)
                    if unify_complexion_str > 0.005 {
                        let lum_now = (0.299 * r + 0.587 * g + 0.114 * b).max(1e-4);
                        let lum_m = (0.299 * rm + 0.587 * gm + 0.114 * bm).max(1e-4);
                        let uc_a = (unify_complexion_str * sw * 0.85).clamp(0.0, 0.90);
                        let cr = (rm / lum_m) * lum_now;
                        let cg = (gm / lum_m) * lum_now;
                        let cb = (bm / lum_m) * lum_now;
                        r = r * (1.0 - uc_a) + cr * uc_a;
                        g = g * (1.0 - uc_a) + cg * uc_a;
                        b = b * (1.0 - uc_a) + cb * uc_a;
                    }

                    // 2f. 3D Dodge & Burn Sculpt + Makeup Highlight & Contour
                    if (db_sculpt_str > 0.005 || makeup_high_str > 0.005 || makeup_cont_str > 0.005) && face_skin_w > 0.01 {
                        let fdx = (nx - face.center_x) / face.face_rx.max(0.05);
                        let fdy = (ny - face.center_y) / face.face_ry.max(0.05);
                        let face_d2 = fdx * fdx + fdy * fdy;

                        // T-zone highlight: nose bridge + forehead center + upper cheekbones + chin center
                        let bridge_d2 = ((nx - face.nose_x) / (face.face_rx * 0.14)).powi(2)
                            + ((ny - (face.nose_y - face.face_ry * 0.16)) / (face.face_ry * 0.24)).powi(2);
                        let tzone = (1.0 - bridge_d2).clamp(0.0, 1.0)
                            .max(sem.cheek * 0.75)
                            .max(sem.forehead * (1.0 - fdx.abs() * 1.5).clamp(0.0, 1.0) * 0.7);
                        // Outer jawline / temple / hollow contour
                        let perimeter = ((face_d2 - 0.42).max(0.0) * 1.25).clamp(0.0, 1.0);

                        let hi_gain = (db_sculpt_str * 0.11 + makeup_high_str * 0.14) * tzone * face_skin_w;
                        let co_gain = (db_sculpt_str * 0.10 + makeup_cont_str * 0.13) * perimeter * face_skin_w;
                        let sculpt_factor = 1.0 + hi_gain - co_gain;
                        r *= sculpt_factor;
                        g *= 1.0 + (hi_gain - co_gain) * 0.94;
                        b *= 1.0 + (hi_gain - co_gain) * 0.88;
                    }

                    // 2g. Double Chin Shadow Contouring (under chin)
                    if double_chin_str > 0.005 && ny > face.chin_y - 0.01 && ny < face.chin_y + face.face_ry * 0.35 {
                        let dc_x = ((nx - face.chin_x) / (face.face_rx * 0.55)).powi(2);
                        let dc_y = ((ny - (face.chin_y + face.face_ry * 0.10)) / (face.face_ry * 0.18)).powi(2);
                        let dc_w = (1.0 - (dc_x + dc_y)).clamp(0.0, 1.0) * double_chin_str * sw;
                        r *= 1.0 - dc_w * 0.12;
                        g *= 1.0 - dc_w * 0.12;
                        b *= 1.0 - dc_w * 0.12;
                    }

                    // 2h. Makeup Blush (Strictly on Cheek Polygons sem.cheek!)
                    if makeup_blush_str > 0.005 && sem.cheek > 0.01 {
                        let blush_a = sem.cheek * makeup_blush_str * face_skin_w;
                        r += blush_a * 0.085;
                        g -= blush_a * 0.032;
                        b -= blush_a * 0.012;
                    }

                    // 2i. Skin Radiance, Rosy Undertone & Warmth
                    if rad_str > 0.005 {
                        let lum_now = 0.299 * r + 0.587 * g + 0.114 * b;
                        let glow = rad_str * sw * 0.14 * lum_now * (1.0 - lum_now * 0.35);
                        r += glow * 1.06;
                        g += glow * 0.98;
                        b += glow * 0.90;
                    }
                    if rosy_str.abs() > 0.005 || warmth_str.abs() > 0.005 {
                        r += (rosy_str * 0.065 + warmth_str * 0.075) * sw;
                        g += (-rosy_str * 0.025 + warmth_str * 0.028) * sw;
                        b += (rosy_str * 0.022 - warmth_str * 0.060) * sw;
                    }
                }

                // =========================================================
                // 3. UNDER-EYE DARK CIRCLES & EYE BAGS (Strictly sem.under_eye!)
                // =========================================================
                if sem.under_eye > 0.01 && (dark_circles_str > 0.005 || eye_bags_str > 0.005) {
                    let lum_curr = 0.299 * r + 0.587 * g + 0.114 * b;
                    let lum_fine = 0.299 * rf + 0.587 * gf + 0.114 * bf;
                    let lum_med = (0.299 * rm + 0.587 * gm + 0.114 * bm).max(1e-4);

                    // Dark Circles: Lifts shadow darkness & warms bluish/purple under-eye cast while keeping 100% of fine pores!
                    if dark_circles_str > 0.005 {
                        let dc_a = (sem.under_eye * dark_circles_str * 0.85).clamp(0.0, 0.90);
                        let shadow_lift = (1.0 - lum_curr).clamp(0.15, 0.85) * 0.14 * dc_a;
                        // Harmonize color toward warmer surrounding cheek med-blur while keeping pore texture
                        let color_blend = dc_a * 0.55;
                        let base_r = rf * (1.0 - color_blend) + (rm * (lum_fine / lum_med)) * color_blend;
                        let base_g = gf * (1.0 - color_blend) + (gm * (lum_fine / lum_med)) * color_blend;
                        let base_b = bf * (1.0 - color_blend) + (bm * (lum_fine / lum_med)) * color_blend;
                        r = base_r + high_r + shadow_lift * 1.08;
                        g = base_g + high_g + shadow_lift * 0.96;
                        b = base_b + high_b + shadow_lift * 0.78;
                    }

                    // Eye Bags: Smooths out the puffy bag shadow trough line (lum_med - lum_fine) while preserving fine skin pores!
                    if eye_bags_str > 0.005 {
                        let bag_trough = (lum_med - lum_fine).clamp(-0.02, 0.16);
                        if bag_trough > 0.0 {
                            let eb_a = (sem.under_eye * eye_bags_str * 0.88).clamp(0.0, 0.92);
                            r = (rf * (1.0 - eb_a) + rm * eb_a) + high_r * 0.65;
                            g = (gf * (1.0 - eb_a) + gm * eb_a) + high_g * 0.65;
                            b = (bf * (1.0 - eb_a) + bm * eb_a) + high_b * 0.65;
                        }
                    }
                }

                // =========================================================
                // 4. GLASSES GLARE (Anti-reflective green/blue lens glare)
                // =========================================================
                if glasses_glare_str > 0.005 {
                    let eye_band_x = ((nx - face.center_x) / (face.face_rx * 0.82)).powi(2);
                    let eye_band_y = ((ny - ((face.left_eye_y + face.right_eye_y) * 0.5)) / (face.face_ry * 0.26)).powi(2);
                    let in_glasses = (1.0 - (eye_band_x + eye_band_y)).clamp(0.0, 1.0) * (1.0 - sem.eye * 0.85);
                    if in_glasses > 0.05 {
                        let lum_now = 0.299 * r + 0.587 * g + 0.114 * b;
                        let ar_tint = (g - r).max(b - r).max(0.0);
                        if lum_now > 0.48 && (ar_tint > 0.015 || lum_now > 0.72) {
                            let gl_a = (in_glasses * glasses_glare_str * 0.78).clamp(0.0, 0.88);
                            r = r * (1.0 - gl_a) + rm * gl_a;
                            g = g * (1.0 - gl_a) + (gm - ar_tint * 0.45) * gl_a;
                            b = b * (1.0 - gl_a) + (bm - ar_tint * 0.45) * gl_a;
                        }
                    }
                }

                // =========================================================
                // 5. EYES, EYEBROWS & EYESHADOW (Strictly isolated!)
                // =========================================================
                // 5a. Upper Eyelid Eyeshadow (sem.eyeshadow, strictly outside eyeball!)
                if makeup_eyeshadow_str > 0.005 && sem.eyeshadow > 0.01 {
                    let es_a = sem.eyeshadow * makeup_eyeshadow_str * 0.42;
                    // Soft warm rose-bronze eyeshadow tint that preserves skin texture
                    r *= 1.0 - es_a * 0.22;
                    g *= 1.0 - es_a * 0.38;
                    b *= 1.0 - es_a * 0.34;
                }

                // 5b. Eyes: Iris Brighten, Catchlight Boost, and Sclera Red Veins (Strictly sem.eye!)
                if sem.eye > 0.01 && (eye_bright_str > 0.005 || catchlight_str > 0.005 || red_vein_str > 0.005) {
                    let lum_now = 0.299 * r + 0.587 * g + 0.114 * b;
                    // Protect dark pupil (lum < 0.15) so pupils stay jet black
                    let pupil_guard = ((lum_now - 0.12) / 0.16).clamp(0.0, 1.0);

                    if eye_bright_str > 0.005 {
                        let eb = sem.eye * eye_bright_str * pupil_guard;
                        let detail_gain = eb * 1.45;
                        let mid_lift = eb * 0.11 * (1.0 - lum_now * 0.5);
                        r += high_r * detail_gain + mid_lift;
                        g += high_g * detail_gain + mid_lift;
                        b += high_b * detail_gain + mid_lift * 1.05;
                    }

                    if catchlight_str > 0.005 && lum_now > 0.52 {
                        let spec = ((lum_now - 0.52) / 0.48).clamp(0.0, 1.0).powf(1.3);
                        let cb_boost = sem.eye * catchlight_str * spec * 0.38;
                        r += cb_boost + high_r * sem.eye * catchlight_str * 0.8;
                        g += cb_boost + high_g * sem.eye * catchlight_str * 0.8;
                        b += cb_boost * 1.05 + high_b * sem.eye * catchlight_str * 0.8;
                    }

                    if red_vein_str > 0.005 && lum_now > 0.30 && r > g {
                        // Sclera vein neutralization: pulls excess red/pink toward clean neutral sclera luminance
                        let redness = (r - g).clamp(0.0, 0.25);
                        let sclera_w = ((lum_now - 0.28) / 0.45).clamp(0.0, 1.0);
                        let vein_a = (sem.eye * red_vein_str * sclera_w * 0.90).clamp(0.0, 0.95);
                        r -= redness * vein_a * 0.88;
                        g += redness * vein_a * 0.15;
                        b += redness * vein_a * 0.22;
                    }
                }

                // 5c. Eyebrows Thickness (Strictly sem.eyebrow!)
                if brow_thick_str.abs() > 0.005 && sem.eyebrow > 0.01 {
                    let lum_now = 0.299 * r + 0.587 * g + 0.114 * b;
                    let hair_w = (1.0 - lum_now).clamp(0.15, 0.95);
                    let bt = sem.eyebrow * brow_thick_str * hair_w * 0.34;
                    r = (r * (1.0 - bt)) + high_r * sem.eyebrow * brow_thick_str.max(0.0) * 0.45;
                    g = (g * (1.0 - bt)) + high_g * sem.eyebrow * brow_thick_str.max(0.0) * 0.45;
                    b = (b * (1.0 - bt)) + high_b * sem.eyebrow * brow_thick_str.max(0.0) * 0.45;
                }

                // =========================================================
                // 6. TEETH & LIPSTICK (Strictly isolated inner mouth vs outer lips!)
                // =========================================================
                // 6a. Teeth Whitening, De-Yellowing & Brightening (Strictly sem.teeth_inner!)
                if sem.teeth_inner > 0.01 && (teeth_whiten_str > 0.005 || teeth_desat_str > 0.005 || teeth_bright_str > 0.005) {
                    let (h_deg, s_val, v_val) = rgb_to_hsv(r, g, b);
                    // Exclude dark mouth cavity shadows and red/pink gums/tongue
                    let is_tooth = if v_val > 0.24 && !((h_deg < 18.0 || h_deg > 335.0) && s_val > 0.34) {
                        ((v_val - 0.22) / 0.25).clamp(0.0, 1.0) * (1.0 - (s_val - 0.35).max(0.0) * 2.5).clamp(0.0, 1.0)
                    } else {
                        0.0
                    };

                    if is_tooth > 0.02 {
                        let tw = sem.teeth_inner * is_tooth;
                        // De-yellowing (Teeth Desaturate + part of Teeth Whiten)
                        let desat_amt = ((teeth_desat_str * 0.90 + teeth_whiten_str * 0.75) * tw).clamp(0.0, 0.92);
                        // Brightening (Teeth Brighten + part of Teeth Whiten)
                        let bright_amt = (teeth_bright_str * 0.20 + teeth_whiten_str * 0.16) * tw;

                        let new_s = s_val * (1.0 - desat_amt);
                        let new_v = (v_val + bright_amt * (1.0 - v_val * 0.35)).clamp(0.0, 0.99);
                        let (tr, tg, tb) = hsv_to_rgb(h_deg, new_s, new_v);
                        // Add a subtle cool pearl-white tint when whitening
                        r = tr;
                        g = tg;
                        b = (tb + teeth_whiten_str * tw * 0.025).min(1.0);
                    }
                }

                // 6b. Lipstick (Strictly sem.lips_outer — preserves 100% of lip texture and gloss highlights!)
                if sem.lips_outer > 0.01 && lip_str > 0.005 {
                    let lum_now = 0.299 * r + 0.587 * g + 0.114 * b;
                    // Preserve natural white lip gloss / specular highlight
                    let gloss_protect = ((lum_now - 0.68) / 0.28).clamp(0.0, 0.85);
                    let la = sem.lips_outer * lip_str * (1.0 - gloss_protect) * 0.62;

                    let (tr, tg, tb) = match params.makeup_lip_tint.as_str() {
                        "coral" => (0.94, 0.34, 0.26),
                        "berry" => (0.66, 0.14, 0.36),
                        "nude" => (0.78, 0.48, 0.42),
                        "ruby" => (0.86, 0.10, 0.16),
                        _ => (0.86, 0.24, 0.35), // rose
                    };
                    // Luminance-modulated color blend + 100% fine lip texture preservation
                    let shade_scale = (lum_now * 1.35).clamp(0.25, 1.15);
                    let target_r = tr * shade_scale + high_r * 0.85;
                    let target_g = tg * shade_scale + high_g * 0.85;
                    let target_b = tb * shade_scale + high_b * 0.85;
                    r = r * (1.0 - la) + target_r * la;
                    g = g * (1.0 - la) + target_g * la;
                    b = b * (1.0 - la) + target_b * la;
                }

                // =========================================================
                // 7. HAIR, CLOTHING & STUDIO BACKDROP (Neural 6-Class Masks!)
                // =========================================================
                // 7a. Hair Shine & Hair Dye (Strictly sem.hair!)
                if sem.hair > 0.02 {
                    if params.hair_shine > 0.5 {
                        let lum = 0.299 * r + 0.587 * g + 0.114 * b;
                        let sheen = ((lum - 0.12).max(0.0) * (0.82 - lum).max(0.0) * 5.8).clamp(0.0, 1.0);
                        let add = sem.hair * sheen * (params.hair_shine / 100.0) * 0.30;
                        r += add + high_r * sem.hair * 0.25;
                        g += add + high_g * sem.hair * 0.25;
                        b += add + high_b * sem.hair * 0.25;
                    }
                    if params.hair_dye_strength > 0.5 && params.hair_dye_color != "none" {
                        let dye_a = (sem.hair * (params.hair_dye_strength / 100.0) * 0.72).clamp(0.0, 0.90);
                        let lum = 0.299 * r + 0.587 * g + 0.114 * b;
                        let (dr, dg, db) = match params.hair_dye_color.as_str() {
                            "espresso" => (lum * 0.62, lum * 0.50, lum * 0.42),
                            "golden_blonde" => ((lum * 1.48 + 0.06).min(0.96), (lum * 1.30 + 0.04).min(0.88), lum * 0.82),
                            "auburn" => ((lum * 1.42 + 0.06).min(0.94), lum * 0.76, lum * 0.52),
                            "silver_ash" => (lum * 1.16 + 0.05, lum * 1.17 + 0.05, lum * 1.24 + 0.07),
                            _ => (r, g, b),
                        };
                        r = r * (1.0 - dye_a) + (dr + high_r * 0.6) * dye_a;
                        g = g * (1.0 - dye_a) + (dg + high_g * 0.6) * dye_a;
                        b = b * (1.0 - dye_a) + (db + high_b * 0.6) * dye_a;
                    }
                }

                // 7b. Clothing De-Wrinkle & Lint Clean (Strictly sem.clothing!)
                if sem.clothing > 0.02 {
                    if params.clothing_dewrinkle > 0.5 {
                        let diff = ((rf - rm).abs() + (gf - gm).abs() + (bf - bm).abs()) / 3.0;
                        let seam_guard = (1.0 - (diff / 0.16)).clamp(0.0, 1.0);
                        let cw_a = (sem.clothing * seam_guard * (params.clothing_dewrinkle / 100.0) * 0.88).clamp(0.0, 0.92);
                        r = r * (1.0 - cw_a) + (rm + high_r * 0.40) * cw_a;
                        g = g * (1.0 - cw_a) + (gm + high_g * 0.40) * cw_a;
                        b = b * (1.0 - cw_a) + (bm + high_b * 0.40) * cw_a;
                    }
                    if params.clothing_lint_clean > 0.5 {
                        let lum_curr = 0.299 * r + 0.587 * g + 0.114 * b;
                        let lum_fine = 0.299 * rf + 0.587 * gf + 0.114 * bf;
                        let speck = (lum_curr - lum_fine).max(0.0);
                        if speck > 0.015 {
                            let lint_a = (sem.clothing * (params.clothing_lint_clean / 100.0) * (speck * 18.0)).clamp(0.0, 0.95);
                            r = r * (1.0 - lint_a) + rf * lint_a;
                            g = g * (1.0 - lint_a) + gf * lint_a;
                            b = b * (1.0 - lint_a) + bf * lint_a;
                        }
                    }
                }

                // 7c. Stray Hair & Background Bokeh / Clean / Banding / Sky / Replace (Strictly sem.background!)
                let bg_mask = sem.background;
                let bg_blur_str = params.bg_blur_bokeh / 100.0;
                let stray_str = params.stray_hair / 100.0;

                if !blur_bokeh.is_empty() && (bg_blur_str > 0.005 || stray_str > 0.005) {
                    let rb = blur_bokeh[idx];
                    let gb = blur_bokeh[idx + 1];
                    let bb = blur_bokeh[idx + 2];

                    if stray_str > 0.005 && bg_mask > 0.15 && ny < face.mouth_y + 0.08 {
                        let sa = (stray_str * bg_mask * 0.88).clamp(0.0, 0.94);
                        r = r * (1.0 - sa) + rb * sa;
                        g = g * (1.0 - sa) + gb * sa;
                        b = b * (1.0 - sa) + bb * sa;
                    }
                    if bg_blur_str > 0.005 && bg_mask > 0.01 {
                        let ba = (bg_mask * bg_blur_str).clamp(0.0, 0.98);
                        r = r * (1.0 - ba) + rb * ba;
                        g = g * (1.0 - ba) + gb * ba;
                        b = b * (1.0 - ba) + bb * ba;
                    }
                }

                if params.backdrop_clean > 0.5 && bg_mask > 0.02 {
                    let ca = (bg_mask * (params.backdrop_clean / 100.0) * 0.85).clamp(0.0, 0.92);
                    r = r * (1.0 - ca) + rm * ca;
                    g = g * (1.0 - ca) + gm * ca;
                    b = b * (1.0 - ca) + bm * ca;
                }

                if params.backdrop_banding_fix > 0.5 && bg_mask > 0.02 && !blur_bokeh.is_empty() {
                    let bf_a = (bg_mask * (params.backdrop_banding_fix / 100.0) * 0.82).clamp(0.0, 0.92);
                    let dither = ((((x * 1973 + y * 9277) ^ 0x68bc21eb) & 0xff) as f32 / 255.0 - 0.5) * 0.006;
                    r = r * (1.0 - bf_a) + (blur_bokeh[idx] + dither) * bf_a;
                    g = g * (1.0 - bf_a) + (blur_bokeh[idx + 1] + dither) * bf_a;
                    b = b * (1.0 - bf_a) + (blur_bokeh[idx + 2] + dither) * bf_a;
                }

                if params.sky_enhance > 0.5 && bg_mask > 0.05 && ny < 0.60 {
                    let sky_a = bg_mask * (params.sky_enhance / 100.0) * (1.0 - ny * 1.5).clamp(0.0, 1.0) * 0.42;
                    r *= 1.0 - sky_a * 0.35;
                    g *= 1.0 - sky_a * 0.10;
                    b = (b + sky_a * 0.28).min(1.0);
                }

                if params.bg_replace_mode != "original" {
                    let b_mix = bg_mask;
                    let fg_mix = (1.0 - bg_mask).clamp(0.0, 1.0);
                    match params.bg_replace_mode.as_str() {
                        "warm_beige" if b_mix > 0.02 => {
                            r = r * (1.0 - b_mix * 0.85) + 0.92 * (b_mix * 0.85);
                            g = g * (1.0 - b_mix * 0.85) + 0.86 * (b_mix * 0.85);
                            b = b * (1.0 - b_mix * 0.85) + 0.78 * (b_mix * 0.85);
                        }
                        "studio_grey" if b_mix > 0.02 => {
                            r = r * (1.0 - b_mix * 0.88) + 0.48 * (b_mix * 0.88);
                            g = g * (1.0 - b_mix * 0.88) + 0.50 * (b_mix * 0.88);
                            b = b * (1.0 - b_mix * 0.88) + 0.53 * (b_mix * 0.88);
                        }
                        "passport_blue" if b_mix > 0.02 => {
                            r = r * (1.0 - b_mix * 0.90) + 0.22 * (b_mix * 0.90);
                            g = g * (1.0 - b_mix * 0.90) + 0.46 * (b_mix * 0.90);
                            b = b * (1.0 - b_mix * 0.90) + 0.78 * (b_mix * 0.90);
                        }
                        "pure_white" if b_mix > 0.02 => {
                            r = r * (1.0 - b_mix * 0.96) + 0.98 * (b_mix * 0.96);
                            g = g * (1.0 - b_mix * 0.96) + 0.98 * (b_mix * 0.96);
                            b = b * (1.0 - b_mix * 0.96) + 0.98 * (b_mix * 0.96);
                        }
                        "moody_black" if b_mix > 0.02 => {
                            r *= 1.0 - b_mix * 0.92;
                            g *= 1.0 - b_mix * 0.92;
                            b *= 1.0 - b_mix * 0.90;
                        }
                        "sky_golden_sunset" => {
                            let h_grad = ny.clamp(0.0, 1.0);
                            let cloud = ((nx * 9.0 + ny * 4.0).sin() * (nx * 15.0 - ny * 7.0).cos()).max(0.0) * 0.14;
                            let sr = (0.98 - h_grad * 0.12 + cloud).clamp(0.0, 1.0);
                            let sg = (0.52 + h_grad * 0.22 + cloud * 0.8).clamp(0.0, 1.0);
                            let sb = (0.34 + (1.0 - h_grad) * 0.28 + cloud * 0.5).clamp(0.0, 1.0);
                            let sky_w = (b_mix * 0.92).clamp(0.0, 1.0);
                            r = r * (1.0 - sky_w) + sr * sky_w + fg_mix * 0.045;
                            g = g * (1.0 - sky_w) + sg * sky_w + fg_mix * 0.015;
                            b = b * (1.0 - sky_w) + sb * sky_w - fg_mix * 0.025;
                        }
                        "sky_alpine_blue" => {
                            let h_grad = ny.clamp(0.0, 1.0);
                            let cloud = ((nx * 11.0 + 1.2).sin() * (ny * 8.0 + 0.5).cos()).max(0.0).powi(2) * 0.26 * (1.0 - h_grad * 0.6);
                            let sr = (0.18 + h_grad * 0.48 + cloud).clamp(0.0, 1.0);
                            let sg = (0.48 + h_grad * 0.36 + cloud).clamp(0.0, 1.0);
                            let sb = (0.88 + h_grad * 0.10 + cloud * 0.5).clamp(0.0, 1.0);
                            let sky_w = (b_mix * 0.92).clamp(0.0, 1.0);
                            r = r * (1.0 - sky_w) + sr * sky_w - fg_mix * 0.015;
                            g = g * (1.0 - sky_w) + sg * sky_w + fg_mix * 0.008;
                            b = b * (1.0 - sky_w) + sb * sky_w + fg_mix * 0.032;
                        }
                        "sky_dramatic_clouds" => {
                            let h_grad = ny.clamp(0.0, 1.0);
                            let storm = ((nx * 7.5).sin() * (ny * 6.0).cos() * 0.5 + 0.5) * 0.22;
                            let sr = (0.24 + h_grad * 0.26 + storm).clamp(0.0, 1.0);
                            let sg = (0.28 + h_grad * 0.26 + storm).clamp(0.0, 1.0);
                            let sb = (0.36 + h_grad * 0.24 + storm * 1.1).clamp(0.0, 1.0);
                            let sky_w = (b_mix * 0.92).clamp(0.0, 1.0);
                            r = r * (1.0 - sky_w) + sr * sky_w - fg_mix * 0.02;
                            g = g * (1.0 - sky_w) + sg * sky_w - fg_mix * 0.01;
                            b = b * (1.0 - sky_w) + sb * sky_w + fg_mix * 0.025;
                        }
                        "sky_night_stars" => {
                            let h_grad = ny.clamp(0.0, 1.0);
                            let star_hash = (((x * 747796405 + y * 2891336453) ^ 0x27d4eb2d) & 0xfff) as f32 / 4095.0;
                            let star = if star_hash > 0.996 && ny < 0.65 { 0.85 } else { 0.0 };
                            let sr = (0.05 + h_grad * 0.08 + star).clamp(0.0, 1.0);
                            let sg = (0.08 + h_grad * 0.12 + star).clamp(0.0, 1.0);
                            let sb = (0.18 + h_grad * 0.18 + star).clamp(0.0, 1.0);
                            let sky_w = (b_mix * 0.94).clamp(0.0, 1.0);
                            r = r * (1.0 - sky_w) + sr * sky_w - fg_mix * 0.035;
                            g = g * (1.0 - sky_w) + sg * sky_w - fg_mix * 0.020;
                            b = b * (1.0 - sky_w) + sb * sky_w + fg_mix * 0.040;
                        }
                        _ => {}
                    }
                }

                // =========================================================
                // 8. MODULE 1: COLOR ADJUSTMENT, TONE CURVE, HSL & GRADING
                // =========================================================
                r *= exp_mult;
                g *= exp_mult;
                b *= exp_mult;

                r += temp_shift + tint_shift * 0.4;
                g -= tint_shift * 0.7;
                b -= temp_shift - tint_shift * 0.3;

                let lum = (0.299 * r + 0.587 * g + 0.114 * b).clamp(0.0, 1.0);
                let shadow_w = (1.0 - lum).powi(2);
                let highlight_w = lum.powi(2);
                let tone_delta = shad_adj * shadow_w
                    + high_adj * highlight_w
                    + whites_adj * lum.powi(3)
                    + blacks_adj * (1.0 - lum).powi(3);
                r += tone_delta;
                g += tone_delta;
                b += tone_delta;

                if params.curve_shadows.abs() > 0.5
                    || params.curve_darks.abs() > 0.5
                    || params.curve_lights.abs() > 0.5
                    || params.curve_highlights.abs() > 0.5
                {
                    let cs = (params.curve_shadows / 100.0) * 0.28 * (1.0 - lum).powi(3);
                    let cd = (params.curve_darks / 100.0) * 0.28 * (3.0 * lum * (1.0 - lum).powi(2));
                    let cl = (params.curve_lights / 100.0) * 0.28 * (3.0 * lum.powi(2) * (1.0 - lum));
                    let ch = (params.curve_highlights / 100.0) * 0.28 * lum.powi(3);
                    let c_delta = cs + cd + cl + ch;
                    r += c_delta;
                    g += c_delta;
                    b += c_delta;
                }

                // Interactive 256-entry Tone Curve LUT (from Interactive Curve Canvas)
                if params.curve_lut.len() == 256 {
                    let lut_sample = |v: f32| -> f32 {
                        let scaled = (v.clamp(0.0, 1.0) * 255.0).clamp(0.0, 255.0);
                        let i0 = scaled.floor() as usize;
                        let i1 = (i0 + 1).min(255);
                        let frac = scaled - i0 as f32;
                        let v0 = params.curve_lut[i0] as f32 / 255.0;
                        let v1 = params.curve_lut[i1] as f32 / 255.0;
                        v0 * (1.0 - frac) + v1 * frac
                    };
                    r = lut_sample(r);
                    g = lut_sample(g);
                    b = lut_sample(b);
                }

                if dehaze_str.abs() > 0.005 {
                    r = (r - 0.14 * dehaze_str) / (1.0 - 0.14 * dehaze_str).max(0.2);
                    g = (g - 0.14 * dehaze_str) / (1.0 - 0.14 * dehaze_str).max(0.2);
                    b = (b - 0.14 * dehaze_str) / (1.0 - 0.14 * dehaze_str).max(0.2);
                }

                if texture_str.abs() > 0.01 || params.sharpen_amount > 0.5 || params.noise_reduction > 0.5 {
                    // Radius: blend the fine high-pass towards a wider one. Detail: boost fine contrast. Masking: edges only.
                    let rad_t = ((params.sharpen_radius - 0.5) / 2.5).clamp(0.0, 1.0);
                    let hr = r - (rf + (rm - rf) * rad_t);
                    let hg = g - (gf + (gm - gf) * rad_t);
                    let hb = b - (bf + (bm - bf) * rad_t);
                    let edge = (high_r.abs() + high_g.abs() + high_b.abs()) / 3.0;
                    let mask_k = (params.sharpen_masking / 100.0).clamp(0.0, 1.0);
                    let edge_mask = 1.0 - mask_k + mask_k * smoothstep(0.004, 0.05, edge);
                    let detail_gain = 0.5 + (params.sharpen_detail / 50.0);
                    let sharp_amt = (params.sharpen_amount / 100.0) * 1.25 * detail_gain * edge_mask;
                    let sharp = texture_str + sharp_amt - (params.noise_reduction / 100.0) * 0.90;
                    r += hr * sharp;
                    g += hg * sharp;
                    b += hb * sharp;
                }

                if clarity_str.abs() > 0.01 {
                    let mid_w = 4.0 * lum * (1.0 - lum);
                    r += (r - rm) * clarity_str * mid_w;
                    g += (g - gm) * clarity_str * mid_w;
                    b += (b - bm) * clarity_str * mid_w;
                }

                if contrast_k.abs() > 0.005 {
                    r = (r - 0.5) * (1.0 + contrast_k) + 0.5;
                    g = (g - 0.5) * (1.0 + contrast_k) + 0.5;
                    b = (b - 0.5) * (1.0 + contrast_k) + 0.5;
                }

                // 8-Channel HSL Color Mixer (Red, Orange, Yellow, Green, Aqua, Blue, Purple, Magenta)
                let has_hsl = params.hsl_red_hue.abs() > 0.5 || params.hsl_red_sat.abs() > 0.5 || params.hsl_red_lum.abs() > 0.5
                    || params.hsl_orange_hue.abs() > 0.5 || params.hsl_orange_sat.abs() > 0.5 || params.hsl_orange_lum.abs() > 0.5
                    || params.hsl_yellow_hue.abs() > 0.5 || params.hsl_yellow_sat.abs() > 0.5 || params.hsl_yellow_lum.abs() > 0.5
                    || params.hsl_green_hue.abs() > 0.5 || params.hsl_green_sat.abs() > 0.5 || params.hsl_green_lum.abs() > 0.5
                    || params.hsl_aqua_hue.abs() > 0.5 || params.hsl_aqua_sat.abs() > 0.5 || params.hsl_aqua_lum.abs() > 0.5
                    || params.hsl_blue_hue.abs() > 0.5 || params.hsl_blue_sat.abs() > 0.5 || params.hsl_blue_lum.abs() > 0.5
                    || params.hsl_purple_hue.abs() > 0.5 || params.hsl_purple_sat.abs() > 0.5 || params.hsl_purple_lum.abs() > 0.5
                    || params.hsl_magenta_hue.abs() > 0.5 || params.hsl_magenta_sat.abs() > 0.5 || params.hsl_magenta_lum.abs() > 0.5;

                if has_hsl {
                    let (mut hh, mut ss, mut vv) = rgb_to_hsv(r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0));
                    let band_weight = |center: f32, width: f32, h_val: f32| -> f32 {
                        let mut d = (h_val - center).abs();
                        if d > 180.0 {
                            d = 360.0 - d;
                        }
                        (1.0 - d / width).clamp(0.0, 1.0)
                    };
                    let wr = band_weight(0.0, 26.0, hh);
                    let wo = band_weight(28.0, 25.0, hh);
                    let wy = band_weight(58.0, 28.0, hh);
                    let wg = band_weight(120.0, 42.0, hh);
                    let wa = band_weight(180.0, 36.0, hh);
                    let wb = band_weight(225.0, 40.0, hh);
                    let wp = band_weight(275.0, 36.0, hh);
                    let wm = band_weight(322.0, 34.0, hh);

                    hh += (wr * params.hsl_red_hue
                        + wo * params.hsl_orange_hue
                        + wy * params.hsl_yellow_hue
                        + wg * params.hsl_green_hue
                        + wa * params.hsl_aqua_hue
                        + wb * params.hsl_blue_hue
                        + wp * params.hsl_purple_hue
                        + wm * params.hsl_magenta_hue)
                        * 0.35;
                    let sat_shift = wr * params.hsl_red_sat
                        + wo * params.hsl_orange_sat
                        + wy * params.hsl_yellow_sat
                        + wg * params.hsl_green_sat
                        + wa * params.hsl_aqua_sat
                        + wb * params.hsl_blue_sat
                        + wp * params.hsl_purple_sat
                        + wm * params.hsl_magenta_sat;
                    let lum_shift = wr * params.hsl_red_lum
                        + wo * params.hsl_orange_lum
                        + wy * params.hsl_yellow_lum
                        + wg * params.hsl_green_lum
                        + wa * params.hsl_aqua_lum
                        + wb * params.hsl_blue_lum
                        + wp * params.hsl_purple_lum
                        + wm * params.hsl_magenta_lum;
                    ss = (ss * (1.0 + sat_shift / 100.0)).clamp(0.0, 1.0);
                    vv = (vv + (lum_shift / 100.0) * 0.28).clamp(0.0, 1.0);
                    let (nr, ng, nb) = hsv_to_rgb(hh, ss, vv);
                    r = nr;
                    g = ng;
                    b = nb;
                }

                // Reference Image Color Match (Histogram / Moment Transfer)
                if color_match_str > 0.005 && params.color_match_stats.len() == 6 {
                    let t_mr = params.color_match_stats[0];
                    let t_sr = params.color_match_stats[1].max(0.02);
                    let t_mg = params.color_match_stats[2];
                    let t_sg = params.color_match_stats[3].max(0.02);
                    let t_mb = params.color_match_stats[4];
                    let t_sb = params.color_match_stats[5].max(0.02);

                    let mr = ((r - src_stats[0]) * (t_sr / src_stats[1]) + t_mr).clamp(0.0, 1.0);
                    let mg = ((g - src_stats[2]) * (t_sg / src_stats[3]) + t_mg).clamp(0.0, 1.0);
                    let mb = ((b - src_stats[4]) * (t_sb / src_stats[5]) + t_mb).clamp(0.0, 1.0);

                    r = r * (1.0 - color_match_str) + mr * color_match_str;
                    g = g * (1.0 - color_match_str) + mg * color_match_str;
                    b = b * (1.0 - color_match_str) + mb * color_match_str;
                }

                if params.grade_shadow_sat > 0.5 || params.grade_high_sat > 0.5 {
                    let (sr, sg, sb) = hsv_to_rgb(params.grade_shadow_hue, 1.0, 0.5);
                    let (hr, hg, hb) = hsv_to_rgb(params.grade_high_hue, 1.0, 0.5);
                    let s_amt = (params.grade_shadow_sat / 100.0) * 0.25 * (1.0 - lum);
                    let h_amt = (params.grade_high_sat / 100.0) * 0.25 * lum;
                    r += (sr - 0.25) * s_amt + (hr - 0.25) * h_amt;
                    g += (sg - 0.25) * s_amt + (hg - 0.25) * h_amt;
                    b += (sb - 0.25) * s_amt + (hb - 0.25) * h_amt;
                }

                let luma = 0.299 * r + 0.587 * g + 0.114 * b;
                let max_c = r.max(g).max(b);
                let min_c = r.min(g).min(b);
                let cur_sat = (max_c - min_c).clamp(0.0, 1.0);
                let vib_factor = 1.0 + vib_strength * (1.0 - cur_sat) * (1.0 - sw * 0.45);
                let total_sat = sat_mult * vib_factor;
                r = luma + (r - luma) * total_sat;
                g = luma + (g - luma) * total_sat;
                b = luma + (b - luma) * total_sat;

                match params.lut_preset.as_str() {
                    "clean_portrait" => {
                        let l = 0.299 * r + 0.587 * g + 0.114 * b;
                        r = (r * 1.03 + 0.015).min(1.0);
                        g = (g * 1.01 + 0.01).min(1.0);
                        b = (b * 0.99 + 0.015 * (1.0 - l)).min(1.0);
                    }
                    "fuji_pro" => {
                        let l = 0.299 * r + 0.587 * g + 0.114 * b;
                        r = r * 0.98 + 0.02 * l;
                        g = g * 1.03 + 0.01;
                        b = b * 1.02 + 0.025 * (1.0 - l);
                    }
                    "kodak_gold" => {
                        r = r * 1.06 + 0.02;
                        g = g * 1.01 + 0.01;
                        b = b * 0.92;
                    }
                    "cinematic_teal" => {
                        let l = 0.299 * r + 0.587 * g + 0.114 * b;
                        r += l * 0.06 - (1.0 - l) * 0.04;
                        g += 0.01;
                        b += (1.0 - l) * 0.08 - l * 0.05;
                    }
                    "warm_wedding" => {
                        r = r * 1.04 + 0.025;
                        g = g * 1.01 + 0.018;
                        b = b * 0.95 + 0.015;
                    }
                    "bw_studio" => {
                        let bw = (0.32 * r + 0.58 * g + 0.10 * b - 0.5) * 1.12 + 0.5;
                        r = bw;
                        g = bw;
                        b = bw;
                    }
                    _ => {}
                }

                // Extended 3-way colour grading: midtones, global, luminance per range, balance & blending
                if params.grade_mid_sat > 0.5
                    || params.grade_global_sat > 0.5
                    || params.grade_shadow_lum.abs() > 0.5
                    || params.grade_mid_lum.abs() > 0.5
                    || params.grade_high_lum.abs() > 0.5
                    || params.grade_global_lum.abs() > 0.5
                {
                    let l0 = (0.299 * r + 0.587 * g + 0.114 * b).clamp(0.0, 1.0);
                    let bal = params.grade_balance / 100.0;
                    let blend = (params.grade_blending / 100.0).clamp(0.0, 1.0);
                    let la = (l0 - bal * 0.25).clamp(0.0, 1.0);
                    let pw = 1.0 + (1.0 - blend) * 2.0;
                    let sh_w = (1.0 - la).powf(pw);
                    let hi_w = la.powf(pw);
                    let mid_w = (4.0 * la * (1.0 - la)).clamp(0.0, 1.0);
                    if params.grade_mid_sat > 0.5 {
                        let (tr, tg, tb) = hsv_to_rgb(params.grade_mid_hue, 1.0, 0.5);
                        let a = (params.grade_mid_sat / 100.0) * 0.25 * mid_w;
                        r += (tr - 0.25) * a;
                        g += (tg - 0.25) * a;
                        b += (tb - 0.25) * a;
                    }
                    if params.grade_global_sat > 0.5 {
                        let (tr, tg, tb) = hsv_to_rgb(params.grade_global_hue, 1.0, 0.5);
                        let a = (params.grade_global_sat / 100.0) * 0.20;
                        r += (tr - 0.25) * a;
                        g += (tg - 0.25) * a;
                        b += (tb - 0.25) * a;
                    }
                    let lum_shift = (params.grade_shadow_lum * sh_w
                        + params.grade_mid_lum * mid_w
                        + params.grade_high_lum * hi_w
                        + params.grade_global_lum)
                        / 100.0
                        * 0.25;
                    r += lum_shift;
                    g += lum_shift;
                    b += lum_shift;
                }

                // Colour noise reduction: replace noisy chroma with smoothed chroma, keep luminance detail
                if params.nr_color > 0.5 {
                    let k = (params.nr_color / 100.0).clamp(0.0, 1.0);
                    let l_cur = 0.299 * r + 0.587 * g + 0.114 * b;
                    let l_med = 0.299 * rm + 0.587 * gm + 0.114 * bm;
                    r = r * (1.0 - k) + (l_cur + (rm - l_med)) * k;
                    g = g * (1.0 - k) + (l_cur + (gm - l_med)) * k;
                    b = b * (1.0 - k) + (l_cur + (bm - l_med)) * k;
                }

                // Black & White treatment with 8-channel B&W mixer
                if params.treatment == "bw" {
                    let (hh, ss, _) = rgb_to_hsv(r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0));
                    let bw_band = |center: f32, width: f32| -> f32 {
                        let mut d = (hh - center).abs();
                        if d > 180.0 {
                            d = 360.0 - d;
                        }
                        (1.0 - d / width).clamp(0.0, 1.0)
                    };
                    let mix = bw_band(0.0, 30.0) * params.bw_red
                        + bw_band(28.0, 26.0) * params.bw_orange
                        + bw_band(58.0, 30.0) * params.bw_yellow
                        + bw_band(120.0, 45.0) * params.bw_green
                        + bw_band(180.0, 38.0) * params.bw_aqua
                        + bw_band(225.0, 42.0) * params.bw_blue
                        + bw_band(275.0, 38.0) * params.bw_purple
                        + bw_band(322.0, 36.0) * params.bw_magenta;
                    let base_l = 0.299 * r + 0.587 * g + 0.114 * b;
                    let v = (base_l * (1.0 + (mix / 100.0) * 0.9 * ss.clamp(0.0, 1.0))).clamp(0.0, 1.0);
                    r = v;
                    g = v;
                    b = v;
                }

                let out_idx = x * 3;
                row_out[out_idx] = r.clamp(0.0, 1.0);
                row_out[out_idx + 1] = g.clamp(0.0, 1.0);
                row_out[out_idx + 2] = b.clamp(0.0, 1.0);
            }
        });

    // =========================================================
    // Stage 2: 3D Liquify Facial & Body Reshaping + Transform + Vignette/Grain
    // =========================================================
    let needs_warp = params.face_width.abs() > 0.3
        || params.jawline_slim.abs() > 0.3
        || params.cheekbone_lift.abs() > 0.3
        || params.chin_length.abs() > 0.3
        || params.forehead_height.abs() > 0.3
        || params.nose_width.abs() > 0.3
        || params.nose_tip_lift.abs() > 0.3
        || params.mouth_size.abs() > 0.3
        || params.lip_plump.abs() > 0.3
        || params.smile_enhance > 0.3
        || params.eye_size.abs() > 0.3
        || params.eyebrow_lift.abs() > 0.3
        || params.hair_volume > 0.3
        || params.hair_part_line > 0.3
        || params.neck_slim > 0.3
        || params.shoulder_sculpt.abs() > 0.3
        || params.waist_slim > 0.3
        || params.double_chin > 0.3
        || params.rotate_angle.abs() > 0.1
        || params.keystone_vertical.abs() > 0.3
        || params.keystone_horizontal.abs() > 0.3
        || params.flip_horizontal
        || params.flip_vertical
        || (params.geo_scale - 100.0).abs() > 0.1
        || params.geo_offset_x.abs() > 0.1
        || params.geo_offset_y.abs() > 0.1
        || params.optics_distortion.abs() > 0.3
        || params.optics_ca_red.abs() > 0.3
        || params.optics_ca_blue.abs() > 0.3;

    let wf = w as f32;
    let hf = h as f32;
    let rad_rot = -params.rotate_angle.to_radians();
    let cos_r = rad_rot.cos();
    let sin_r = rad_rot.sin();

    let mut final_rgba = vec![0u8; w * h * 4];
    final_rgba
        .par_chunks_mut(w * 4)
        .enumerate()
        .for_each(|(y, row_out)| {
            let ny = y as f32 / hf;
            for x in 0..w {
                let nx = x as f32 / wf;
                let (mut r, mut g, mut b) = if needs_warp {
                    let mut sx = if params.flip_horizontal { 1.0 - nx } else { nx };
                    let mut sy = ny;

                    // Perspective Keystone & Free Rotation
                    if params.rotate_angle.abs() > 0.1
                        || params.keystone_vertical.abs() > 0.3
                        || params.keystone_horizontal.abs() > 0.3
                    {
                        let mut cx = sx - 0.5;
                        let mut cy = sy - 0.5;
                        let kv = (params.keystone_vertical / 100.0) * 0.28;
                        let kh = (params.keystone_horizontal / 100.0) * 0.28;
                        cx *= 1.0 + cy * kv;
                        cy *= 1.0 + cx * kh;
                        let rx = cx * cos_r - cy * sin_r;
                        let ry = cx * sin_r + cy * cos_r;
                        sx = rx + 0.5;
                        sy = ry + 0.5;
                    }

                    // 1. Eye Size (Anchored on exact 478-point left & right iris centers)
                    if params.eye_size.abs() > 0.3 {
                        let strength = (params.eye_size / 100.0) * 0.42;
                        let eye_r = face.face_rx * 0.28;
                        for &(ex, ey) in &[
                            (face.left_eye_x, face.left_eye_y),
                            (face.right_eye_x, face.right_eye_y),
                        ] {
                            let dx = sx - ex;
                            let dy = sy - ey;
                            let d = (dx * dx + dy * dy).sqrt();
                            if d < eye_r && eye_r > 1e-4 {
                                let factor = 1.0 - strength * (1.0 - (d / eye_r).powi(2)).powi(2);
                                sx = ex + dx * factor;
                                sy = ey + dy * factor;
                            }
                        }
                    }

                    // 2. Eyebrow Lift (Strictly above eyes)
                    if params.eyebrow_lift.abs() > 0.3 {
                        let lift = (params.eyebrow_lift / 100.0) * 0.038;
                        for &(ex, ey) in &[
                            (face.left_eye_x, face.left_eye_y),
                            (face.right_eye_x, face.right_eye_y),
                        ] {
                            let by = ey - face.face_ry * 0.13;
                            let dx = (sx - ex) / (face.face_rx * 0.30);
                            let dy = (sy - by) / (face.face_ry * 0.11);
                            let r2 = dx * dx + dy * dy;
                            if r2 < 1.0 {
                                sy += lift * (1.0 - r2).powi(2);
                            }
                        }
                    }

                    // 3. Face Width (Anchored on exact Left & Right Cheekbone Landmarks 234 & 454, protecting center features!)
                    if params.face_width.abs() > 0.3 {
                        let fw = (params.face_width / 100.0) * 0.085 * face.face_rx;
                        for &(cx_cheek, cy_cheek, dir) in &[
                            (face.cheek_left_x, face.cheek_left_y, -1.0f32),
                            (face.cheek_right_x, face.cheek_right_y, 1.0f32),
                        ] {
                            let dx = (sx - cx_cheek) / (face.face_rx * 0.55);
                            let dy = (sy - cy_cheek) / (face.face_ry * 0.58);
                            let r2 = dx * dx + dy * dy;
                            if r2 < 1.0 {
                                sx -= dir * fw * (1.0 - r2).powi(2);
                            }
                        }
                    }

                    // 4. Jawline Slim (Anchored on exact Left & Right Jaw Angles 172 & 397!)
                    if params.jawline_slim.abs() > 0.3 {
                        let jw = (params.jawline_slim / 100.0) * 0.095 * face.face_rx;
                        for &(jx, jy, dir) in &[
                            (face.jaw_left_x, face.jaw_left_y, -1.0f32),
                            (face.jaw_right_x, face.jaw_right_y, 1.0f32),
                        ] {
                            let dx = (sx - jx) / (face.face_rx * 0.58);
                            let dy = (sy - jy) / (face.face_ry * 0.48);
                            let r2 = dx * dx + dy * dy;
                            if r2 < 1.0 {
                                let w_fall = (1.0 - r2).powi(2);
                                sx += dir * jw * w_fall;
                                sy += jw * 0.25 * w_fall;
                            }
                        }
                    }

                    // 5. Cheekbone Lift (Lifts left & right cheekbones vertically)
                    if params.cheekbone_lift.abs() > 0.3 {
                        let lift = (params.cheekbone_lift / 100.0) * 0.042 * face.face_ry;
                        for &(cx_cheek, cy_cheek) in &[
                            (face.cheek_left_x, face.cheek_left_y),
                            (face.cheek_right_x, face.cheek_right_y),
                        ] {
                            let dx = (sx - cx_cheek) / (face.face_rx * 0.52);
                            let dy = (sy - cy_cheek) / (face.face_ry * 0.42);
                            let r2 = dx * dx + dy * dy;
                            if r2 < 1.0 {
                                sy += lift * (1.0 - r2).powi(2);
                            }
                        }
                    }

                    // 6. Chin Length & Double Chin Tuck (Anchored on exact Chin Tip 152, below mouth!)
                    if params.chin_length.abs() > 0.3 || params.double_chin > 0.3 {
                        let shift = (-params.chin_length / 100.0) * 0.075 * face.face_ry
                            + (params.double_chin / 100.0) * 0.055 * face.face_ry;
                        let dx = (sx - face.chin_x) / (face.face_rx * 0.68);
                        let dy = (sy - face.chin_y) / (face.face_ry * 0.36);
                        let r2 = dx * dx + dy * dy;
                        if r2 < 1.0 && sy > face.mouth_y + face.face_ry * 0.08 {
                            sy += shift * (1.0 - r2).powi(2);
                        }
                    }

                    // 7. Forehead Height (Anchored on exact Forehead Top 10, above eyebrows!)
                    if params.forehead_height.abs() > 0.3 {
                        let fh = (params.forehead_height / 100.0) * 0.065 * face.face_ry;
                        let dx = (sx - face.forehead_x) / (face.face_rx * 0.90);
                        let dy = (sy - face.forehead_y) / (face.face_ry * 0.42);
                        let r2 = dx * dx + dy * dy;
                        if r2 < 1.0 {
                            sy += fh * (1.0 - r2).powi(2);
                        }
                    }

                    // 8. Nose Width & Nose Tip Lift (Anchored on Nose Tip 4)
                    if params.nose_width.abs() > 0.3 || params.nose_tip_lift.abs() > 0.3 {
                        let nw = (params.nose_width / 100.0) * 0.24;
                        let nl = (params.nose_tip_lift / 100.0) * 0.036 * face.face_ry;
                        let dx = (sx - face.nose_x) / (face.face_rx * 0.28);
                        let dy = (sy - face.nose_y) / (face.face_ry * 0.22);
                        let r2 = dx * dx + dy * dy;
                        if r2 < 1.0 {
                            let fall = (1.0 - r2).powi(2);
                            sx -= (sx - face.nose_x) * nw * fall;
                            sy += nl * fall;
                        }
                    }

                    // 9. Mouth Size & Lip Plump (Anchored on exact Mouth Center)
                    if params.mouth_size.abs() > 0.3 || params.lip_plump.abs() > 0.3 {
                        let ms = (params.mouth_size / 100.0) * 0.30;
                        let lp = (params.lip_plump / 100.0) * 0.38;
                        let dx = sx - face.mouth_x;
                        let dy = sy - face.mouth_y;
                        let rx = face.face_rx * 0.34;
                        let ry = face.face_ry * 0.16;
                        let r2 = (dx / rx).powi(2) + (dy / ry).powi(2);
                        if r2 < 1.0 {
                            let fall = (1.0 - r2).powi(2);
                            sx -= dx * ms * fall;
                            sy -= dy * (ms * 0.45 + lp) * fall;
                        }
                    }

                    // 10. Smile Enhance (Anchored on exact Left & Right Mouth Corners 61 & 291!)
                    if params.smile_enhance > 0.3 {
                        let lift = (params.smile_enhance / 100.0) * 0.038 * face.face_ry;
                        for &(mcx, mcy) in &[
                            (face.mouth_left_x, face.mouth_left_y),
                            (face.mouth_right_x, face.mouth_right_y),
                        ] {
                            let dx = (sx - mcx) / (face.face_rx * 0.18);
                            let dy = (sy - mcy) / (face.face_ry * 0.16);
                            let r2 = dx * dx + dy * dy;
                            if r2 < 1.0 {
                                sy += lift * (1.0 - r2).powi(2);
                            }
                        }
                    }

                    // 11. Hair Volume & Hair Part Line
                    if params.hair_volume > 0.3 || params.hair_part_line > 0.3 {
                        let vol = ((params.hair_volume + params.hair_part_line * 0.6) / 100.0) * 0.068 * face.face_ry;
                        let crown_y = (face.forehead_y - face.face_ry * 0.18).max(0.04);
                        let dx = (sx - face.center_x) / (face.face_rx * 1.40);
                        let dy = (sy - crown_y) / (face.face_ry * 0.68);
                        let r2 = dx * dx + dy * dy;
                        if r2 < 1.0 {
                            sy += vol * (1.0 - r2).powi(2);
                        }
                    }

                    // 12. Body Reshape: Neck Slim, Shoulder Sculpt, Waist Slim
                    if params.neck_slim > 0.3 {
                        let ns = (params.neck_slim / 100.0) * 0.14;
                        let neck_y = face.chin_y + face.face_ry * 0.32;
                        let dx = (sx - face.center_x) / (face.face_rx * 0.85);
                        let dy = (sy - neck_y) / (face.face_ry * 0.45);
                        let r2 = dx * dx + dy * dy;
                        if r2 < 1.0 {
                            sx += (sx - face.center_x) * ns * (1.0 - r2).powi(2);
                        }
                    }

                    if params.shoulder_sculpt.abs() > 0.3 {
                        let ss = (params.shoulder_sculpt / 100.0) * 0.065;
                        let sh_y = face.chin_y + face.face_ry * 0.85;
                        let dx = (sx - face.center_x) / (face.face_rx * 2.4);
                        let dy = (sy - sh_y) / (face.face_ry * 0.70);
                        let r2 = dx * dx + dy * dy;
                        if r2 < 1.0 {
                            sy += ss * (1.0 - r2).powi(2);
                        }
                    }

                    if params.waist_slim > 0.3 && sy > face.chin_y + face.face_ry * 0.8 {
                        let ws = (params.waist_slim / 100.0) * 0.13;
                        let waist_y = (face.chin_y + face.face_ry * 1.8).min(0.92);
                        let dx = (sx - face.center_x) / (face.face_rx * 2.4);
                        let dy = (sy - waist_y) / (face.face_ry * 1.10);
                        let r2 = dx * dx + dy * dy;
                        if r2 < 1.0 {
                            sx += (sx - face.center_x) * ws * (1.0 - r2).powi(2);
                        }
                    }

                    // Vertical flip, geometry scale/offset, lens distortion
                    if params.flip_vertical {
                        sy = 1.0 - sy;
                    }
                    if (params.geo_scale - 100.0).abs() > 0.1 || params.geo_offset_x.abs() > 0.1 || params.geo_offset_y.abs() > 0.1 {
                        let sc = (params.geo_scale / 100.0).max(0.1);
                        sx = 0.5 + (sx - 0.5) / sc - params.geo_offset_x / 100.0 * 0.5;
                        sy = 0.5 + (sy - 0.5) / sc - params.geo_offset_y / 100.0 * 0.5;
                    }
                    if params.optics_distortion.abs() > 0.3 {
                        let cx = sx - 0.5;
                        let cy = sy - 0.5;
                        let k = (params.optics_distortion / 100.0) * 0.6;
                        let f = 1.0 + k * (cx * cx + cy * cy) * 4.0;
                        sx = 0.5 + cx * f;
                        sy = 0.5 + cy * f;
                    }

                    let (mut pr, pg, mut pb) = sample_bilinear(&retouched_rgb, w, h, sx * wf, sy * hf);
                    if params.optics_ca_red.abs() > 0.3 || params.optics_ca_blue.abs() > 0.3 {
                        let cx = sx - 0.5;
                        let cy = sy - 0.5;
                        let kr = params.optics_ca_red / 100.0 * 0.012;
                        let kb = params.optics_ca_blue / 100.0 * 0.012;
                        pr = sample_bilinear(&retouched_rgb, w, h, (sx + cx * kr) * wf, (sy + cy * kr) * hf).0;
                        pb = sample_bilinear(&retouched_rgb, w, h, (sx + cx * kb) * wf, (sy + cy * kb) * hf).2;
                    }
                    (pr, pg, pb)
                } else {
                    let idx = (y * w + x) * 3;
                    (retouched_rgb[idx], retouched_rgb[idx + 1], retouched_rgb[idx + 2])
                };

                // Defringe: desaturate purple / green fringes on bright areas
                if params.optics_defringe_purple > 0.5 || params.optics_defringe_green > 0.5 {
                    let (hh, ss, vv) = rgb_to_hsv(r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0));
                    let band = |c: f32, wd: f32| -> f32 {
                        let mut d = (hh - c).abs();
                        if d > 180.0 {
                            d = 360.0 - d;
                        }
                        (1.0 - d / wd).clamp(0.0, 1.0)
                    };
                    let kill = band(285.0, 40.0) * (params.optics_defringe_purple / 100.0).clamp(0.0, 1.0)
                        + band(110.0, 40.0) * (params.optics_defringe_green / 100.0).clamp(0.0, 1.0);
                    let kill = (kill * smoothstep(0.35, 0.8, vv)).clamp(0.0, 1.0);
                    if kill > 0.0 {
                        let (nr, ng, nb) = hsv_to_rgb(hh, ss * (1.0 - kill), vv);
                        r = nr;
                        g = ng;
                        b = nb;
                    }
                }

                // Film grain: size, roughness, strongest in midtones
                if params.film_grain > 0.5 {
                    let gr = lc_grain_noise(x as f32, y as f32, params.film_grain, params.grain_size, params.grain_roughness);
                    let l = ((r + g + b) / 3.0).clamp(0.0, 1.0);
                    let lw = 0.45 + 0.55 * 4.0 * l * (1.0 - l);
                    r += gr * lw;
                    g += gr * lw;
                    b += gr * lw;
                }

                // Post-crop vignette: amount, midpoint, roundness, feather, highlight priority
                if params.vignette.abs() > 0.5 {
                    let ro = params.vignette_roundness / 100.0;
                    let aspect = hf / wf;
                    let vx = (nx - 0.5) * 2.0;
                    let mut vy = (ny - 0.5) * 2.0;
                    if ro > 0.0 {
                        vy *= 1.0 + (aspect - 1.0) * ro;
                    }
                    let p = if ro < 0.0 { 2.0 + (-ro) * 2.0 } else { 2.0 };
                    let d = (vx.abs().powf(p) + vy.abs().powf(p)).powf(1.0 / p) / std::f32::consts::SQRT_2;
                    let start = (params.vignette_midpoint / 100.0).clamp(0.0, 1.0) * 0.95;
                    let width = 0.12 + (params.vignette_feather / 100.0).clamp(0.0, 1.0) * 0.88;
                    let t = smoothstep(start, start + width, d);
                    let lum_here = (0.299 * r + 0.587 * g + 0.114 * b).clamp(0.0, 1.0);
                    let protect = 1.0 - (params.vignette_highlights / 100.0).clamp(0.0, 1.0) * 0.85 * lum_here;
                    let v_mult = (1.0 - (params.vignette / 100.0) * t * protect * 0.9).clamp(0.05, 1.9);
                    r *= v_mult;
                    g *= v_mult;
                    b *= v_mult;
                }

                // Lens-profile vignetting correction (positive brightens the corners)
                if params.optics_vignetting.abs() > 0.5 {
                    let vx = (nx - 0.5) * 2.0;
                    let vy = (ny - 0.5) * 2.0;
                    let d = (vx * vx + vy * vy).sqrt() / std::f32::consts::SQRT_2;
                    let start = (params.optics_vignetting_midpoint / 100.0).clamp(0.0, 1.0) * 0.8;
                    let t = smoothstep(start, 1.0, d);
                    let m = (1.0 + (params.optics_vignetting / 100.0) * t * 0.9).clamp(0.2, 2.0);
                    r *= m;
                    g *= m;
                    b *= m;
                }

                let out_idx = x * 4;
                row_out[out_idx] = (r.clamp(0.0, 1.0) * 255.0).round() as u8;
                row_out[out_idx + 1] = (g.clamp(0.0, 1.0) * 255.0).round() as u8;
                row_out[out_idx + 2] = (b.clamp(0.0, 1.0) * 255.0).round() as u8;
                row_out[out_idx + 3] = 255;
            }
        });

    RgbaImage::from_raw(w_u32, h_u32, final_rgba).unwrap_or_else(|| img.clone())
}

fn decode_data_url(data_url: &str) -> Result<DynamicImage, String> {
    let b64_part = data_url
        .split_once(',')
        .map(|(_, b64)| b64)
        .unwrap_or(data_url);
    let bytes = BASE64
        .decode(b64_part)
        .map_err(|e| format!("Base64 decode error: {}", e))?;
    image::load_from_memory(&bytes).map_err(|e| format!("Image decode error: {}", e))
}

fn encode_jpeg_data_url(img: &RgbaImage, quality: u8) -> Result<String, String> {
    let mut buf = Cursor::new(Vec::new());
    let dyn_img = DynamicImage::ImageRgba8(img.clone());
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, quality);
    encoder
        .encode_image(&dyn_img)
        .map_err(|e| format!("JPEG encode error: {}", e))?;
    Ok(format!(
        "data:image/jpeg;base64,{}",
        BASE64.encode(buf.into_inner())
    ))
}

/// Read EXIF / TIFF Orientation tag (0x0112) from JPEG, TIFF, or Canon CR2/CR3 header
fn read_exif_orientation_tag(path: &Path) -> u16 {
    use std::io::Read;
    let mut file = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return 1,
    };
    let mut buf = vec![0u8; 65536];
    let n = match file.read(&mut buf) {
        Ok(n) if n >= 16 => n,
        _ => return 1,
    };
    let data = &buf[..n];

    if (data.starts_with(b"II*\0") || data.starts_with(b"MM\0*")) && data.len() >= 12 {
        if let Some(o) = parse_tiff_orientation(data) {
            return o;
        }
    }

    for i in 0..data.len().saturating_sub(16) {
        if &data[i..i + 6] == b"Exif\0\0" {
            if let Some(o) = parse_tiff_orientation(&data[i + 6..]) {
                return o;
            }
        }
    }
    1
}

fn parse_tiff_orientation(tiff: &[u8]) -> Option<u16> {
    if tiff.len() < 12 {
        return None;
    }
    let le = if &tiff[0..2] == b"II" {
        true
    } else if &tiff[0..2] == b"MM" {
        false
    } else {
        return None;
    };
    let r16 = |off: usize| -> Option<u16> {
        let s = tiff.get(off..off + 2)?;
        Some(if le {
            u16::from_le_bytes([s[0], s[1]])
        } else {
            u16::from_be_bytes([s[0], s[1]])
        })
    };
    let r32 = |off: usize| -> Option<u32> {
        let s = tiff.get(off..off + 4)?;
        Some(if le {
            u32::from_le_bytes([s[0], s[1], s[2], s[3]])
        } else {
            u32::from_be_bytes([s[0], s[1], s[2], s[3]])
        })
    };

    let ifd0 = r32(4)? as usize;
    let count = r16(ifd0)? as usize;
    for idx in 0..count.min(64) {
        let entry = ifd0 + 2 + idx * 12;
        let tag = r16(entry)?;
        if tag == 0x0112 {
            let val = r16(entry + 8)?;
            if (1..=8).contains(&val) {
                return Some(val);
            }
        }
    }
    None
}

fn apply_exif_orientation(img: DynamicImage, orientation: u16) -> DynamicImage {
    match orientation {
        2 => img.fliph(),
        3 => img.rotate180(),
        4 => img.flipv(),
        5 => img.rotate90().fliph(),
        6 => img.rotate90(),
        7 => img.rotate270().fliph(),
        8 => img.rotate270(),
        _ => img,
    }
}

/// Decode Canon CR2 / camera RAW files and automatically apply vertical/horizontal EXIF orientation
fn decode_raw_or_image(path: &Path) -> Result<DynamicImage, String> {
    let exif_orient = read_exif_orientation_tag(path);

    if let Ok(img) = image::open(path) {
        return Ok(apply_exif_orientation(img, exif_orient));
    }

    #[cfg(target_os = "macos")]
    {
        let tmp_jpg = std::env::temp_dir().join(format!(
            "retouch_raw_{}.jpg",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        if let Ok(status) = std::process::Command::new("sips")
            .args([
                "-s",
                "format",
                "jpeg",
                "-s",
                "formatOptions",
                "96",
                "-Z",
                "2400",
                path.to_str().unwrap_or_default(),
                "--out",
                tmp_jpg.to_str().unwrap_or_default(),
            ])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
        {
            if status.success() {
                let jpg_orient = read_exif_orientation_tag(&tmp_jpg);
                if let Ok(img) = image::open(&tmp_jpg) {
                    let _ = std::fs::remove_file(&tmp_jpg);
                    let effective_orient = if jpg_orient != 1 {
                        jpg_orient
                    } else if exif_orient != 1 && img.width() > img.height() {
                        exif_orient
                    } else {
                        1
                    };
                    return Ok(apply_exif_orientation(img, effective_orient));
                }
            }
        }
        let _ = std::fs::remove_file(&tmp_jpg);
    }

    let raw = rawloader::decode_file(path)
        .map_err(|e| format!("RAW decode error for {:?}: {}", path.file_name(), e))?;

    let w = raw.width;
    let h = raw.height;
    if w < 4 || h < 4 {
        return Err("Invalid RAW sensor dimensions".to_string());
    }

    let wb = raw.wb_coeffs;
    let r_gain = if wb[0].is_finite() && wb[0] > 0.0 { wb[0] } else { 1.9 };
    let g_gain = if wb[1].is_finite() && wb[1] > 0.0 { wb[1] } else { 1.0 };
    let b_gain = if wb[2].is_finite() && wb[2] > 0.0 { wb[2] } else { 1.5 };
    let min_gain = r_gain.min(g_gain).min(b_gain).max(1e-4);
    let rg = r_gain / min_gain;
    let gg = g_gain / min_gain;
    let bg = b_gain / min_gain;

    let black = raw.blacklevels[0] as f32;
    let white = (raw.whitelevels[0] as f32).max(black + 100.0);
    let range = white - black;

    let sensor_f32: Vec<f32> = match raw.data {
        rawloader::RawImageData::Integer(ref data) => data
            .par_iter()
            .map(|&v| ((v as f32 - black) / range).clamp(0.0, 1.0))
            .collect(),
        rawloader::RawImageData::Float(ref data) => data
            .par_iter()
            .map(|&v| ((v - black) / range).clamp(0.0, 1.0))
            .collect(),
    };

    let out_w = w / 2;
    let out_h = h / 2;
    let cfa = &raw.cfa;
    let mut rgb_out = vec![0u8; out_w * out_h * 4];

    rgb_out
        .par_chunks_mut(out_w * 4)
        .enumerate()
        .for_each(|(oy, row_out)| {
            let sy = oy * 2;
            for ox in 0..out_w {
                let sx = ox * 2;
                let mut r_acc = 0.0f32;
                let mut g_acc = 0.0f32;
                let mut b_acc = 0.0f32;
                let mut rc = 0.0f32;
                let mut gc = 0.0f32;
                let mut bc = 0.0f32;

                for dy in 0..2 {
                    for dx in 0..2 {
                        let px = sx + dx;
                        let py = sy + dy;
                        let val = sensor_f32[py * w + px];
                        match cfa.color_at(py, px) {
                            0 => {
                                r_acc += val;
                                rc += 1.0;
                            }
                            1 | 3 => {
                                g_acc += val;
                                gc += 1.0;
                            }
                            2 => {
                                b_acc += val;
                                bc += 1.0;
                            }
                            _ => {
                                g_acc += val;
                                gc += 1.0;
                            }
                        }
                    }
                }

                let r_lin = ((r_acc / rc.max(1.0)) * rg).clamp(0.0, 1.0);
                let g_lin = ((g_acc / gc.max(1.0)) * gg).clamp(0.0, 1.0);
                let b_lin = ((b_acc / bc.max(1.0)) * bg).clamp(0.0, 1.0);

                let r_srgb = (r_lin.powf(1.0 / 2.2) * 255.0).round() as u8;
                let g_srgb = (g_lin.powf(1.0 / 2.2) * 255.0).round() as u8;
                let b_srgb = (b_lin.powf(1.0 / 2.2) * 255.0).round() as u8;

                let idx = ox * 4;
                row_out[idx] = r_srgb;
                row_out[idx + 1] = g_srgb;
                row_out[idx + 2] = b_srgb;
                row_out[idx + 3] = 255;
            }
        });

    let rgba_img = RgbaImage::from_raw(out_w as u32, out_h as u32, rgb_out)
        .ok_or_else(|| "Failed to build decoded RAW buffer".to_string())?;
    let dyn_raw = DynamicImage::ImageRgba8(rgba_img);
    Ok(apply_exif_orientation(dyn_raw, exif_orient))
}

#[tauri::command]
fn load_photo_from_path_cmd(file_path: String) -> Result<ImportedPhotoPayload, String> {
    let path = Path::new(&file_path);
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Imported_Photo.CR2")
        .to_string();

    let dyn_img = decode_raw_or_image(path)?;
    let orig_w = dyn_img.width();
    let orig_h = dyn_img.height();

    let master = if orig_w > 2200 || orig_h > 2200 {
        dyn_img.resize(2200, 2200, image::imageops::FilterType::Lanczos3)
    } else {
        dyn_img
    };

    let rgba = master.to_rgba8();
    let detected_face = detect_face_region(&rgba);
    let data_url = encode_jpeg_data_url(&rgba, 94)?;

    Ok(ImportedPhotoPayload {
        path: file_path,
        name,
        data_url,
        width: orig_w,
        height: orig_h,
        detected_face,
    })
}

#[tauri::command]
fn rotate_photo_90_cmd(image_base64: String, clockwise: bool) -> Result<ImportedPhotoPayload, String> {
    let dyn_img = decode_data_url(&image_base64)?;
    let rotated = if clockwise {
        dyn_img.rotate90()
    } else {
        dyn_img.rotate270()
    };
    let rgba = rotated.to_rgba8();
    let w = rgba.width();
    let h = rgba.height();
    let detected_face = detect_face_region(&rgba);
    let data_url = encode_jpeg_data_url(&rgba, 94)?;

    Ok(ImportedPhotoPayload {
        path: String::new(),
        name: String::new(),
        data_url,
        width: w,
        height: h,
        detected_face,
    })
}

#[tauri::command]
fn analyze_portrait_cmd(image_base64: String) -> Result<FaceRegion, String> {
    let dyn_img = decode_data_url(&image_base64)?;
    let rgba = dyn_img.to_rgba8();
    Ok(detect_face_region(&rgba))
}

#[tauri::command]
fn process_photo_cmd(
    image_base64: String,
    params: RetouchParams,
    high_quality: bool,
    mask_base64: Option<String>,
) -> Result<ProcessResult, String> {
    let start = std::time::Instant::now();
    let dyn_img = decode_data_url(&image_base64)?;

    let max_dim = if high_quality { 3840 } else { 1350 };
    let working_img = if dyn_img.width() > max_dim || dyn_img.height() > max_dim {
        dyn_img
            .resize(max_dim, max_dim, image::imageops::FilterType::Triangle)
            .to_rgba8()
    } else {
        dyn_img.to_rgba8()
    };

    let decoded_mask = mask_base64
        .as_deref()
        .and_then(|b64| decode_data_url(b64).ok())
        .map(|d| d.to_rgba8());

    let mut active_params = params;
    if active_params.face.face_rx <= 0.01 {
        active_params.face = detect_face_region(&working_img);
    }
    let detected = active_params.face.clone();

    let result_rgba = apply_retouch_pipeline(&working_img, &active_params, decoded_mask.as_ref());
    let quality = if high_quality { 95 } else { 90 };
    let data_url = encode_jpeg_data_url(&result_rgba, quality)?;

    Ok(ProcessResult {
        data_url,
        width: result_rgba.width(),
        height: result_rgba.height(),
        elapsed_ms: start.elapsed().as_millis(),
        detected_face: detected,
    })
}

#[tauri::command]
fn export_photo_to_disk_cmd(
    image_base64: String,
    params: RetouchParams,
    output_path: String,
    mask_base64: Option<String>,
) -> Result<String, String> {
    let dyn_img = decode_data_url(&image_base64)?;
    let rgba = dyn_img.to_rgba8();
    let decoded_mask = mask_base64
        .as_deref()
        .and_then(|b64| decode_data_url(b64).ok())
        .map(|d| d.to_rgba8());

    let mut active_params = params;
    if active_params.face.face_rx <= 0.01 {
        active_params.face = detect_face_region(&rgba);
    }
    let result_rgba = apply_retouch_pipeline(&rgba, &active_params, decoded_mask.as_ref());
    let dyn_out = DynamicImage::ImageRgba8(result_rgba);
    if output_path.to_lowercase().ends_with(".png") {
        dyn_out
            .save_with_format(&output_path, ImageFormat::Png)
            .map_err(|e| format!("Save error: {}", e))?;
    } else {
        dyn_out
            .save_with_format(&output_path, ImageFormat::Jpeg)
            .map_err(|e| format!("Save error: {}", e))?;
    }
    Ok(output_path)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            load_photo_from_path_cmd,
            rotate_photo_90_cmd,
            analyze_portrait_cmd,
            process_photo_cmd,
            export_photo_to_disk_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running Retouch Pro");
}

#[cfg(test)]
mod slider_tests {
    use super::*;

    /// Full-hue test chart: every hue band present, varying brightness, plus fine noise.
    fn test_image() -> RgbaImage {
        let (w, h) = (192u32, 160u32);
        let mut img = RgbaImage::new(w, h);
        for y in 0..h {
            for x in 0..w {
                let n = hash_u32(x * 7919 + y * 104729) & 0xff;
                let hue = x as f32 / w as f32 * 360.0;
                let val = 0.30 + 0.60 * y as f32 / h as f32;
                let (r, g, b) = hsv_to_rgb(hue, 0.85, val);
                let jitter = (n as f32 - 128.0) / 255.0 * 0.08;
                let c = |v: f32| ((v + jitter).clamp(0.0, 1.0) * 255.0) as u8;
                img.put_pixel(x, y, image::Rgba([c(r), c(g), c(b), 255]));
            }
        }
        img
    }

    fn run(img: &RgbaImage, v: &serde_json::Value) -> Vec<u8> {
        let mut p: RetouchParams = serde_json::from_value(v.clone()).unwrap();
        p.face = detect_face_region(img);
        apply_retouch_pipeline(img, &p, None).into_raw()
    }

    fn dead_params(img: &RgbaImage, base: &serde_json::Value, only: Option<&Vec<String>>) -> Vec<String> {
        let baseline = run(img, base);
        let mut dead = Vec::new();
        for (key, val) in base.as_object().unwrap() {
            if key == "face" || key == "blemishSpots" {
                continue;
            }
            if let Some(list) = only {
                if !list.contains(key) {
                    continue;
                }
            }
            let candidates: Vec<serde_json::Value> = if let Some(n) = val.as_f64() {
                vec![serde_json::json!(n + 60.0), serde_json::json!(n - 60.0), serde_json::json!(n + 25.0)]
            } else if val.is_boolean() {
                vec![serde_json::json!(!val.as_bool().unwrap())]
            } else {
                continue;
            };
            let mut changed = false;
            for c in candidates {
                let mut v = base.clone();
                v[key] = c;
                if run(img, &v) != baseline {
                    changed = true;
                    break;
                }
            }
            if !changed {
                dead.push(key.clone());
            }
        }
        dead
    }

    /// Every slider must change the image once the slider it depends on is switched on.
    #[test]
    fn every_slider_changes_the_image() {
        let img = test_image();
        let plain = serde_json::to_value(RetouchParams::default()).unwrap();
        let dead_plain = dead_params(&img, &plain, None);

        // Second pass: switch on "master" sliders that child sliders depend on.
        let mut enabled = plain.clone();
        for (k, v) in [
            ("sharpenAmount", 60.0), ("vignette", 60.0), ("filmGrain", 60.0),
            ("gradeShadowSat", 60.0), ("gradeHighSat", 60.0), ("gradeMidSat", 60.0), ("gradeGlobalSat", 60.0),
            ("opticsVignetting", 60.0),
        ] {
            enabled[k] = serde_json::json!(v);
        }
        enabled["treatment"] = serde_json::json!("bw");
        let still_dead = dead_params(&img, &enabled, Some(&dead_plain));
        println!("NO-EFFECT (even with masters on): {:?}", still_dead);

        // Third pass: face / hair / teeth sliders need a real portrait.
        let p_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../public/samples/portrait_1.jpg");
        if let Ok(pimg) = image::open(&p_path) {
            let pimg = pimg.to_rgba8();
            let nh = (384.0 * pimg.height() as f32 / pimg.width() as f32) as u32;
            let pimg = image::imageops::resize(&pimg, 384, nh, image::imageops::FilterType::Triangle);
            let mut en2 = enabled.clone();
            en2["treatment"] = serde_json::json!("color");
            en2["hairDyeColor"] = serde_json::json!("espresso");
            en2["colorMatchStats"] = serde_json::json!([0.4, 0.2, 0.5, 0.2, 0.3, 0.2]);
            en2["gradeShadowLum"] = serde_json::json!(40.0);
            let dead_portrait = dead_params(&pimg, &en2, Some(&still_dead));
            println!("NO-EFFECT ON REAL PORTRAIT: {:?}", dead_portrait);
        }
    }

    fn mean_abs_err(a: &[u8], b: &[u8]) -> f32 {
        let mut s = 0.0f64;
        for (x, y) in a.chunks(4).zip(b.chunks(4)) {
            for c in 0..3 {
                s += (x[c] as f64 - y[c] as f64).abs();
            }
        }
        (s / (a.len() as f64 / 4.0 * 3.0)) as f32
    }

    #[test]
    fn ai_denoise_removes_noise_and_keeps_edges() {
        let (w, h) = (128u32, 128u32);
        let mut clean = RgbaImage::new(w, h);
        let mut noisy = RgbaImage::new(w, h);
        for y in 0..h {
            for x in 0..w {
                // two flat colour regions with a hard vertical edge + a soft gradient
                let (r, g, b) = if x < w / 2 { (60.0, 90.0, 140.0) } else { (200.0, 170.0, 110.0) };
                let grad = y as f32 / h as f32 * 30.0;
                let base = [r + grad, g + grad, b + grad];
                let mut n = [0.0f32; 3];
                for c in 0..3 {
                    // luminance + chroma noise, roughly +-14 levels
                    n[c] = (hash2(x as i32, y as i32, 100 + c as u32) * 2.0) * 14.0;
                }
                clean.put_pixel(x, y, image::Rgba([base[0] as u8, base[1] as u8, base[2] as u8, 255]));
                let f = |v: f32| v.clamp(0.0, 255.0) as u8;
                noisy.put_pixel(x, y, image::Rgba([f(base[0] + n[0]), f(base[1] + n[1]), f(base[2] + n[2]), 255]));
            }
        }
        let mut v = serde_json::to_value(RetouchParams::default()).unwrap();
        let before = mean_abs_err(noisy.as_raw(), clean.as_raw());
        v["aiDenoise"] = serde_json::json!(70.0);
        let out = run(&noisy, &v);
        let after = mean_abs_err(&out, clean.as_raw());
        println!("denoise error: {before:.2} -> {after:.2}");
        assert!(after < before * 0.55, "noise should drop by at least 45% ({before} -> {after})");

        // the hard edge in the middle must still be there
        let row = 64usize;
        let left = out[(row * w as usize + 20) * 4] as i32;
        let right = out[(row * w as usize + 108) * 4] as i32;
        assert!((right - left).abs() > 100, "edge was destroyed: {left} vs {right}");
        let near_l = out[(row * w as usize + 62) * 4] as i32;
        let near_r = out[(row * w as usize + 65) * 4] as i32;
        assert!((near_r - near_l).abs() > 80, "edge was blurred: {near_l} vs {near_r}");
    }

    #[test]
    fn black_and_white_treatment_is_monochrome() {
        let img = test_image();
        let mut v = serde_json::to_value(RetouchParams::default()).unwrap();
        v["treatment"] = serde_json::json!("bw");
        let out = run(&img, &v);
        assert!(out.chunks(4).all(|p| p[0] == p[1] && p[1] == p[2]));
        v["bwRed"] = serde_json::json!(80.0);
        v["bwBlue"] = serde_json::json!(-80.0);
        assert_ne!(run(&img, &v), out);
    }
}
