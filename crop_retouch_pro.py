from PIL import Image
import numpy as np

splash = Image.open('/home/dave/Applications/Collabs Retouch Pro/splashscreens/test_composite_splash.png').convert('RGB')
arr = np.array(splash)

# On test_composite_splash:
# 'RETOUCH PRO' is located at Y: ~715..765, X: ~120..620
# Let's crop tightly around RETOUCH PRO:
crop = splash.crop((120, 715, 620, 765))
crop.save('/home/dave/Applications/Collabs Retouch Pro/splashscreens/retouch_pro_tight_crop.png')
print("Tight crop saved. Size:", crop.size)
