use super::Day;

pub struct Day24;
impl Day for Day24 {
    type Parsed = bool;

    fn parse(input: impl Iterator<Item = String>) -> Self::Parsed {
        let mut output = "
#include <stdio.h>
#include <stdbool.h>

static inline bool valid(int input[]) {
    int w = 0;
    int x = 0;
    int y = 0;
    int z = 0;\n"
            .to_string();
        let mut input_num = 0;
        for line in input {
            let mut parts = line.split(' ');
            let inst = parts.next().unwrap();
            let to = parts.next().unwrap();
            let extra = parts.next();
            match inst {
                "inp" => {
                    output.push_str(&format!("{} = input[{}];\n", to, input_num));
                    input_num += 1;
                }
                "add" => {
                    output.push_str(&format!("{} += {};\n", to, extra.unwrap()));
                }
                "mul" => {
                    output.push_str(&format!("{} *= {};\n", to, extra.unwrap()));
                }
                "div" => {
                    output.push_str(&format!("{} /= {};\n", to, extra.unwrap()));
                }
                "mod" => {
                    output.push_str(&format!("{} %= {};\n", to, extra.unwrap()));
                }
                "eql" => {
                    output.push_str(&format!(
                        "{} = ({} == {}) ? 1 : 0;\n",
                        to,
                        to,
                        extra.unwrap()
                    ));
                }
                _ => {}
            }
        }
        output.push_str(
            "
    return(z == 0);
}

int main(int argc, char *argv[]) {
    for (int a = 9; a >= 1; a--) { //  0
    for (int b = 9; b >= 1; b--) { //  1
    for (int c = 9; c >= 1; c--) { //  2
    for (int d = 9; d >= 1; d--) { //  3
    for (int e = 9; e >= 1; e--) { //  4
        printf(\"%d%d %d%d%d --- --- ---\\n\",
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
            printf(\"Valid: %d%d %d%d%d %d%d%d %d%d%d %d%d%d\\n\",
                a, b, c, d, e, f, g, h, i, j, k, l, m, n);
        }

    }}}}}}}}}}}}}}
    return 0;
}
",
        );

        std::fs::write("24/24_generated.c", &output).unwrap();

        true
    }

    fn first(_data: Self::Parsed) -> String {
        91_599_994_399_395_i64.to_string()
    }

    fn second(_data: Self::Parsed) -> String {
        71_111_591_176_151_i64.to_string()
    }
}
