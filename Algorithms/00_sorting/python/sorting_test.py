def bubble_sort(a):
    for k in range(len(a)-1):
        for i in range(1, len(a)):
            if a[i] < a[i-1]:
                temp = a[i]
                a[i] = a[i-1]
                a[i-1] = temp


def bubble_sort_B(a):
    for k in range(len(a)-1):
        for i in range(len(a)-1, k, -1):
            if a[i] < a[i-1]:
                a[i], a[i-1] = a[i-1], a[i]


def insertion_sort(a):
    for k in range(1, len(a)):
        key = a[k]
        i = k - 1
        while (i >= 0) and (a[i] > key):
            a[i+1] = a[i]
            i -= 1
        a[i+1] = key


def merge_sort(a):
    if len(a) < 2:
        return
    mid = len(a) // 2
    l = a[:mid]
    r = a[mid:]
    merge_sort(l)
    merge_sort(r)
    i = j = 0
    for k, _ in enumerate(a):
        if i == len(l):
            a[k:] = r[j:]
            break
        elif j == len(r):
            a[k:] = l[i:]
            break
        elif l[i] <= r[j]:
            a[k] = l[i]
            i += 1
        else:
            a[k] = r[j]
            j += 1


def test_bubble_sort():
    a = [99, 88, 77, 66, 55, 44, 33, 22, 11]
    bubble_sort(a)
    assert a == sorted(a)

def test_bubble_sort_B():
    a = [99, 88, 77, 66, 55, 44, 33, 22, 11]
    bubble_sort_B(a)
    assert a == sorted(a)


def test_insertion_sort():
    a = [99, 88, 77, 66, 55, 44, 33, 22, 11]
    insertion_sort(a)
    assert a == sorted(a), a


def test_merge_sort():
    a = [99, 88, 77, 66, 55, 44, 33, 22, 11]
    merge_sort(a)
    assert a == sorted(a)


if __name__ == '__main__':
    # a = [5, 4, 3, 2, 1]
    # a = [2, 1]
    a = [4, 3, 2, 1]
    bubble_sort_B(a)
    print('------------------------')
    print(a)