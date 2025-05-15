class K:
    __slots__ = ('a',)


k = K()
k.a = 3
print(dir(k))

try:
    k.b = 3
except AttributeError as err:
    print(err)
