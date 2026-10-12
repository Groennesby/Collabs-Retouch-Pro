from PIL import Image, ImageFilter
import numpy as np

# Load test_composite_splash.png
splash = Image.open('/home/dave/Applications/Collabs Retouch Pro/splashscreens/test_composite_splash.png').convert('RGBA')

# The background color in the text column (X: 100..650) is very dark: ~[4, 4, 7]
# Line 1: 'COLLABS' is at Y: 640..675, X: 130..610
# Line 2: 'RETOUCH PRO' is at Y: 720..760, X: 130..610
# Line 3: 'AI Editorial & Portrait Studio' is at Y: 765..795
# Line 4: 'Initializing Neural Engine...' is at Y: 830..860

# Version A: Cleanly remove 'COLLABS' from Y: 635..685 by inpainting with background
splash_a = splash.copy()
# Create feathered mask for the COLLABS text strip:
mask_strip = Image.new('L', splash.size, 0)
from PIL import ImageDraw
draw = ImageDraw.Draw(mask_strip)
draw.rectangle([110, 635, 620, 680], fill=255)
mask_strip = mask_strip.filter(ImageFilter.GaussianBlur(8))

bg_fill = Image.new('RGBA', splash.size, (4, 4, 7, 255))
splash_a = Image.composite(bg_fill, splash_a, mask_strip)

# Version B: Remove COLLABS and reposition RETOUCH PRO + Subtitle up by 70px to sit right under the RP icon
splash_b = splash_a.copy()
# Crop 'RETOUCH PRO' + 'AI Editorial & Portrait Studio' from splash (Y: 715..805, X: 120..620)
text_block = splash.crop((120, 715, 620, 805))
# Clear the old position on splash_b
mask_old = Image.new('L', splash.size, 0)
draw_old = ImageDraw.Draw(mask_old)
draw_old.rectangle([115, 710, 625, 810], fill=255)
mask_old = mask_old.filter(ImageFilter.GaussianBlur(8))
splash_b = Image.composite(bg_fill, splash_b, mask_old)
# Paste text_block up at Y: 560
splash_b.paste(text_block, (120, 560), mask=text_block.split()[3])

splash_a.save('/home/dave/Applications/Collabs Retouch Pro/splashscreens/splash_no_collabs_a.png')
splash_b.save('/home/dave/Applications/Collabs Retouch Pro/splashscreens/splash_no_collabs_b.png')
print("Both versions saved.")
