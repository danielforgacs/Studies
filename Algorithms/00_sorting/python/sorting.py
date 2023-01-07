def insertion_sort(a):
    for k in range(1, len(a)):
        key = a[k]
        i = k - 1
        while (i >= 0) and (a[i] > key):
            a[i+1] = a[i]
            i -= 1
        a[i+1] = key

def test_insertion_sort():
    a = [500, 400, 300, 200, 100]
    insertion_sort(a)
    assert a == sorted(a), a



# if __name__ == '__main__':
#     print(a)