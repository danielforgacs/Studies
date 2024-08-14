class Meta(type):
    pass

class Descriptor:
    def __set_name__(self, owner, name):
        self.private_name = '_' + name
    def __set__(self, obj, value):
        if not isinstance(value, int):
            raise Exception('!!! THIS SHOULD HAVE BEEN AN int TYPE.')
        obj.__dict__[self.private_name] = value
    def __get__(self, obj, objtype):
        return obj.__dict__[self.private_name]

class A(metaclass=Meta):
    attr = Descriptor()
    stuff = Descriptor()

class B:
    attr = Descriptor()

if __name__ == '__main__':
    aaa = A()
    bbb = A()
    ccc = B()
    ddd = B()
    try:
        aaa.attr = 'n/a'
    except:
        pass
    else:
        raise Exception('!!! this should have errored !!!')

    aaa.attr = 5
    bbb.attr = 9876
    ccc.attr = 1111
    ddd.attr = 2222
    aaa.stuff = 3
    assert aaa.attr == 5, '!!! GOT THE WRONG VALUE BACK.'
    assert bbb.attr == 9876, '!!! GOT THE WRONG VALUE BACK.'
    assert ccc.attr == 1111, '!!! GOT THE WRONG VALUE BACK.'
    assert ddd.attr == 2222, '!!! GOT THE WRONG VALUE BACK.'
