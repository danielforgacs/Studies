class K:
    def __init__(self):
        self.attr = 'plain attr'
        self._attr_2 = 'private attr'
        self.__attr_3 = 'name mangled attr'

    # @property
    # def _attr_2(self):
    #     return self._attr_2

    # @_attr_2.setter
    # def _attr_2(self, value):
    #     pass
    #     # self._attr_2 = value


    @property
    def _attr_3(self):
        return self.__attr_3


i = K()
print(dir(i))
print(i.attr)
print(i._attr_2)
print(i._attr_3)
try:
    print(i.__attr_3)
except AttributeError:
    pass

print(i._K__attr_3)
i._K__attr_3 = 'lasjkdhlkjfh'
try:
    i.__attr_3 = 'LLLLLLLLLLLLLLl'
except AttributeError:
    pass
print(i._K__attr_3)
