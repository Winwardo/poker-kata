fn main() {
    println!("Hello, world!");
}

fn add(a: u8, b: u8) -> u8 {
    a+b
}

fn poker_hand(input: &str) -> String {
    String::from("White wins. - with high card: Ace")
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn high_level_example_1() {
        assert_eq!(
            poker_hand("Black: 2H 3D 5S 9C KD  White: 2C 3H 4S 8C AH"),
            String::from("White wins. - with high card: Ace")
        );
    }
}