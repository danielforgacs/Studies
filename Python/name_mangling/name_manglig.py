class _BaseClass:
    __instance_counter = 0

    def __init__(self):
        self.__class__.__instance_counter += 1
        self.__counter = self.__class__.__instance_counter


    def get_counter(self):
        return self.__counter


class Klass(_BaseClass):
    def get_id(self):
        # return self.__counter
        sup = super()
        return sup.get_counter()


def main():
    stuff_1 = Klass()
    print(stuff_1.get_id())
    stuff_2 = Klass()
    print(stuff_2.get_id())
    stuff_3 = Klass()
    print(stuff_3.get_id())
    stuff_4 = Klass()
    stuff_5 = Klass()
    stuff_6 = Klass()
    print(stuff_6.get_id())
    print(stuff_1.get_id())


if __name__ == '__main__':
    main()
