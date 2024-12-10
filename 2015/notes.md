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

# Day 10 - Look-and-say sequence
I solved this with brute force, but it looks like there are interesting ways to solve this in a much shorter fashion -[Wikipedia Article](https://en.wikipedia.org/wiki/Look-and-say_sequence)

## Cosmological Decay
Assuming that only digits 1, 2, and 3 are used then there are a finite number of subsequences that never again interact with their neighbors. There are 92 sequences,
aligning with the number of elements on the periodic table. E.g., the sequence `3113322113` aligns with `Bi` and decays into `Pm.Pb`. `Pm` is `132` and `Pb` decays into `123222113`. You can
continue this using only the element formats to quickly iterate through what a sequence will look like at some step.

As an example, `Bi` on step three will look like this:
1. `Bi` -> `Pm.Pb`
2. `Pm.Pb` -> `Nd.Ti`
    - `Pm` -> `Nd`, `Pb` -> `Ti`
3. `Nd.Ti` -> `Pr.Hg`
  - To convert this back into the correct number, just add together the string sequences for `Pr` and `Hg`, which will be `"31131112" + "31121123222113"`, or `"3113111231121123222113"`

## Length Formula
Lim(n->∞) `L(n+1) / L(n) = CC` where `CC` is a constant. `L(n)` is the number of digits of the `n`th member of the sequence. `CC` is a constant equal to
`1.303577269034...`.

# Day 11 - Password Expiration
Day 11 involved taking some 8 char string password, incrementing it, checking if the new password meets some requirements, and if it doesn't continue to
increment until those requirements are met. "Incrementing" here means taking the right-most character and transforming it from `a -> b` or `b -> c` ... `z -> a`.

Repeatedly allocating strings is expensive and making large maps for converting characters is non-optimal. I solved this by storing the 8 char password as an
array of 8 bytes and mutating that array in place, as this involved no allocation as the password is being generated.
