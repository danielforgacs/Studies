def insertion_sort(a):
    for j in range(1, len(a)):
        key = a[j]
        i = j - 1
        while (i > 0) and (a[i] > key):
            a[i+1] = a[i]
            i -= 1
        a[i+1] = key

if __name__ == '__main__':
    a = [300, 200, 100]
    insertion_sort(a)
    print(a)