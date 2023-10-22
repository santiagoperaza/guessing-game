# Guessing Game
Simple command line game to guess a number between 1 and 100.

Inspired by [The Rust Programming Language book](https://doc.rust-lang.org/book/title-page.html), on top of that I added some additional features:

- Show a hint of weather the secret number is even or odd.
- Display number of attempts until guessing the secret number.

If you have rust installed, simply execute `cargo run` and start playing!

## Steps to run using docker

- docker build -t game .
- docker run -it game

### Demo

<img src="https://github.com/santiagoperaza/guessing-game/assets/36051251/3cd5ed47-7d4c-4615-a3b0-6631185c6c46" width="500">
