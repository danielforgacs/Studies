FILENAME = 'photo.jpg'

with open (FILENAME, 'br') as binf:
    data = binf.read()

print(data)
