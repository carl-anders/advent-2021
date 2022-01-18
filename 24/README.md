## Simplifying the code
The day 24 data was changed from psuedocode to valid C. The output of this is shown in `24_generated.c`. This file will produce the right results, but it will take several hours to do so.

I then manually (sometimes with regex replace) simplified the c into `24_simplified.c` and then further to `24_simplified_2.c`.

## Noticing a pattern

After simplifying enough I noticed that the operations was like pushing and popping a stack. One operation was that the input was pushed onto the stack plus a constant. The other operation was: The stack was popped if the last pushed input was equal to the new input minus a constant.

Simplifying further into the following psuedocode:

```
push input[0] + 5;
push input[1] + 14;
push input[2] + 15;
push input[3] + 16;
pop if last - 16 == input[4];
pop if last - 11 == input[5];
pop if last - 6 == input[6];
push input[7] + 13;
push input[8] + 16;
pop if last - 10 == input[9];
pop if last - 8 == input[10];
pop if last - 11 == input[11];
push input[12] + 11;
pop if last - 15 == input[13];
```

I noticed that there were the same amount of `push` and `pop if last` operations, and the stack had to be empty for the program to be valid. Therefore all `pop if last` instructions had to succeed. This could therefore be simplified even further to:

```
input[3] + 16 == input[4] + 16
input[2] + 15 == input[5] + 11
input[1] + 14 == input[6] + 6
input[8] + 16 == input[9] + 10
input[7] + 13 == input[10] + 8
input[0] + 5 == input[11] + 11
input[12] + 11 == input[13] + 15
```

## Finished input constraints

With the data from above it could be simplified further into the following equality and range constraints:

```
input[3] == input[4]
input[2] + 4 == input[5]
input[1] + 8 == input[6]
input[8] + 6 == input[9]
input[7] + 5 == input[10]
input[0] == input[11] + 6
input[12] == input[13] + 4
```

```
input[3] = 1..=9 && input[4] = 1..=9
input[2] = 1..=5 && input[5] = 5..=9
input[1] = 1..=1 && input[6] = 9..=9
input[8] = 1..=3 && input[9] = 7..=9
input[7] = 1..=4 && input[10] = 6..=9
input[0] = 7..=9 && input[11] = 1..=3
input[12] = 5..=9 && input[13] = 1..=5
```

With these it's simple to find the maximum and minimum values manually. To get all values I also created `24_check_answers.c` to verify my findings.
