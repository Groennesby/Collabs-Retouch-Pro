from PIL import Image, ImageFilter, ImageEnhance
import numpy as np

im = Image.open('/home/dave/Applications/Collabs Retouch Pro/splashscreens/Collabs_Retouch_Pro_RP_Icon.png')
arr = np.array(im)

# Let's inspect the yellow channel contrast by boosting contrast of the G/R channels:
# The yellow background is mostly R~255, G~230..245, B~0..15.
# Let's normalize the G channel in the yellow area (where arr[:, :, 3] > 100 and not dark):
is_dark = (arr[:, :, 0] < 80) & (arr[:, :, 1] < 80) & (arr[:, :, 2] < 80)
valid = (arr[:, :, 3] > 100) & ~is_dark

# Normalize G channel variation
g = arr[:, :, 1].astype(float)
g_valid = g[valid]
g_min, g_max = np.percentile(g_valid, 2), np.percentile(g_valid, 98)
print("G range in background:", g_min, g_max)

contrast_map = np.zeros(g.shape, dtype=np.uint8)
contrast_map[valid] = np.clip((g[valid] - g_min) / (g_max - g_min) * 255.0, 0, 255).astype(np.uint8)

# Save as visual map
contrast_img = Image.fromarray(contrast_map)
contrast_img.save('/home/dave/Applications/Collabs Retouch Pro/splashscreens/icon_bg_contrast_map.png')
print("Saved icon_bg_contrast_map.png")
