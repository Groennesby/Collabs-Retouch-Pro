from PIL import Image
import numpy as np

im = Image.open('/home/dave/Applications/Collabs Retouch Pro/splashscreens/Collabs_Retouch_Pro_RP_Icon.png')
arr = np.array(im)

# Let's inspect where dark pixels exist in the icon:
# Black text 'RP' is in the middle.
# Where else are there dark pixels (r < 100, g < 100, b < 100)?
y_indices, x_indices = np.ogrid[:arr.shape[0], :arr.shape[1]]
is_dark = (arr[:, :, 0] < 80) & (arr[:, :, 1] < 80) & (arr[:, :, 2] < 80) & (arr[:, :, 3] > 100)

print("Total dark pixels:", np.sum(is_dark))
# Find connected components or bounding boxes of dark regions:
# Y < 340 (above RP):
above_rp = is_dark & (y_indices < 340)
print("Dark pixels above RP (Y < 340):", np.sum(above_rp))
if np.sum(above_rp) > 0:
    coords = np.argwhere(above_rp)
    print("Above RP bbox:", coords[:, 0].min(), coords[:, 0].max(), coords[:, 1].min(), coords[:, 1].max())

# Below Retouch Pro (Y > 800):
below = is_dark & (y_indices > 800)
print("Dark pixels below (Y > 800):", np.sum(below))
if np.sum(below) > 0:
    coords = np.argwhere(below)
    print("Below bbox:", coords[:, 0].min(), coords[:, 0].max(), coords[:, 1].min(), coords[:, 1].max())

# Between Y 730 and Y 800 (where 'Retouch pro' might be):
mid_lower = is_dark & (y_indices >= 730) & (y_indices <= 800)
print("Dark pixels at Y 730..800:", np.sum(mid_lower))

# What about subtle text (not pure black, but darker yellow / watermark)?
# Let's check G channel variation in the yellow background:
yellow_mask = (arr[:, :, 3] > 100) & ~is_dark
# Let's see if there is text in the yellow background:
# High-pass filter on G channel:
from PIL import ImageFilter
g_img = Image.fromarray(arr[:, :, 1])
# unsharp / edge detect
edges = g_img.filter(ImageFilter.FIND_EDGES)
edges_arr = np.array(edges)
# In areas outside RP:
edges_arr[is_dark] = 0
print("Edges in background:", np.max(edges_arr))
