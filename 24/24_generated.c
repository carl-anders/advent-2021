
#include <stdio.h>
#include <stdbool.h>

static inline bool valid(int input[]) {
    int w = 0;
    int x = 0;
    int y = 0;
    int z = 0;
w = input[0];
x *= 0;
x += z;
x %= 26;
z /= 1;
x += 13;
x = (x == w) ? 1 : 0;
x = (x == 0) ? 1 : 0;
y *= 0;
y += 25;
y *= x;
y += 1;
z *= y;
y *= 0;
y += w;
y += 5;
y *= x;
z += y;
w = input[1];
x *= 0;
x += z;
x %= 26;
z /= 1;
x += 15;
x = (x == w) ? 1 : 0;
x = (x == 0) ? 1 : 0;
y *= 0;
y += 25;
y *= x;
y += 1;
z *= y;
y *= 0;
y += w;
y += 14;
y *= x;
z += y;
w = input[2];
x *= 0;
x += z;
x %= 26;
z /= 1;
x += 15;
x = (x == w) ? 1 : 0;
x = (x == 0) ? 1 : 0;
y *= 0;
y += 25;
y *= x;
y += 1;
z *= y;
y *= 0;
y += w;
y += 15;
y *= x;
z += y;
w = input[3];
x *= 0;
x += z;
x %= 26;
z /= 1;
x += 11;
x = (x == w) ? 1 : 0;
x = (x == 0) ? 1 : 0;
y *= 0;
y += 25;
y *= x;
y += 1;
z *= y;
y *= 0;
y += w;
y += 16;
y *= x;
z += y;
w = input[4];
x *= 0;
x += z;
x %= 26;
z /= 26;
x += -16;
x = (x == w) ? 1 : 0;
x = (x == 0) ? 1 : 0;
y *= 0;
y += 25;
y *= x;
y += 1;
z *= y;
y *= 0;
y += w;
y += 8;
y *= x;
z += y;
w = input[5];
x *= 0;
x += z;
x %= 26;
z /= 26;
x += -11;
x = (x == w) ? 1 : 0;
x = (x == 0) ? 1 : 0;
y *= 0;
y += 25;
y *= x;
y += 1;
z *= y;
y *= 0;
y += w;
y += 9;
y *= x;
z += y;
w = input[6];
x *= 0;
x += z;
x %= 26;
z /= 26;
x += -6;
x = (x == w) ? 1 : 0;
x = (x == 0) ? 1 : 0;
y *= 0;
y += 25;
y *= x;
y += 1;
z *= y;
y *= 0;
y += w;
y += 2;
y *= x;
z += y;
w = input[7];
x *= 0;
x += z;
x %= 26;
z /= 1;
x += 11;
x = (x == w) ? 1 : 0;
x = (x == 0) ? 1 : 0;
y *= 0;
y += 25;
y *= x;
y += 1;
z *= y;
y *= 0;
y += w;
y += 13;
y *= x;
z += y;
w = input[8];
x *= 0;
x += z;
x %= 26;
z /= 1;
x += 10;
x = (x == w) ? 1 : 0;
x = (x == 0) ? 1 : 0;
y *= 0;
y += 25;
y *= x;
y += 1;
z *= y;
y *= 0;
y += w;
y += 16;
y *= x;
z += y;
w = input[9];
x *= 0;
x += z;
x %= 26;
z /= 26;
x += -10;
x = (x == w) ? 1 : 0;
x = (x == 0) ? 1 : 0;
y *= 0;
y += 25;
y *= x;
y += 1;
z *= y;
y *= 0;
y += w;
y += 6;
y *= x;
z += y;
w = input[10];
x *= 0;
x += z;
x %= 26;
z /= 26;
x += -8;
x = (x == w) ? 1 : 0;
x = (x == 0) ? 1 : 0;
y *= 0;
y += 25;
y *= x;
y += 1;
z *= y;
y *= 0;
y += w;
y += 6;
y *= x;
z += y;
w = input[11];
x *= 0;
x += z;
x %= 26;
z /= 26;
x += -11;
x = (x == w) ? 1 : 0;
x = (x == 0) ? 1 : 0;
y *= 0;
y += 25;
y *= x;
y += 1;
z *= y;
y *= 0;
y += w;
y += 9;
y *= x;
z += y;
w = input[12];
x *= 0;
x += z;
x %= 26;
z /= 1;
x += 12;
x = (x == w) ? 1 : 0;
x = (x == 0) ? 1 : 0;
y *= 0;
y += 25;
y *= x;
y += 1;
z *= y;
y *= 0;
y += w;
y += 11;
y *= x;
z += y;
w = input[13];
x *= 0;
x += z;
x %= 26;
z /= 26;
x += -15;
x = (x == w) ? 1 : 0;
x = (x == 0) ? 1 : 0;
y *= 0;
y += 25;
y *= x;
y += 1;
z *= y;
y *= 0;
y += w;
y += 5;
y *= x;
z += y;

    return(z == 0);
}

int main(int argc, char *argv[]) {
    for (int a = 9; a >= 1; a--) { //  0
    for (int b = 9; b >= 1; b--) { //  1
    for (int c = 9; c >= 1; c--) { //  2
    for (int d = 9; d >= 1; d--) { //  3
    for (int e = 9; e >= 1; e--) { //  4
        printf("%d%d %d%d%d --- --- ---\n",
            a, b, c, d, e);
    for (int f = 9; f >= 1; f--) { //  5
    for (int g = 9; g >= 1; g--) { //  6
    for (int h = 9; h >= 1; h--) { //  7
    for (int i = 9; i >= 1; i--) { //  8
    for (int j = 9; j >= 1; j--) { //  9
    for (int k = 9; k >= 1; k--) { // 10
    for (int l = 9; l >= 1; l--) { // 11
    for (int m = 9; m >= 1; m--) { // 12
    for (int n = 9; n >= 1; n--) { // 13
        int input[] = {a,b,c,d,e,f,g,h,i,j,k,l,m,n};
        if (valid(input)) {
            printf("Valid: %d%d %d%d%d %d%d%d %d%d%d %d%d%d\n",
                a, b, c, d, e, f, g, h, i, j, k, l, m, n);
        }

    }}}}}}}}}}}}}}
    return 0;
}
