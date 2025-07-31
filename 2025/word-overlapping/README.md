# Word Overlapping

Given two words, overlap them in such a way, morphing the last few letters of the first word with the first few letters of the second word.

Return the shortest overlapped word possible.

# Examples

```js
overlap("sweden", "denmark")
output = "swedenmark"

overlap("honey", "milk")
output = "honeymilk"

overlap("dodge", "dodge") "dodge"
```

# Notes

- All words will be given in lowercase.
- If no overlap is possible, return both words one after the other
