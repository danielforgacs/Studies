class K:
    def __init__(self):
        self.attr = 1234
        self._attr_2 = 1234
        self.__attr_3 = 1234


i = K()
print(dir(i))
