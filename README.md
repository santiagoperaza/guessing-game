# Guessing Game
Simple command line game to guess a number between 1 and 100.

Inspired by [The Rust Programming Language book](https://doc.rust-lang.org/book/title-page.html), on top of that I added some additional features:

- Show a hint of whether the secret number is even or odd.
- Display number of attempts until guessing the secret number.

If you have rust installed, simply execute `cargo run` and start playing!

## Steps to run using docker

- docker build -t game .
- docker run -it game