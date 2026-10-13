use image::{imageops::FilterType, DynamicImage, Rgba, RgbaImage};
use std::collections::VecDeque;
use std::path::Path;

fn center_on_1024(
    crop: &RgbaImage,
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
) -> RgbaImage {
    if max_x <= min_x || max_y <= min_y {
        return RgbaImage::new(1024, 1024);
    }
    let bw = (max_x - min_x + 1) as u32;
    let bh = (max_y - min_y + 1) as u32;
    let trimmed = image::imageops::crop_imm(crop, min_x as u32, min_y as u32, bw, bh).to_image();

    let target_max = 910.0f32;
    let scale = (target_max / (bw.max(bh) as f32)).min(8.0);
    let nw = ((bw as f32 * scale).round() as u32).clamp(1, 1000);
    let nh = ((bh as f32 * scale).round() as u32).clamp(1, 1000);

    let resized = DynamicImage::ImageRgba8(trimmed)
        .resize_exact(nw, nh, FilterType::Lanczos3)
        .to_rgba8();

    let mut out = RgbaImage::new(1024, 1024);
    let ox = (1024 - nw) / 2;
    let oy = (1024 - nh) / 2;

    for y in 0..nh {
        for x in 0..nw {
            let p = *resized.get_pixel(x, y);
            if p[3] > 0 {
                out.put_pixel(ox + x, oy + y, p);
            }
        }
    }
    out
}

/// Extract a character/head region from a white-background sheet into a 1024x1024 100% transparent RGBA icon!
fn extract_transparent_cutout(
    src: &RgbaImage,
    x0_norm: f32,
    y0_norm: f32,
    x1_norm: f32,
    y1_norm: f32,
    clear_top_corners: bool,
    strap_cut_y_norm: Option<f32>,
) -> RgbaImage {
    let (sw, sh) = src.dimensions();
    let x0 = ((x0_norm * sw as f32).round() as u32).min(sw - 1);
    let y0 = ((y0_norm * sh as f32).round() as u32).min(sh - 1);
    let x1 = ((x1_norm * sw as f32).round() as u32).clamp(x0 + 1, sw);
    let y1 = ((y1_norm * sh as f32).round() as u32).clamp(y0 + 1, sh);

    let cw = (x1 - x0) as usize;
    let ch = (y1 - y0) as usize;

    let mut crop = RgbaImage::new(cw as u32, ch as u32);
    for y in 0..ch {
        for x in 0..cw {
            let mut p = *src.get_pixel(x0 + x as u32, y0 + y as u32);
            let nx = x as f32 / cw as f32;
            let ny = y as f32 / ch as f32;

            if clear_top_corners && ny < 0.30 && (nx < 0.22 || nx > 0.78) {
                p = Rgba([255, 255, 255, 255]);
            }
            if let Some(cut_y) = strap_cut_y_norm {
                if ny > cut_y {
                    let r = p[0] as i32;
                    let g = p[1] as i32;
                    let b = p[2] as i32;
                    let yellowness = (r - b).max(g - b);
                    if yellowness < 55 || (ny > cut_y + 0.08 && (nx < 0.25 || nx > 0.75)) {
                        p = Rgba([255, 255, 255, 255]);
                    }
                }
            }
            crop.put_pixel(x as u32, y as u32, p);
        }
    }

    // 1. Exterior BFS Flood-Fill from all border pixels through white/light-grey background
    let mut visited = vec![false; cw * ch];
    let mut is_bg = vec![false; cw * ch];
    let mut queue = VecDeque::new();

    let is_bg_candidate = |r: u8, g: u8, b: u8| -> bool {
        let ri = r as i32;
        let gi = g as i32;
        let bi = b as i32;
        let yellowness = (ri - bi).max(gi - bi);
        let max_c = ri.max(gi).max(bi);
        let min_c = ri.min(gi).min(bi);
        let sat = max_c - min_c;
        (bi > 165 && ri > 165 && gi > 165 && yellowness < 48)
            || (bi > 110 && sat < 22 && yellowness < 18 && max_c > 115)
    };

    for x in 0..cw {
        for &y in &[0, ch - 1] {
            let idx = y * cw + x;
            visited[idx] = true;
            is_bg[idx] = true;
            queue.push_back((x, y));
        }
    }
    for y in 0..ch {
        for &x in &[0, cw - 1] {
            let idx = y * cw + x;
            if !visited[idx] {
                visited[idx] = true;
                is_bg[idx] = true;
                queue.push_back((x, y));
            }
        }
    }

    while let Some((x, y)) = queue.pop_front() {
        let dirs: [(isize, isize); 8] = [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
            (1, 1),
        ];
        for (dx, dy) in dirs {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && nx < cw as isize && ny >= 0 && ny < ch as isize {
                let ux = nx as usize;
                let uy = ny as usize;
                let nidx = uy * cw + ux;
                if !visited[nidx] {
                    visited[nidx] = true;
                    let p = crop.get_pixel(ux as u32, uy as u32);
                    if is_bg_candidate(p[0], p[1], p[2]) {
                        is_bg[nidx] = true;
                        queue.push_back((ux, uy));
                    }
                }
            }
        }
    }

    // 2. Compute distance to exterior background for soft fur-tip alpha matting (radius 5px)
    let mut dist_to_bg = vec![100u8; cw * ch];
    let mut d_queue = VecDeque::new();
    for i in 0..(cw * ch) {
        if is_bg[i] {
            dist_to_bg[i] = 0;
            d_queue.push_back((i % cw, i / cw));
        }
    }
    while let Some((x, y)) = d_queue.pop_front() {
        let d = dist_to_bg[y * cw + x];
        if d >= 6 {
            continue;
        }
        for (dx, dy) in [(-1isize, 0), (1, 0), (0, -1isize), (0, 1)] {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && nx < cw as isize && ny >= 0 && ny < ch as isize {
                let nidx = (ny as usize) * cw + (nx as usize);
                if dist_to_bg[nidx] > d + 1 {
                    dist_to_bg[nidx] = d + 1;
                    d_queue.push_back((nx as usize, ny as usize));
                }
            }
        }
    }

    // 3. Apply Fur & Sunglasses Alpha Matting + White Background Color Decontamination
    let mut min_x = cw;
    let mut min_y = ch;
    let mut max_x = 0usize;
    let mut max_y = 0usize;

    for y in 0..ch {
        for x in 0..cw {
            let idx = y * cw + x;
            let d = dist_to_bg[idx];
            let p = crop.get_pixel(x as u32, y as u32);
            let rf = p[0] as f32;
            let gf = p[1] as f32;
            let bf = p[2] as f32;

            let mut alpha = if d == 0 {
                let yellowness = (rf - bf).max(gf - bf);
                if yellowness > 24.0 {
                    ((yellowness - 24.0) / 55.0).clamp(0.0, 0.85)
                } else {
                    0.0
                }
            } else if d <= 4 {
                let yellowness = (rf - bf).max(gf - bf);
                let darkness = 242.0 - bf;
                let a_fur = ((yellowness - 16.0) / 50.0).clamp(0.0, 1.0);
                let a_dark = ((darkness - 22.0) / 65.0).clamp(0.0, 1.0);
                let a_dist = (d as f32 / 4.0).clamp(0.0, 1.0);
                (a_fur.max(a_dark) * 0.75 + a_dist * 0.25).clamp(0.0, 1.0)
            } else {
                1.0
            };

            if x < 3 || y < 3 || x + 3 >= cw || y + 3 >= ch {
                alpha = 0.0;
            }

            if alpha > 0.02 {
                let (ro, go, bo) = if alpha < 0.96 {
                    let w_bg = 1.0 - alpha;
                    let safe_a = alpha.max(0.25);
                    let r_clean = ((rf - w_bg * 252.0) / safe_a).clamp(0.0, 255.0);
                    let g_clean = ((gf - w_bg * 252.0) / safe_a).clamp(0.0, 255.0);
                    let b_clean = ((bf - w_bg * 252.0) / safe_a).clamp(0.0, 255.0);
                    (r_clean as u8, g_clean as u8, b_clean as u8)
                } else {
                    (p[0], p[1], p[2])
                };

                let a_u8 = (alpha * 255.0).round() as u8;
                crop.put_pixel(x as u32, y as u32, Rgba([ro, go, bo, a_u8]));
                if a_u8 > 25 {
                    if x < min_x {
                        min_x = x;
                    }
                    if x > max_x {
                        max_x = x;
                    }
                    if y < min_y {
                        min_y = y;
                    }
                    if y > max_y {
                        max_y = y;
                    }
                }
            } else {
                crop.put_pixel(x as u32, y as u32, Rgba([0, 0, 0, 0]));
            }
        }
    }

    center_on_1024(&crop, min_x, min_y, max_x, max_y)
}

/// Seamlessly place the upper head + sunglasses cutout (from white background) onto the standalone round mascot head (`round_head`)
/// by crossfading right across the yellow fur forehead bridge (y = 335..400, above the eyes!) so there is ZERO mouth overlap and a 100% round furry chin!
fn fuse_sunglasses_head_with_round_chin(top_head: &RgbaImage, round_head: &RgbaImage) -> RgbaImage {
    let mut combined = RgbaImage::new(1024, 1024);
    let mut min_x = 1024usize;
    let mut min_y = 1024usize;
    let mut max_x = 0usize;
    let mut max_y = 0usize;

    for y in 0..1024usize {
        // Shift round_head down by 48px so the eyes sit naturally right below the sunglasses
        let ry = ((y as isize) - 48).clamp(0, 1023) as u32;

        for x in 0..1024usize {
            let pt = *top_head.get_pixel(x as u32, y as u32);
            let pr = *round_head.get_pixel(x as u32, ry);

            // Keep black sunglasses frame & lenses 100% solid (never crossfaded), and crossfade yellow fur at y = 395..450
            let is_sunglasses_pixel =
                y < 445 && pt[3] > 160 && (pt[0] as i32 - pt[2] as i32).abs() < 42 && pt[2] < 135;
            let t_face = if is_sunglasses_pixel {
                0.0
            } else {
                ((y as f32 - 392.0) / 55.0).clamp(0.0, 1.0)
            };

            let out_p = if t_face <= 0.0 {
                pt
            } else if t_face >= 1.0 {
                pr
            } else {
                let at = (pt[3] as f32 / 255.0) * (1.0 - t_face);
                let ar = (pr[3] as f32 / 255.0) * t_face;
                let a_sum = (at + ar).clamp(0.0, 1.0);
                if a_sum > 0.01 {
                    let r = ((pt[0] as f32 * at + pr[0] as f32 * ar) / a_sum).clamp(0.0, 255.0) as u8;
                    let g = ((pt[1] as f32 * at + pr[1] as f32 * ar) / a_sum).clamp(0.0, 255.0) as u8;
                    let b = ((pt[2] as f32 * at + pr[2] as f32 * ar) / a_sum).clamp(0.0, 255.0) as u8;
                    let a = ((pt[3] as f32 * (1.0 - t_face) + pr[3] as f32 * t_face)).clamp(0.0, 255.0) as u8;
                    Rgba([r, g, b, a])
                } else {
                    Rgba([0, 0, 0, 0])
                }
            };

            if out_p[3] > 20 {
                if x < min_x {
                    min_x = x;
                }
                if x > max_x {
                    max_x = x;
                }
                if y < min_y {
                    min_y = y;
                }
                if y > max_y {
                    max_y = y;
                }
            }
            combined.put_pixel(x as u32, y as u32, out_p);
        }
    }

    center_on_1024(&combined, min_x, min_y, max_x, max_y)
}

/// Remove the outer white/grey/dark corners + outer drop-shadow outside a macOS Squircle icon so the outer background is 100% transparent!
fn make_squircle_icon_transparent(src: &RgbaImage) -> RgbaImage {
    let resized = DynamicImage::ImageRgba8(src.clone())
        .resize_exact(1024, 1024, FilterType::Lanczos3)
        .to_rgba8();
    let w = 1024usize;
    let h = 1024usize;

    let corner_p = resized.get_pixel(4, 4);
    let bg_r = corner_p[0] as i32;
    let bg_g = corner_p[1] as i32;
    let bg_b = corner_p[2] as i32;

    let mut visited = vec![false; w * h];
    let mut is_outer_bg = vec![false; w * h];
    let mut queue = VecDeque::new();

    for x in 0..w {
        for &y in &[0, h - 1] {
            let idx = y * w + x;
            visited[idx] = true;
            is_outer_bg[idx] = true;
            queue.push_back((x, y));
        }
    }
    for y in 0..h {
        for &x in &[0, w - 1] {
            let idx = y * w + x;
            if !visited[idx] {
                visited[idx] = true;
                is_outer_bg[idx] = true;
                queue.push_back((x, y));
            }
        }
    }

    while let Some((x, y)) = queue.pop_front() {
        let p_curr = resized.get_pixel(x as u32, y as u32);
        for (dx, dy) in [(-1isize, 0), (1, 0), (0, -1isize), (0, 1)] {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && nx < w as isize && ny >= 0 && ny < h as isize {
                let ux = nx as usize;
                let uy = ny as usize;
                let nidx = uy * w + ux;
                if !visited[nidx] {
                    let p = resized.get_pixel(ux as u32, uy as u32);
                    let dr_corner = (p[0] as i32 - bg_r).abs()
                        + (p[1] as i32 - bg_g).abs()
                        + (p[2] as i32 - bg_b).abs();
                    let dr_step = (p[0] as i32 - p_curr[0] as i32).abs()
                        + (p[1] as i32 - p_curr[1] as i32).abs()
                        + (p[2] as i32 - p_curr[2] as i32).abs();
                    let max_c = p[0].max(p[1]).max(p[2]) as i32;
                    let min_c = p[0].min(p[1]).min(p[2]) as i32;
                    let low_sat = (max_c - min_c) < 22;

                    if (dr_corner < 150 && dr_step < 24 && low_sat) || (max_c > 215 && low_sat) {
                        visited[nidx] = true;
                        is_outer_bg[nidx] = true;
                        queue.push_back((ux, uy));
                    }
                }
            }
        }
    }

    let mut left_x = 100usize;
    for x in 0..512 {
        if !is_outer_bg[512 * w + x] {
            left_x = x;
            break;
        }
    }
    let mut right_x = 924usize;
    for x in (512..w).rev() {
        if !is_outer_bg[512 * w + x] {
            right_x = x;
            break;
        }
    }
    let mut top_y = 100usize;
    for y in 0..512 {
        if !is_outer_bg[y * w + 512] {
            top_y = y;
            break;
        }
    }
    let mut bot_y = 924usize;
    for y in (512..h).rev() {
        if !is_outer_bg[y * w + 512] {
            bot_y = y;
            break;
        }
    }

    let cx = (left_x + right_x) as f32 * 0.5;
    let cy = (top_y + bot_y) as f32 * 0.5;
    let r_half = ((right_x - left_x).min(bot_y - top_y) as f32 * 0.5 - 4.0).clamp(360.0, 445.0);

    let mut out = RgbaImage::new(w as u32, h as u32);
    for y in 0..h {
        for x in 0..w {
            let idx = y * w + x;
            let p = resized.get_pixel(x as u32, y as u32);
            let ux = ((x as f32 - cx).abs() / r_half).max(0.0);
            let uy = ((y as f32 - cy).abs() / r_half).max(0.0);
            let squircle_d = (ux.powf(4.6) + uy.powf(4.6)).powf(1.0 / 4.6);
            let squircle_alpha = ((1.0 - squircle_d) * 90.0).clamp(0.0, 1.0);
            let flood_alpha = if is_outer_bg[idx] { 0.0 } else { 1.0 };
            let final_a = (squircle_alpha * flood_alpha * 255.0).round() as u8;

            if final_a > 0 {
                out.put_pixel(x as u32, y as u32, Rgba([p[0], p[1], p[2], final_a]));
            } else {
                out.put_pixel(x as u32, y as u32, Rgba([0, 0, 0, 0]));
            }
        }
    }

    out
}

fn main() {
    let base_dir = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let public_icons = base_dir.join("public/icons");
    let artifact_dir =
        Path::new("/Users/m4pro/.gemini/antigravity/brain/cef0d372-fa15-4e75-9160-bca14359f054");
    let sheet_path = base_dir.join("public/collabs_mascot.png");

    if let Ok(sheet_dyn) = image::open(&sheet_path) {
        let sheet = sheet_dyn.to_rgba8();

        // Extract standalone round heads first
        // 3. Bottom-Left Happy Mascot Head (100% transparent background, no box, full top tufts!)
        let happy_head = extract_transparent_cutout(&sheet, 0.225, 0.692, 0.395, 0.902, true, None);
        happy_head
            .save(public_icons.join("Transparant_3_Mascot_Kop_Happy.png"))
            .unwrap();

        // 4. Bottom-Right Playful Wink Mascot Head (100% transparent background, no box, full top tufts!)
        let wink_head = extract_transparent_cutout(&sheet, 0.562, 0.692, 0.728, 0.902, true, None);
        wink_head
            .save(public_icons.join("Transparant_4_Mascot_Kop_Knipoog.png"))
            .unwrap();

        // 5. Bottom-Center Curious Mascot Head (100% transparent background, no box, full top tufts!)
        let curious_head =
            extract_transparent_cutout(&sheet, 0.395, 0.692, 0.562, 0.902, true, None);
        curious_head
            .save(public_icons.join("Transparant_5_Mascot_Kop_Curious.png"))
            .unwrap();

        // 1. Center Mascot Head + Sunglasses fused with round furry chin (100% transparent background, round chin!)
        let raw_top_sunglasses =
            extract_transparent_cutout(&sheet, 0.31, 0.145, 0.66, 0.485, false, Some(0.76));
        let head_sunglasses = fuse_sunglasses_head_with_round_chin(&raw_top_sunglasses, &happy_head);
        head_sunglasses
            .save(public_icons.join("Transparant_1_Mascot_Kop_Zonnebril.png"))
            .unwrap();

        // 2. Center Full Mascot with Sunglasses + Vintage Camera (100% transparent background, no box!)
        let full_mascot =
            extract_transparent_cutout(&sheet, 0.30, 0.145, 0.665, 0.692, false, None);
        full_mascot
            .save(public_icons.join("Transparant_2_Mascot_Met_Camera_En_Zonnebril.png"))
            .unwrap();

        // 6. Top-Right 3/4 Mascot with Sunglasses & Camera (100% transparent background, no box!)
        let side_mascot =
            extract_transparent_cutout(&sheet, 0.695, 0.075, 0.925, 0.432, false, None);
        side_mascot
            .save(public_icons.join("Transparant_6_Mascot_3D_Schuin_Camera.png"))
            .unwrap();

        // 7. Winking Mascot Head + Sunglasses + Golden Star fused with round furry chin!
        let wink_star_src =
            artifact_dir.join("mascot_icon_3_playful_wink_branded_1791801564043.jpg");
        if let Ok(ws_dyn) = image::open(&wink_star_src) {
            let raw_wink_star =
                extract_transparent_cutout(&ws_dyn.to_rgba8(), 0.11, 0.07, 0.89, 0.675, false, None);
            let wink_star_round = fuse_sunglasses_head_with_round_chin(&raw_wink_star, &wink_head);
            wink_star_round
                .save(public_icons.join("Transparant_7_Mascot_Kop_Knipoog_Zonnebril_Ster.png"))
                .unwrap();
        }
    }

    // 8. Make all Squircle Icons 100% transparent outside their rounded squircle borders!
    let squircle_sources = [
        (
            "mascot_icon_1_classic_head_1791801543941.jpg",
            "Mascot_Icon_1_Classic_Head.png",
        ),
        (
            "mascot_icon_2_aperture_lens_1791801553347.jpg",
            "Mascot_Icon_2_Aperture_Lens.png",
        ),
        (
            "mascot_icon_3_playful_wink_branded_1791801564043.jpg",
            "Mascot_Icon_3_Playful_Wink_Collabs.png",
        ),
        (
            "mascot_icon_4_camera_portrait_1791801574912.jpg",
            "Mascot_Icon_4_Camera_Portrait.png",
        ),
        (
            "collabs_retouch_pro_icon_gold_lens_1791801299742.jpg",
            "Collabs_Retouch_Pro_Icon_1_Gold_Aperture.png",
        ),
        (
            "collabs_retouch_pro_icon_prism_portrait_1791801309749.jpg",
            "Collabs_Retouch_Pro_Icon_2_Prism_Sculpt.png",
        ),
        (
            "collabs_retouch_pro_icon_neural_iris_1791801320557.jpg",
            "Collabs_Retouch_Pro_Icon_3_Neural_Iris.png",
        ),
        (
            "collabs_retouch_pro_branded_badge_1791801392619.jpg",
            "Collabs_Retouch_Pro_Icon_4_3D_Badge.png",
        ),
        (
            "collabs_retouch_pro_dock_icon_1791801414033.jpg",
            "Collabs_Retouch_Pro_Icon_5_Branded_Dock.png",
        ),
    ];

    for (src_file, dst_file) in squircle_sources {
        let p_in = artifact_dir.join(src_file);
        let p_out = public_icons.join(dst_file);
        if let Ok(img) = image::open(&p_in) {
            let sq = make_squircle_icon_transparent(&img.to_rgba8());
            sq.save(&p_out).unwrap();
        }
    }

    println!("All transparent cutout icons & squircle icons generated successfully!");
}
