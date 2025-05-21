import pathlib

# U+00A9: ©, UTF-8 (hex)	0xC2 0xA9


data = bytes([0xC2, 0xA9])


path = pathlib.Path(__file__).parent.joinpath('result.txt')

with path.open('wb') as fobj:
    fobj.write(data)


with path.open('r') as fobj:
    data = fobj.read()


print(data)
