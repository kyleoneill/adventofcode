# Day 2 - Proof of Work Problem
Given some static prefix, append numbers to the prefix until the md5 hash of the resulting string begins
with five 0's for part 1 and six 0's for part two.
```
static_string = abcdef
number = 0
loop {
    combined = static_string + number
    hash = md5(combined)
    if hash.startswith("00000") {
        return number
    }
    number += 1
}
```

Brute forcing all numbers is slow, the jump from checking for five 0's to six 0's makes the program
take too long to be realistic.

Tricks:
1. Bitmasking - Instead of converting the hash into a hexadecimal string and doing a string comparison, we can
   do a bitmask comparison to reduce the computation needed to check the hash output. E.g., `hash[0] & 0xFF == 0x00 && hash[0] & 0xFF == 0x00 && ..`
    - For checking characters between bytes, like for part 1 where we want to check the first half of the third byte of the hash, we can do `hash[n] & 0xF0 == 0x00`
2. Ranges and testing - You can determine which range of numbers lead to the first byte of the hash changing, and then skip number ranges if the first byte is not equal
   to the start of the sequence which is desired.
    1. Compute the digest at the start
    2. Loop checking the hash of each computed string, waiting until the first byte changes
    3. Now that you know you are at the start of a range, complete step 2 again to find the range length
    4. Check the digest for the current number.
        - If it does not begin with the first desired byte, increment the number by the range size
        - If it does begin with the first desired byte check each number in this range, until the first byte is no longer valid
