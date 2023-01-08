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


# def merge_sort(arr):
#     if len(arr) < 2:
#         return
#     mid = len(arr)//2
#     L = arr[:mid]
#     R = arr[mid:]
#     print(arr, '->', L, R)
#     merge_sort(L)
#     merge_sort(R)
#     i = j = k = 0
#     while i < len(L) and j < len(R):
#         if L[i] <= R[j]:
#             arr[k] = L[i]
#             i += 1
#         else:
#             arr[k] = R[j]
#             j += 1
#         k += 1
#     while i < len(L):
#         arr[k] = L[i]
#         i += 1
#         k += 1

#     while j < len(R):
#         arr[k] = R[j]
#         j += 1
#         k += 1

def merge_sort(a):
    if len(a) < 2:
        return
    mid = len(a) // 2
    l = a[:mid]
    r = a[mid:]
    # print('array:', a, '->', 'l:', l, 'r:', r)
    merge_sort(l)
    merge_sort(r)
    print('after l:', l, 'r:', r)
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

def test_merge_sort():
    a = [555, 444, 333, 222, 111]
    merge_sort(a)
    assert a == sorted(a)


if __name__ == '__main__':
    # a = [5, 4, 3, 2, 1]
    # a = [2, 1]
    a = [4, 3, 2, 1]
    merge_sort(a)
    print('------------------------')
    print(a)