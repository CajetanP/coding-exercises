import unittest


def gcdi(a, b):
    return abs(a) if b == 0 else gcdi(abs(b), abs(a) % abs(b))


def lcmu(a, b):
    return abs(a) * abs(b) / gcdi(a, b)


def som(a, b):
    return a + b


def maxi(a, b):
    return max(a, b)


def mini(a, b):
    return min(a, b)


def oper_array(fct, arr, init):
    result = []

    result.append(fct(init, arr[0]))
    for i in range(len(arr)-1):
        result.append(fct(result[i], arr[i+1]))

    return result


class TestExercise(unittest.TestCase):
    def test_som(self):
        self.assertEqual(oper_array(som, [18, 69, -90, -78, 65, 40], 0),
                         [18, 87, -3, -81, -16, 24])

    def test_lcmu(self):
        self.assertEqual(oper_array(lcmu, [18, 69, -90, -78, 65, 40], 18),
                         [18, 414, 2070, 26910, 26910, 107640])

    def test_maxi(self):
        self.assertEqual(oper_array(maxi, [18, 69, -90, -78, 65, 40], 18),
                         [18, 69, 69, 69, 69, 69])

    def test_gcdi(self):
        self.assertEqual(oper_array(gcdi, [18, 69, -90, -78, 65, 40], 18),
                         [18, 3, 3, 3, 1, 1])


if __name__ == '__main__':
    unittest.main()
