from PIL import Image
import numpy as np
import subprocess
import os

# 1. Load icon
icon = Image.open('/home/dave/Applications/Collabs Retouch Pro/splashscreens/Collabs_Retouch_Pro_RP_Icon.png').convert('RGBA')
arr = np.array(icon)

# Extract black letters mask (RP)
# Black letters are where R < 50, G < 50, B < 50, A > 200
r = arr[:, :, 0]
g = arr[:, :, 1]
b = arr[:, :, 2]
a = arr[:, :, 3]

rp_mask = (r < 50) & (g < 50) & (b < 50) & (a > 150)
rp_bmp = Image.fromarray((~rp_mask).astype(np.uint8) * 255) # Potrace traces black (0) on white (255)
rp_bmp.save('/tmp/rp_letters.bmp')

# Run potrace on RP letters to get SVG path
subprocess.run(['potrace', '-s', '/tmp/rp_letters.bmp', '-o', '/tmp/rp_letters.svg'])

# Also trace the badge outer boundary (squircle)
badge_mask = a > 50
badge_bmp = Image.fromarray((~badge_mask).astype(np.uint8) * 255)
badge_bmp.save('/tmp/badge_outline.bmp')
subprocess.run(['potrace', '-s', '/tmp/badge_outline.bmp', '-o', '/tmp/badge_outline.svg'])

# Read SVG paths
with open('/tmp/rp_letters.svg', 'r') as f:
    rp_svg = f.read()

with open('/tmp/badge_outline.svg', 'r') as f:
    badge_svg = f.read()

# Extract path from rp_svg
import re
rp_path_match = re.search(r'<path[^>]+d="([^"]+)"', rp_svg)
badge_path_match = re.search(r'<path[^>]+d="([^"]+)"', badge_svg)

rp_d = rp_path_match.group(1) if rp_path_match else ""
badge_d = badge_path_match.group(1) if badge_path_match else ""

# Create combined, layered SVG with vibrant gold background and deep obsidian black RP lettering
combined_svg = f'''<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024" width="1024" height="1024">
  <defs>
    <linearGradient id="goldGradient" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#FFF04D" />
      <stop offset="50%" stop-color="#FFE606" />
      <stop offset="100%" stop-color="#F5C518" />
    </linearGradient>
  </defs>

  <!-- Squircle Gold Badge -->
  <g transform="translate(0, 1024) scale(1, -1)">
    <!-- Badge Background -->
    <rect x="64" y="64" width="896" height="896" rx="220" ry="220" fill="url(#goldGradient)" stroke="#FFE606" stroke-width="8" />
    
    <!-- Inner subtle border -->
    <rect x="80" y="80" width="864" height="864" rx="205" ry="205" fill="none" stroke="rgba(255,255,255,0.4)" stroke-width="4" />

    <!-- Pure Black RP Letters (Vector Traced) -->
    <path d="{rp_d}" fill="#121214" />
  </g>
</svg>'''

out_svg = '/home/dave/Applications/Collabs Retouch Pro/splashscreens/Collabs_Retouch_Pro_RP_Icon.svg'
with open(out_svg, 'w') as f:
    f.write(combined_svg)

# Also save standalone letters SVG for Photoshop drag-and-drop
standalone_letters = f'''<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024" width="1024" height="1024">
  <g transform="translate(0, 1024) scale(1, -1)">
    <path d="{rp_d}" fill="#121214" />
  </g>
</svg>'''
with open('/home/dave/Applications/Collabs Retouch Pro/splashscreens/RP_Letters_Vector_Only.svg', 'w') as f:
    f.write(standalone_letters)

print("SVG vectors generated successfully at:", out_svg)
