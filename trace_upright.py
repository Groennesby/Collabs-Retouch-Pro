from PIL import Image
import numpy as np
import subprocess
import os
import re

crop = Image.open('/home/dave/Applications/Collabs Retouch Pro/splashscreens/retouch_pro_tight_crop.png').convert('L')
w, h = crop.size
up_w, up_h = w * 4, h * 4
upscaled = crop.resize((up_w, up_h), Image.Resampling.LANCZOS)
arr = np.array(upscaled)
mask = arr > 85

# Invert vertically before potrace so that potrace's coordinate system matches standard top-left SVG!
# Or don't invert and remove the negative scale:
trace_bmp = np.where(mask, 0, 255).astype(np.uint8)
# Flip vertically so Potrace produces upright paths:
trace_bmp_flipped = np.flipud(trace_bmp)
bmp_img = Image.fromarray(trace_bmp_flipped)
bmp_path = '/tmp/retouch_pro_letters_upright.bmp'
bmp_img.save(bmp_path)

svg_raw_path = '/tmp/retouch_pro_letters_upright.svg'
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

# Extract paths and viewBox
path_matches = re.findall(r'<path[^>]+d="([^"]+)"', svg_content)
all_paths = " ".join(path_matches)
viewbox_match = re.search(r'viewBox="([^"]+)"', svg_content)
viewbox = viewbox_match.group(1) if viewbox_match else f"0 0 {up_w} {up_h}"

# In potrace, flipping the input bitmap makes scale(1, 1) or scale(1, -1) upright
# Let's test standard SVG without any flipping transforms:
def make_svg(fill_color):
    return f'''<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="{viewbox}" width="100%" height="100%">
  <path d="{all_paths}" fill="{fill_color}" fill-rule="evenodd" />
</svg>'''

out_dir = '/home/dave/Applications/Collabs Retouch Pro/splashscreens'
with open(os.path.join(out_dir, 'Retouch_Pro_Letters_White.svg'), 'w') as f:
    f.write(make_svg('#FFFFFF'))

with open(os.path.join(out_dir, 'Retouch_Pro_Letters_Black.svg'), 'w') as f:
    f.write(make_svg('#121214'))

with open(os.path.join(out_dir, 'Retouch_Pro_Letters_Gold.svg'), 'w') as f:
    f.write(make_svg('#FFE606'))

with open(os.path.join(out_dir, 'Retouch_Pro_Letters_Vector.svg'), 'w') as f:
    f.write(make_svg('#FFFFFF'))

# Also update git repo
git_out_dir = '/home/dave/Collabs GIT Software/collabs-retouch-pro/splashscreens'
with open(os.path.join(git_out_dir, 'Retouch_Pro_Letters_White.svg'), 'w') as f:
    f.write(make_svg('#FFFFFF'))
with open(os.path.join(git_out_dir, 'Retouch_Pro_Letters_Black.svg'), 'w') as f:
    f.write(make_svg('#121214'))
with open(os.path.join(git_out_dir, 'Retouch_Pro_Letters_Gold.svg'), 'w') as f:
    f.write(make_svg('#FFE606'))
with open(os.path.join(git_out_dir, 'Retouch_Pro_Letters_Vector.svg'), 'w') as f:
    f.write(make_svg('#FFFFFF'))

print("Updated upright SVGs generated.")
