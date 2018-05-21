import unittest


def find_nb(m):
    n = 1

    while (n*(n+1)//2)**2 < m:
        n += 1

    if (n*(n+1)//2)**2 == m:
        return n
    else:
        return -1


class TestExercise(unittest.TestCase):
    def test_find_mb(self):
        self.assertEqual(find_nb(4183059834009), 2022)
        self.assertEqual(find_nb(24723578342962), -1)
        self.assertEqual(find_nb(135440716410000), 4824)
        self.assertEqual(find_nb(40539911473216), 3568)
        self.assertEqual(find_nb(26825883955641), 3218)


if __name__ == '__main__':
    unittest.main()
