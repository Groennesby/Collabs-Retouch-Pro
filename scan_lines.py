from PIL import Image
import numpy as np

crop = Image.open('/home/dave/Applications/Collabs Retouch Pro/splashscreens/text_under_icon.png').convert('RGB')
arr = np.array(crop)

# Let's inspect horizontal line brightness across Y in the crop
for y in range(0, 160, 5):
    line = arr[y, :, :]
    bright = (line[:, 0] > 100) | (line[:, 1] > 100) | (line[:, 2] > 100)
    count = np.sum(bright)
    if count > 20:
        avg_color = np.mean(line[bright], axis=0).astype(int)
        print(f"Y={y} (splash Y={520+y}): {count} bright pixels, avg RGB={avg_color}")
