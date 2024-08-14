class Descriptor:
    def __set__(self, obj, value):
        if not isinstance(value, int):
            raise Exception('!!! THIS SHOULD HAVE BEEN AN int TYPE.')
        obj.__dict__['attr'] = value
    def __get__(self, obj, objtype=None):
        return obj.__dict__['attr']

class Klass:
    attr = Descriptor()

if __name__ == '__main__':
    aaa = Klass()
    bbb = Klass()
    try:
        aaa.attr = 'n/a'
    except:
        pass
    else:
        raise Exception('!!! this should have errored !!!')

    aaa.attr = 5
    bbb.attr = 9876
    assert aaa.attr == 5, '!!! GOT THE WRONG VALUE BACK.'
    assert bbb.attr == 9876, '!!! GOT THE WRONG VALUE BACK.'
