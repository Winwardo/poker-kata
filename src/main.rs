extern crate regex;

fn main() {
    println!("Hello, world!");
}

mod poker;

#[allow(dead_code)]
fn poker_hand(input: &str) -> Result<String, &str> {
    // NOT SAFE
    poker::serialize(&poker::deserialize(input).unwrap().compare()).map_err(|_| "Error")
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    extern crate rstest;
    use rstest::rstest_parametrize;

    use super::*;

    #[rstest_parametrize(
        expected,
        input,
        case(
            "White wins. - with high card: Ace",
            "Black: 2H 3D 5S 9C KD  White: 2C 3H 4S 8C AH"
        ),
        case(
            "Black wins. - with full house: 4 over 2",
            "Black: 2H 4S 4C 2D 4H  White: 2S 8S AS QS 3S"
        ),
        case(
            "Black wins. - with high card: 9",
            "Black: 2H 3D 5S 9C KD  White: 2C 3H 4S 8C KH"
        ),
        case("Tie.", "Black: 2H 3D 5S 9C KD  White: 2D 3H 5C 9S KH")
    )]
    fn poker_hand_test(expected: &str, input: &str) {
        assert_eq!(Ok(expected.to_string()), poker_hand(input));
    }
}
