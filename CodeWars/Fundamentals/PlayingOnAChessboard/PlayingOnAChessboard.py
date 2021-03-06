import unittest


def game(n):
    return [n/2*n] if n % 2 == 0 else [n**2, 2]


class TestExercise(unittest.TestCase):
    def test_gap(self):
        self.assertEqual(game(204), [20808])
        self.assertEqual(game(807), [651249, 2])
        self.assertEqual(game(5014), [12570098])
        self.assertEqual(game(750001), [562501500001, 2])


if __name__ == '__main__':
    unittest.main()
