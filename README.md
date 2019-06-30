# tetris_core
Simple Tetris game logic (with no interface)

**WARNING:**
This project was created on an early stage of learning RUST.
It might lack many good practices.

## Usage:
- Use any drawing library. (i.e: [piston_window](https://crates.io/crates/piston_window))
- Use any randomizer library or method (i.e: [rand](https://crates.io/crates/rand))

As an example of implementation, you can check https://github.com/etoledom/rust_practice/blob/master/07_tetris/src/main.rs

Implement the `Randomizer` trait
```rust
struct Rand;
impl Randomizer for Rand {
    fn random_between(&self, lower: i32, higher: i32) -> i32 {
        let mut rng = rand::thread_rng();
        return rng.gen_range(lower, higher);
    }
}
``` 

Instantiate a Tetris Game instance using an instance or your randomizer struct and the desired board size:
```rust
let game_size = Size {
    height: 20,
    width: 10,
};
let mut game = Game::new(&game_size, Box::new(rand));
```

#### `update(&mut self, delta_time: f64)`

Call `game.update(delta_time);` on every game loop.

#### `draw(&self) -> Vec<Block>`

Get the board model to be drawn:
```rust
let game_blocks = game.draw();
```

A block is a structure that specifies the block's position, size and color in rgba. Position and size are unitary, you can give it the unit and size you want.
```
struct Block {
	pub rect: Rect,
	pub color: Color,
}
```

#### `perform(&mut self, action: Action)`
 Perform movement and rotation actions
```rust
game.perform(Action::Rotate);
```

#### `is_game_over(&self) -> bool`
Checks if is game over.

#### `get_score(&self) -> u64`
Gets the current score.
