import os
files = os.listdir()
for item in files:
	spaceless = item.replace(' ', '')
	os.rename(item, spaceless)

