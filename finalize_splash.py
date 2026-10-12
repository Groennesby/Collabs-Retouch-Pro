from PIL import Image

splash = Image.open('/home/dave/Applications/Collabs Retouch Pro/splashscreens/splash_no_collabs_a.png')

# Save to Applications
splash.save('/home/dave/Applications/Collabs Retouch Pro/splashscreens/Collabs_Retouch_Pro_Splash_1_Editorial_Studio.png')
splash.convert('RGB').save('/home/dave/Applications/Collabs Retouch Pro/splashscreens/Collabs_Retouch_Pro_Splash_1_Editorial_Studio.jpg', quality=98)

# Save to Collabs GIT Software repo
splash.save('/home/dave/Collabs GIT Software/collabs-retouch-pro/splashscreens/Collabs_Retouch_Pro_Splash_1_Editorial_Studio.png')
splash.convert('RGB').save('/home/dave/Collabs GIT Software/collabs-retouch-pro/splashscreens/Collabs_Retouch_Pro_Splash_1_Editorial_Studio.jpg', quality=98)

print("Active splash screens updated successfully with RP icon and RETOUCH PRO (no Collabs)!")
