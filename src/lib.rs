use proc_macro::TokenStream;

#[proc_macro]
pub fn match_days(up_to: TokenStream) -> TokenStream {
    let mut output = "match day {\n".to_string();
    let up_to = up_to.to_string().parse().unwrap();
    for i in 1..=up_to {
        output += &*format!("{0} => run::<days::Day{0}>(file, result.results),\n", i);
    }
    output += "_ => unreachable!()\n}";
    output.parse().unwrap()
}
#[proc_macro]
pub fn mod_days(up_to: TokenStream) -> TokenStream {
    let mut output = String::new();
    let up_to = up_to.to_string().parse().unwrap();
    for i in 1..=up_to {
        output += &*format!("mod day{0};\npub use day{0}::Day{0};", i);
    }
    output.parse().unwrap()
}
