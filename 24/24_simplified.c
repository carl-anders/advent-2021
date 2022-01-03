
#include <stdio.h>
#include <stdbool.h>

static inline bool valid(int input[]) {
    int w = 0;
    int x = 0;
    int z = 0;

w = input[0];
z = w + 5;

w = input[1];
z *= 26;
z += (w + 14) * x;

w = input[2];
x = (z % 26) + 15;
x = (x == w) ? 0 : 1;
z *= (25 * x) + 1;
z += (15 + w) * x;

w = input[3];
x = (z % 26) + 11;
x = (x == w) ? 0 : 1;
z *= (25 * x) + 1;
z += (16 + w) * x;

w = input[4];
x = (z % 26) + -16;
x = (x == w) ? 0 : 1;
z /= 26;
z *= (25 * x) + 1;
z += (8 + w) * x;

w = input[5];
x = (z % 26) + -11;
x = (x == w) ? 0 : 1;
z /= 26;
z *= (25 * x) + 1;
z += (9 + w) * x;

w = input[6];
x = (z % 26) + -6;
x = (x == w) ? 0 : 1;
z /= 26;
z *= (25 * x) + 1;
z += (2 + w) * x;

w = input[7];
x = (z % 26) + 11;
x = (x == w) ? 0 : 1;
z *= (25 * x) + 1;
z += (13 + w) * x;

w = input[8];
x = (z % 26) + 10;
x = (x == w) ? 0 : 1;
z *= (25 * x) + 1;
z += (16 + w) * x;

w = input[9];
x = (z % 26) + -10;
x = (x == w) ? 0 : 1;
z /= 26;
z *= (25 * x) + 1;
z += (6 + w) * x;

w = input[10];
x = (z % 26) + -8;
x = (x == w) ? 0 : 1;
z /= 26;
z *= (25 * x) + 1;
z += (6 + w) * x;

w = input[11];
x = (z % 26) + -11;
x = (x == w) ? 0 : 1;
z /= 26;
z *= (25 * x) + 1;
z += (9 + w) * x;

w = input[12];
x = (z % 26) + 12;
x = (x == w) ? 0 : 1;
z *= (25 * x) + 1;
z += (11 + w) * x;

w = input[13];
x = (z % 26) + -15;
x = (x == w) ? 0 : 1;
z /= 26;
z *= (25 * x) + 1;
z += (5 + w) * x;

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
