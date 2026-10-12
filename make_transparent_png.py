from PIL import Image
import numpy as np
import os

crop = Image.open('/home/dave/Applications/Collabs Retouch Pro/splashscreens/retouch_pro_tight_crop.png').convert('RGB')
arr = np.array(crop, dtype=float)

# Background is ~[4, 4, 7]
# The text 'RETOUCH PRO' is bright light gray/white ~[170, 175, 185] up to [240, 240, 240]
# Let's extract clean luminance
r = arr[:, :, 0]
g = arr[:, :, 1]
b = arr[:, :, 2]
lum = 0.299 * r + 0.587 * g + 0.114 * b

# Background baseline is < 25
# Letters are > 50
# Clean anti-aliased alpha mask:
alpha_norm = np.clip((lum - 20.0) / (180.0 - 20.0), 0.0, 1.0)

# Tight crop to content bounds horizontally and vertically:
y_valid, x_valid = np.where(alpha_norm > 0.1)
ymin, ymax = y_valid.min(), y_valid.max()
xmin, xmax = x_valid.min(), x_valid.max()

# Crop alpha
cropped_alpha = alpha_norm[ymin-2:ymax+3, xmin-2:xmax+3]
h, w = cropped_alpha.shape

# Upscale 4x with bicubic/lanczos for ultra-crisp resolution in Photoshop
alpha_img = Image.fromarray((cropped_alpha * 255).astype(np.uint8))
target_w = w * 4
target_h = h * 4
alpha_highres = alpha_img.resize((target_w, target_h), Image.Resampling.LANCZOS)
alpha_highres_arr = np.array(alpha_highres)

out_dir = '/home/dave/Applications/Collabs Retouch Pro/splashscreens'
git_out_dir = '/home/dave/Collabs GIT Software/collabs-retouch-pro/splashscreens'

# 1. White Letters (Transparent PNG)
white_png = np.zeros((target_h, target_w, 4), dtype=np.uint8)
white_png[:, :, 0] = 255
white_png[:, :, 1] = 255
white_png[:, :, 2] = 255
white_png[:, :, 3] = alpha_highres_arr
img_white = Image.fromarray(white_png)
img_white.save(os.path.join(out_dir, 'Retouch_Pro_Letters_White_Transparent.png'))
img_white.save(os.path.join(git_out_dir, 'Retouch_Pro_Letters_White_Transparent.png'))

# 2. Black Letters (Transparent PNG)
black_png = np.zeros((target_h, target_w, 4), dtype=np.uint8)
black_png[:, :, 0] = 18
black_png[:, :, 1] = 18
black_png[:, :, 2] = 20
black_png[:, :, 3] = alpha_highres_arr
img_black = Image.fromarray(black_png)
img_black.save(os.path.join(out_dir, 'Retouch_Pro_Letters_Black_Transparent.png'))
img_black.save(os.path.join(git_out_dir, 'Retouch_Pro_Letters_Black_Transparent.png'))

# 3. Gold Letters (Transparent PNG, Collabs Gold #FFE606)
gold_png = np.zeros((target_h, target_w, 4), dtype=np.uint8)
gold_png[:, :, 0] = 255
gold_png[:, :, 1] = 230
gold_png[:, :, 2] = 6
gold_png[:, :, 3] = alpha_highres_arr
img_gold = Image.fromarray(gold_png)
img_gold.save(os.path.join(out_dir, 'Retouch_Pro_Letters_Gold_Transparent.png'))
img_gold.save(os.path.join(git_out_dir, 'Retouch_Pro_Letters_Gold_Transparent.png'))

# Default main PNG
img_white.save(os.path.join(out_dir, 'Retouch_Pro_Letters_Transparent.png'))
img_white.save(os.path.join(git_out_dir, 'Retouch_Pro_Letters_Transparent.png'))

print("High-res transparent PNGs generated successfully!")
print("Dimensions:", target_w, "x", target_h)
