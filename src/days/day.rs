pub trait Day {
    type Parsed: Clone;
    fn parse(input: impl Iterator<Item = String>) -> Self::Parsed;
    fn first(data: Self::Parsed) -> String;
    fn second(data: Self::Parsed) -> String;
}
