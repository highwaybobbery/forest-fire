extern crate rand;
extern crate ansi_term;
use Tile::{Empty, Tree, Burning};
use rand::Rng;
use std::cmp::Ordering;
use std::time::Duration;
use std::fmt;
use ansi_term::Colour::*;

const GROW_PROB: u32 = 2;
const INITIAL_TREE_PROB: u32 = 20;
const FIRE_PROB: u32 = 5;
const MAX_GENERATIONS: u32 = 31;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Empty,
    Tree,
    Burning,
}
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Empty => write!(f, " "),
            Tree => write!(f, "{}", Green.paint("T")),
            Burning => write!(f, "{}", Red.paint("#"))
        }
    }
}

fn main() {
    let mut tiles = [[Empty; 30]; 30];
    for row in tiles.iter_mut() {
        for tile in row.iter_mut() {
            *tile = if prob_check(INITIAL_TREE_PROB) { Tree } else { Empty } ;
        }
    }
    let sleep_time = Duration::new(0,500000000); // 1/2 seconds
    let mut y = 0;
    for gen in 1..MAX_GENERATIONS {
        println!("{}", gen);
        let mut x = 0;
        for row in tiles.iter() {
            for tile in row.iter() {
                let mut neighbor_is_burning = false;
                if ((x - 1) >= 0) && ((y - 1) >= 0) {
                    // SAAAAD FACE
                    if tiles[y - 1][x - 1] == Burning { neighbor_is_burning = true }
                }
                // if (y - 1) >= 0 {
                //     if tiles[y - 1][x] == Burning { neighbor_is_burning = true }
                // }
                // if (x + 1) < 30 {
                //     if tiles[y - 1][x + 1] == Burning { neighbor_is_burning = true }
                // }
                // if (x - 1) >= 0 {
                //     if tiles[y][x - 1] == Burning { neighbor_is_burning = true }
                // }
                // if (x + 1) < 30 {
                //     if tiles[y][x + 1] == Burning { neighbor_is_burning = true }
                // }
                // if (x - 1) >= 0 && (y + 1) < 30 {
                //     if tiles[y + 1][x - 1] == Burning { neighbor_is_burning = true }
                // }
                // if (y + 1) < 30 {
                //     if tiles[y + 1][x] == Burning { neighbor_is_burning = true }
                // }
                // if (x + 1) < 30 && (y + 1) < 30 {
                //     if tiles[y + 1][x + 1] == Burning { neighbor_is_burning = true }
                // }
                *tile = modify_tile(*tile, neighbor_is_burning);
                print!("{}", tile);
                x += 1;
            }
            println!("");
            y += 1;
        }
        std::thread::sleep(sleep_time);
        clear_screen();
    }
}

fn modify_tile(tile: Tile, neighbor_is_burning: bool) -> Tile {
    match tile {
        Empty => {
            if prob_check(GROW_PROB) == true {
                Tree
            } else {
                tile
            }
        },
        Tree => {
            if prob_check(FIRE_PROB) == true || neighbor_is_burning {
                Burning
            } else {
                tile
            }
        }
        Burning => Empty
    }
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}

fn prob_check(chance: u32) -> bool {
    let roll: u32 = rand::thread_rng().gen_range(1, 101);
    if roll.cmp(&chance) == Ordering::Less { true } else { false }
}
