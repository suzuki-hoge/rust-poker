# Poker

## Demo
```
$ cargo run
     
( example ) input: S-2 H-A D-J Joker C-10
input: S-2 H-8 S-8 C-6 C-8

ThreeOfAKind ( strongest: S-8 )
```
```
$ cargo run

( example ) input: S-2 H-A D-J Joker C-10
input: S-2 H-8 S-8 Joker C-8

FourOfAKind ( strongest: Joker )
```

```
$ cargo test

running 10 tests
test hand::tests::flush_test ... ok
test hand::tests::four_of_a_kind_test ... ok
test hand::tests::one_pair_test ... ok
test hand::tests::full_house_test ... ok
test hand::tests::royal_straight_flush_test ... ok
test hand::tests::straight_flush_test ... ok
test hand::tests::straight_test ... ok
test hand::tests::two_pair_test ... ok
test hand::tests::judge_test ... ok
test hand::tests::three_of_a_kind_test ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
## Cards
### Suits
Spade, Heart, Diamond, Club

### Ranks
2 ~ 10, 11, 12, 13, Ace

### Joker
only one joker

## Hands
+ High Cards
+ One Pair
+ Two Pair
+ Three Of A Kind
+ Straight
+ Flush
+ Full House
+ Four Of A Kind
+ Straight Flush
+ Royal Straight Flush
