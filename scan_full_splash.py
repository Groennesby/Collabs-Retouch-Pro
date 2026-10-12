from PIL import Image
import numpy as np

splash = Image.open('/home/dave/Applications/Collabs Retouch Pro/splashscreens/test_composite_splash.png').convert('RGB')
arr = np.array(splash)

for y in range(480, 850, 10):
    line = arr[y, 100:600, :]
    bright = (line[:, 0] > 80) | (line[:, 1] > 80) | (line[:, 2] > 80)
    count = np.sum(bright)
    if count > 20:
        avg_color = np.mean(line[bright], axis=0).astype(int)
        print(f"Splash Y={y}: {count} pixels, avg RGB={avg_color}")
