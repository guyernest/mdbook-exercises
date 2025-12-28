# Exercise: Palindrome Check (JavaScript)

::: exercise
id: js-palindrome
difficulty: beginner
time: 10 minutes
:::

Write a function `isPalindrome(s)` that returns true if the string reads the same
forwards and backwards (ignoring case and spaces), otherwise false.

::: starter file="palindrome.js" language=javascript
```javascript
// Return true if s is a palindrome (case-insensitive, ignore spaces)
function isPalindrome(s) {
  // TODO: implement
  // Hints: normalize with toLowerCase() and replace spaces
  throw new Error('not implemented');
}
```
:::

::: hint level=1 title="Normalize Input"
Convert the input to lowercase and remove spaces: `s.toLowerCase().replace(/\s+/g, '')`.
:::

::: hint level=2 title="Compare With Reverse"
Compute a reversed string and compare to the normalized string.
:::

::: solution reveal=on-demand
```javascript
function isPalindrome(s) {
  const norm = s.toLowerCase().replace(/\s+/g, '');
  const rev = [...norm].reverse().join('');
  return norm === rev;
}
```
:::

::: tests mode=local language=javascript
```javascript
// Run locally with: node palindrome.test.js
// (or paste into a Node REPL after defining isPalindrome)
const assert = require('assert');

assert.strictEqual(isPalindrome('racecar'), true);
assert.strictEqual(isPalindrome('Race Car'), true);
assert.strictEqual(isPalindrome('hello'), false);
assert.strictEqual(isPalindrome(''), true);
console.log('All JS tests passed');
```
:::

::: reflection
- How would you handle punctuation or emoji?
- What’s the complexity of your solution?
:::

