from PIL import Image
import numpy as np

crop = Image.open('/home/dave/Applications/Collabs Retouch Pro/splashscreens/text_under_icon.png').convert('RGB')
arr = np.array(crop)

# In text_under_icon (which was cropped from (100, 520, 650, 680)):
# Find gold/yellow text for 'COLLABS':
# R > 180, G > 140, B < 50
r = arr[:, :, 0]
g = arr[:, :, 1]
b = arr[:, :, 2]

collabs_mask = (r > 150) & (g > 120) & (b < 60)
coords = np.argwhere(collabs_mask)
if len(coords) > 0:
    ymin, xmin = coords.min(axis=0)
    ymax, xmax = coords.max(axis=0)
    print(f"COLLABS in crop: Y: [{ymin}, {ymax}], X: [{xmin}, {xmax}]")
    print(f"In full splash coordinates: Y: [{520 + ymin}, {520 + ymax}], X: [{100 + xmin}, {100 + xmax}]")
else:
    print("COLLABS not found by color")
