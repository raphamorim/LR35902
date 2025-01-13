extern crate gameboy;

use gameboy::gameboy::{load_rom, Gameboy, RenderMode::Terminal};

fn main() {
    // TODO: Allow receive path by arguments
    if let Ok((data, filepath)) = load_rom("./../the-machine.gb") {
    // if let Ok((data, filepath)) = load_rom("./../bakery.gb") {
        let gb = Gameboy::new(data, Some(filepath));
        gb.render(Terminal);
    } else {
        panic!("error loading rom");
    }
}
