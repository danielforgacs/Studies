import os
import zipfile

filenames = [
    'a.txt',
    'b.txt',
]

rootdir = os.path.dirname(__file__)
archivename = os.path.join(rootdir, 'stuff.zip')

with zipfile.ZipFile(archivename, 'w') as zipf:
    for fname in filenames:
        zipf.write('download/{}'.format(fname))