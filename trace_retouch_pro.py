from PIL import Image, ImageFilter, ImageOps
import numpy as np
import subprocess
import os

crop = Image.open('/home/dave/Applications/Collabs Retouch Pro/splashscreens/retouch_pro_tight_crop.png').convert('L')

# Upscale 4x with Lanczos for maximum curve precision
w, h = crop.size
up_w, up_h = w * 4, h * 4
upscaled = crop.resize((up_w, up_h), Image.Resampling.LANCZOS)

# The background is very dark (~5..15), the text letters are bright (~150..255)
# Threshold around 90
arr = np.array(upscaled)
mask = arr > 85

# Potrace traces black (0) shapes on white (255) background
# So make letters 0 (black), background 255 (white)
trace_bmp = np.where(mask, 0, 255).astype(np.uint8)
bmp_img = Image.fromarray(trace_bmp)
bmp_path = '/tmp/retouch_pro_letters.bmp'
bmp_img.save(bmp_path)

# Run potrace with high precision smoothing (-a 1.0 -t 2 -k 0.2)
svg_raw_path = '/tmp/retouch_pro_letters_raw.svg'
subprocess.run([
    'potrace', 
    '-s', 
    '--alphamax', '1.0',
    '--turdsize', '5',
    '--opttolerance', '0.2',
    bmp_path, 
    '-o', svg_raw_path
])

with open(svg_raw_path, 'r') as f:
    svg_content = f.read()

import re
# Extract the path d="..." from potrace SVG
path_matches = re.findall(r'<path[^>]+d="([^"]+)"', svg_content)
all_paths = " ".join(path_matches)

# Extract viewBox from potrace svg
viewbox_match = re.search(r'viewBox="([^"]+)"', svg_content)
viewbox = viewbox_match.group(1) if viewbox_match else f"0 0 {up_w} {up_h}"
width_match = re.search(r'width="([^"]+)"', svg_content)
height_match = re.search(r'height="([^"]+)"', svg_content)

# Clean, production-ready SVG (White letters for dark backgrounds, e.g. Photoshop splash)
svg_white = f'''<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="{viewbox}" width="100%" height="100%">
  <!-- Retouch Pro Vector Letters - White Fill (for dark backgrounds) -->
  <g transform="translate(0, {up_h}) scale(1, -1)">
    <path d="{all_paths}" fill="#FFFFFF" fill-rule="evenodd" />
  </g>
</svg>'''

# Black letters version (for light backgrounds / editorial print)
svg_black = f'''<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="{viewbox}" width="100%" height="100%">
  <!-- Retouch Pro Vector Letters - Black Fill (for light backgrounds) -->
  <g transform="translate(0, {up_h}) scale(1, -1)">
    <path d="{all_paths}" fill="#121214" fill-rule="evenodd" />
  </g>
</svg>'''

# Gold letters version (matching Collabs Gold #FFE606)
svg_gold = f'''<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="{viewbox}" width="100%" height="100%">
  <!-- Retouch Pro Vector Letters - Collabs Gold Fill -->
  <g transform="translate(0, {up_h}) scale(1, -1)">
    <path d="{all_paths}" fill="#FFE606" fill-rule="evenodd" />
  </g>
</svg>'''

# Save to splashscreens folder in Applications
out_dir = '/home/dave/Applications/Collabs Retouch Pro/splashscreens'
with open(os.path.join(out_dir, 'Retouch_Pro_Letters_White.svg'), 'w') as f:
    f.write(svg_white)

with open(os.path.join(out_dir, 'Retouch_Pro_Letters_Black.svg'), 'w') as f:
    f.write(svg_black)

with open(os.path.join(out_dir, 'Retouch_Pro_Letters_Gold.svg'), 'w') as f:
    f.write(svg_gold)

# Default main SVG
with open(os.path.join(out_dir, 'Retouch_Pro_Letters_Vector.svg'), 'w') as f:
    f.write(svg_white)

# Also save to git repo
git_out_dir = '/home/dave/Collabs GIT Software/collabs-retouch-pro/splashscreens'
with open(os.path.join(git_out_dir, 'Retouch_Pro_Letters_White.svg'), 'w') as f:
    f.write(svg_white)
with open(os.path.join(git_out_dir, 'Retouch_Pro_Letters_Black.svg'), 'w') as f:
    f.write(svg_black)
with open(os.path.join(git_out_dir, 'Retouch_Pro_Letters_Gold.svg'), 'w') as f:
    f.write(svg_gold)
with open(os.path.join(git_out_dir, 'Retouch_Pro_Letters_Vector.svg'), 'w') as f:
    f.write(svg_white)

print("Vector files generated successfully in", out_dir)
