from PIL import Image
import numpy as np
import subprocess
import os
import re

src_path = '/home/dave/Applications/Collabs Retouch Pro/splashscreens/Screenshot 2026-10-12 at 15.32.00.png'
im = Image.open(src_path).convert('RGBA')
arr = np.array(im, dtype=float)

# Background is ~[4, 4, 7]
# Letters are bright white/gray with antialiased edges
r = arr[:, :, 0]
g = arr[:, :, 1]
b = arr[:, :, 2]
lum = 0.299 * r + 0.587 * g + 0.114 * b

# Baseline background luminance is < 15
# White letters reach up to 255
alpha = np.clip((lum - 10.0) / (180.0 - 10.0), 0.0, 1.0)

# Upscale 4x with Lanczos for sub-pixel precision vector tracing and 4K PNG export
w, h = im.size
up_w, up_h = w * 4, h * 4

alpha_img = Image.fromarray((alpha * 255).astype(np.uint8))
alpha_upscaled = alpha_img.resize((up_w, up_h), Image.Resampling.LANCZOS)
alpha_up_arr = np.array(alpha_upscaled)

# -------------------------------------------------------------
# 1. GENERATE HIGH-RES TRANSPARENT PNGs (FOR PHOTOSHOP)
# -------------------------------------------------------------
out_dir = '/home/dave/Applications/Collabs Retouch Pro/splashscreens'
git_dir = '/home/dave/Collabs GIT Software/collabs-retouch-pro/splashscreens'

# White Transparent PNG
png_white = np.zeros((up_h, up_w, 4), dtype=np.uint8)
png_white[:, :, :3] = 255
png_white[:, :, 3] = alpha_up_arr
Image.fromarray(png_white).save(os.path.join(out_dir, 'Retouch_Pro_Exact_White_Transparent.png'))
Image.fromarray(png_white).save(os.path.join(git_dir, 'Retouch_Pro_Exact_White_Transparent.png'))

# Black Transparent PNG
png_black = np.zeros((up_h, up_w, 4), dtype=np.uint8)
png_black[:, :, 0] = 18
png_black[:, :, 1] = 18
png_black[:, :, 2] = 20
png_black[:, :, 3] = alpha_up_arr
Image.fromarray(png_black).save(os.path.join(out_dir, 'Retouch_Pro_Exact_Black_Transparent.png'))
Image.fromarray(png_black).save(os.path.join(git_dir, 'Retouch_Pro_Exact_Black_Transparent.png'))

# Gold Transparent PNG
png_gold = np.zeros((up_h, up_w, 4), dtype=np.uint8)
png_gold[:, :, 0] = 255
png_gold[:, :, 1] = 230
png_gold[:, :, 2] = 6
png_gold[:, :, 3] = alpha_up_arr
Image.fromarray(png_gold).save(os.path.join(out_dir, 'Retouch_Pro_Exact_Gold_Transparent.png'))
Image.fromarray(png_gold).save(os.path.join(git_dir, 'Retouch_Pro_Exact_Gold_Transparent.png'))

# Native resolution transparent PNG (exact 588 x 108)
native_alpha = (alpha * 255).astype(np.uint8)
native_white = np.zeros((h, w, 4), dtype=np.uint8)
native_white[:, :, :3] = 255
native_white[:, :, 3] = native_alpha
Image.fromarray(native_white).save(os.path.join(out_dir, 'Retouch_Pro_Exact_Native_Transparent.png'))
Image.fromarray(native_white).save(os.path.join(git_dir, 'Retouch_Pro_Exact_Native_Transparent.png'))

# -------------------------------------------------------------
# 2. GENERATE CLEAN SVG VECTORS (POTRACE)
# -------------------------------------------------------------
# For potrace: black (0) shapes on white (255) background
# We flip vertically so the generated SVG has standard top-to-bottom orientation
mask_up = alpha_up_arr > 90
bmp_arr = np.where(mask_up, 0, 255).astype(np.uint8)
bmp_flipped = np.flipud(bmp_arr)

bmp_path = '/tmp/screenshot_letters.bmp'
Image.fromarray(bmp_flipped).save(bmp_path)

svg_raw_path = '/tmp/screenshot_letters_raw.svg'
subprocess.run([
    'potrace',
    '-s',
    '--alphamax', '1.0',
    '--turdsize', '4',
    '--opttolerance', '0.2',
    bmp_path,
    '-o', svg_raw_path
])

with open(svg_raw_path, 'r') as f:
    svg_raw = f.read()

# Extract paths
path_matches = re.findall(r'<path[^>]+d="([^"]+)"', svg_raw)
all_paths = " ".join(path_matches)

viewbox_match = re.search(r'viewBox="([^"]+)"', svg_raw)
viewbox = viewbox_match.group(1) if viewbox_match else f"0 0 {up_w} {up_h}"

def create_svg(fill_color):
    return f'''<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="{viewbox}" width="100%" height="100%">
  <!-- Retouch Pro - Exact Vector Traced from Screenshot -->
  <path d="{all_paths}" fill="{fill_color}" fill-rule="evenodd" />
</svg>'''

# Save White, Black, and Gold SVG vectors
with open(os.path.join(out_dir, 'Retouch_Pro_Exact_Vector_White.svg'), 'w') as f:
    f.write(create_svg('#FFFFFF'))
with open(os.path.join(out_dir, 'Retouch_Pro_Exact_Vector_Black.svg'), 'w') as f:
    f.write(create_svg('#121214'))
with open(os.path.join(out_dir, 'Retouch_Pro_Exact_Vector_Gold.svg'), 'w') as f:
    f.write(create_svg('#FFE606'))

# Main default vector file
with open(os.path.join(out_dir, 'Retouch_Pro_Exact_Vector.svg'), 'w') as f:
    f.write(create_svg('#FFFFFF'))

# Mirror to git repo
with open(os.path.join(git_dir, 'Retouch_Pro_Exact_Vector_White.svg'), 'w') as f:
    f.write(create_svg('#FFFFFF'))
with open(os.path.join(git_dir, 'Retouch_Pro_Exact_Vector_Black.svg'), 'w') as f:
    f.write(create_svg('#121214'))
with open(os.path.join(git_dir, 'Retouch_Pro_Exact_Vector_Gold.svg'), 'w') as f:
    f.write(create_svg('#FFE606'))
with open(os.path.join(git_dir, 'Retouch_Pro_Exact_Vector.svg'), 'w') as f:
    f.write(create_svg('#FFFFFF'))

print("All exact PNG and SVG assets created successfully!")
