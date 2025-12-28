# Exercise: Reverse a String (Python)

::: exercise
id: py-reverse-string
difficulty: beginner
time: 10 minutes
:::

Write a function `reverse_str` that returns the reverse of a string.

::: starter file="reverse.py" language=python
```python
def reverse_str(s: str) -> str:
    # TODO: return the reverse of s
    # Hint: try slicing or reversed()
    raise NotImplementedError
```
:::

::: hint level=1 title="Slicing"
Python slicing supports negative steps. For example: `s[::-1]`.
:::

::: hint level=2 title="Built-in reversed"
`''.join(reversed(s))` returns a reversed string.
:::

::: solution reveal=on-demand
```python
def reverse_str(s: str) -> str:
    return s[::-1]
```
:::

::: tests mode=local language=python
```python
import unittest

class TestReverse(unittest.TestCase):
    def test_reverse(self):
        self.assertEqual(reverse_str("abc"), "cba")
        self.assertEqual(reverse_str(""), "")
        self.assertEqual(reverse_str("a"), "a")

if __name__ == '__main__':
    unittest.main()
```
:::

::: reflection
- Which approach do you prefer: slicing or reversed()?
- How would you handle non-ASCII strings? (Hint: Python str is Unicode.)
:::

