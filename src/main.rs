#![allow(unused)]

mod builder;
mod corner;
mod enemy;
mod game;
mod map;
mod player;
mod point;
mod startscreen;
mod utilities;

use game::mainloop;
use std::result::Result;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    
    mainloop()?;

    Ok(())

}
