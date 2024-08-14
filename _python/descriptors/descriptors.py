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
    instance = Klass()
    try:
        instance.attr = 'n/a'
    except:
        pass
    else:
        raise Exception('!!! this should have errored !!!')
    instance.attr = 5
    assert instance.attr == 5, '!!! COULD NOT GET BACK THE ASSIGNED VALUE.'
