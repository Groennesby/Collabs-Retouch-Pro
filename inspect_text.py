from PIL import Image
import numpy as np

splash = Image.open('/home/dave/Applications/Collabs Retouch Pro/splashscreens/test_composite_splash.png')
print("Splash size:", splash.size)

# Let's inspect the text region under the icon:
# Icon was placed at X: 135, Y: 170..490
# Below that, around Y: 520..650, X: 100..600 is the text.
crop_text = splash.crop((100, 520, 650, 680))
crop_text.save('/home/dave/Applications/Collabs Retouch Pro/splashscreens/text_under_icon.png')
print("Cropped text under icon saved to text_under_icon.png")
