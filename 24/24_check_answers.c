
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
    long long int max_num = 0;
    long long int min_num = 99999999999999;
    for (int a = 9; a >= 7; a--) { //  0
    for (int b = 5; b >= 1; b--) { //  1
    for (int c = 9; c >= 1; c--) { //  2
    for (int d = 9; d >= 1; d--) { //  3
    for (int e = 9; e >= 1; e--) { //  4
    for (int f = 9; f >= 5; f--) { //  5
    for (int g = 9; g >= 9; g--) { //  6
    for (int h = 4; h >= 1; h--) { //  7
    for (int i = 3; i >= 1; i--) { //  8
    for (int j = 9; j >= 7; j--) { //  9
    for (int k = 9; k >= 6; k--) { // 10
    for (int l = 3; l >= 1; l--) { // 11
    for (int m = 9; m >= 5; m--) { // 12
    for (int n = 5; n >= 1; n--) { // 13
        int input[] = {a,b,c,d,e,f,g,h,i,j,k,l,m,n};
        
        if (input[3] + 16 == input[4] + 16 &&
            input[2] + 15 == input[5] + 11 &&
            input[1] + 14 == input[6] + 6 &&
            input[8] + 16 == input[9] + 10 &&
            input[7] + 13 == input[10] + 8 &&
            input[0] + 5 == input[11] + 11 &&
            input[12] + 11 == input[13] + 15) {
            if (valid(input)) {
                printf("Valid: %d%d %d%d%d %d%d%d %d%d%d %d%d%d\n",
                    a, b, c, d, e, f, g, h, i, j, k, l, m, n);
                    
                long long int num = 0;
                for (int i=0;i<14;i++) {
                    num = num * 10 + input[i];
                }
                if (max_num == 0) {
                    max_num = num;
                }
                min_num = num;
            }
        }

    }}}}}}}}}}}}}}

    printf("--------------------------------\n");
    printf("Max valid number: %lld\n", max_num);
    printf("Min valid number: %lld\n", min_num);
    return 0;
}
