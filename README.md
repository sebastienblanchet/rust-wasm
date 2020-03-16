# rust-wasm

**Sebastien Blanchet***

Follow example in [Book](https://rustwasm.github.io/book/)

## Requirements:

Rust [as per](https://www.rust-lang.org/tools/install):
* `rustup`
* `rustc`
* `cargo`
* `cargo-generate`

Web assembly [as per](https://rustwasm.github.io/wasm-pack/installer/)
* `wasm`

Node [as per](https://www.npmjs.com/get-npm)
* `npm`

## Build

Once in the your `$/rust-wasm`:

```bash
λ cd wasm-game-of-life
λ wasm-pack build
```

This will create a `$/rust-wasm/wasm-game-of-life/pkg`. To put this in a webpage using [create-wasm-app](https://github.com/rustwasm/create-wasm-app):
``` bash
λ npm init wasm-app www
```

This will generate the contents in `$/rust-wasm/wasm-game-of-life/www`.


## Conways Game of Life

Implement From [wikipedia](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life): 

> The universe of the Game of Life is an infinite, 2D orthogonal grid of square cells, each of which is
> in one of two possible states, alive || dead (populated || unpopulated, respectively). Every cell interacts 
> with its eight neighbours, which are the cells that are horizontally, vertically, or diagonally adjacent. At each 
> step in time, the following transitions occur:
> 
>     Any live cell with fewer than two live neighbours dies, as if by underpopulation.
>     Any live cell with two or three live neighbours lives on to the next generation.
>     Any live cell with more than three live neighbours dies, as if by overpopulation.
>     Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
>
> These rules, which compare the behavior of the automaton to real life, can be condensed into the following:
> 
>     Any live cell with two or three neighbors survives.
>     Any dead cell with three live neighbors becomes a live cell.
>     All other live cells die in the next generation. Similarly, all other dead cells stay dead.
>
> The initial pattern constitutes the seed of the system. The first generation is created by applying the above rules 
> simultaneously to every cell in the seed; births and deaths occur simultaneously, and the discrete moment at which 
> this happens is sometimes called a tick. Each generation is a pure function of the preceding one. The rules 
> continue to be applied repeatedly to create further generations.
>


## Contents:

In repository `rust-wasm` found in your current working directory `$`, listed below are the relevant files:

``` bash
.
├── Cargo.lock
├── Cargo.toml
├── pkg
│   ├── package.json
│   ├── wasm_game_of_life.d.ts
│   ├── wasm_game_of_life.js
│   ├── wasm_game_of_life_bg.d.ts
│   └── wasm_game_of_life_bg.wasm
├── src
│   ├── lib.rs
│   └── utils.rs
├── tests
│   └── web.rs
└── www
    ├── bootstrap.js
    ├── index.html
    ├── index.js
    ├── package-lock.json
    ├── package.json
    └── webpack.config.js
```