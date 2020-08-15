import os
import zipfile


rootdir = os.path.dirname(__file__)
archivename = os.path.join(rootdir, 'stuff.zip')

files = [
    'a.txt',
    'b.txt',
]

print(rootdir)
print(archivename)


with zipfile.ZipFile(archivename, 'w') as zipf:
    for fname in files:
        # fpath = os.path.join(rootdir, fname)
        # zipf.write(fpath)
        zipf.write(fname)