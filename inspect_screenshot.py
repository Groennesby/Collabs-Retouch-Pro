from PIL import Image
import numpy as np

im = Image.open('/home/dave/Applications/Collabs Retouch Pro/splashscreens/Screenshot 2026-10-12 at 15.32.00.png')
print("Image format:", im.format, "size:", im.size, "mode:", im.mode)

arr = np.array(im.convert('RGB'))
print("Min RGB:", arr.min(axis=(0,1)))
print("Max RGB:", arr.max(axis=(0,1)))

# Let's inspect background color (corners)
print("Top-left corner:", arr[5, 5])
print("Bottom-right corner:", arr[-5, -5])
