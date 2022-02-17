# rust-bowling-kata

An unfinished solution in Rust to the [Poker Kata](https://codingdojo.org/kata/PokerHands/). This was a great learning experience in terms of pushing Rust's monadic operators for std::result and similar to their limits.
Usage of the code can be seen in `main.rs` as such:
```rust
fn poker_hand(input: &str) -> Result<String, &str> {
    poker::serialize(
      &poker::deserialize(input).unwrap().compare()
    ).map_err(|_| "Error")
}
```

The entry points for the code are thus in `src/poker/serializer.rs` and `src/poker/deserializer.rs`.
This project makes heavy use of monadic chains on the Result type for learning purposes, such as this function:

```rust
pub fn deserialize(input: &str) -> Result<CompareHands, DeserializeError> {
    let re = Regex::new(r"^Black: (.*)  White: (.*)$").expect("Invalid Regex in deserializer.");

    re.captures(input)
        .and_then(|captures| {
            // Did we match something after "Black" and after "White"?
            let black = captures.get(1)?;
            let white = captures.get(2)?;

            Some((black.as_str(), white.as_str()))
        })
        .ok_or(DeserializeError::BadFormat)
        .and_then(|matches| {
            // Convert text to lists of Card
            let (black_text, white_text) = matches;

            let black_hand = deserialize_cards_to_hand(black_text)?;
            let white_hand = deserialize_cards_to_hand(white_text)?;

            Ok(CompareHands {
                black: black_hand,
                white: white_hand,
            })
        })
}
```

By using `ok_or` we can convert a regex level error into a more semantic `DeserializeError`, then using `and_then` lets us write what code should be run if no error happened at all.

We can also see in `src/poker/compare_hands.rs` that overuse of this chaining can lead to very strange looking code.
Trying to determine if there's a Pair scored in a hand produced this code:
```rust
  // if high_card is None, then all this code is ignored and we return None
  high_card(&filtered_black, &filtered_white).and_then(|result| {
      // Same here - if result.winner is None, ignore the .and_then and propagate the None
      result.winner.and_then(|winner| {
          // Match the winner.win_type against HighCard - if it's any other value, you guessed it, propagate None.
          match winner.win_type {
              // We destructure to pull the rank value out, and reuse it to convert from HighCard(rank) to PairHighCard(rank)
              WinType::HighCard(rank) => Some(ComparisonResult::make(
                  winner.player,
                  WinType::PairHighCard(rank),
              )),
              _ => None,
          }
      })
  })
```
This may well have been much simpler written as imperative code.
