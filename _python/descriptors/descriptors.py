class Descriptor:
    def __set__(self, **kwargs):
        pass
    def __get__(self, **kwargs):
        pass



class Klass:
    attr = Descriptor()


if __name__ == '__main__':
    instance = Klass()

    try:
        instance.attr = None
    except:
        pass
    else:
        raise Exception('!!! this should have errored !!!')
