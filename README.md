# Battleship

## Overview

Welcome to Battleship! This application is a recreated version of the classic game "Battleship" using the GGEZ crate in Rust.

We wrote this to better understand how Rust works as a language. So far, we learned that Rust is STRICT! Where other languages may say, "Meh . . . that's good enough. We'll compile that for you," Rust says, "NO! Go back and fix your program until it's EXACTLY correct." Rust's compiler is very strict, but this is what makes Rust as powerful as it is. Because Rust requires exactness, Rust ensures consistency across differing datatypes.

## Development Environment

We programmed this game using cargo (Rust's crate manager). Cargo makes compiling and running Rust code easy and allows the use of crates (like modules in Python). The main crate we used for our program is GGEZ.

GGEZ is a crate in Rust designed for basic 2D game development. This crate comes with an implementation called "EventHandler" that comes with two functions that are required as apart of it's implementation. Update oversees keeping track of changes in the program while draw renders the screen. Both are called at about the rate of 60 frames per second in an infinite loop. We used this loop to update and display our game by drawing elements on the screen and creating functions to select and update those elements (like HTML and JavaScript).

Rust does not come with "classes" as other languages do, but rather Rust obtains this same functionality by using a struct combined with an impl (implementation). We used BattleShipGame, a struct combined with an impl, to operate as the object to handle our game's logic as well as a Player struct to act as objects for each individual player.

## Useful Websites

The following websites and recourses were helpful for us:

- [YouTube "Intro to Rust" Tutorial](https://www.youtube.com/watch?v=4q3Z5RBX7hQ&t=296s)
  - We used other YouTube tutorials as well.
- [Rust Docs](https://doc.rust-lang.org/rustc/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [GGEZ Docs](https://ggez.rs/)
  - We looked up several functions in this library for this.
- [Rust Battle Ship Game Example](https://github.com/BekBrace/rust-bship-game/blob/main/src/main.rs)
  - We did use the CellState struct from this as well as the [[CellState; GRID_SIZE]; GRID_SIZE] datatype. The rest of our code is original.

## Future Work

We can do the following in the future to make this application better:

- Update this program to be able to display the sprites we created.
- Revise and refractor the code to reduce redundancies in datatypes and ensure maintainability.
- Add stats to the player struct to manage a player's number of ships.
- Set up an IA for the main player to play against. (this was our original goal)
