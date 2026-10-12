from PIL import Image

splash = Image.open('/home/dave/Applications/Collabs Retouch Pro/splashscreens/test_composite_splash.png')
crop = splash.crop((100, 630, 650, 870))
crop.save('/home/dave/Applications/Collabs Retouch Pro/splashscreens/full_text_area.png')
print("full_text_area.png saved")
