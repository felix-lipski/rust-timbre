use std::fs::File;
use std::io::prelude::*;
mod plot;

use byteorder::WriteBytesExt;
use byteorder::LittleEndian;


fn main() -> std::io::Result<()> {
    

    let mut file = File::create("master_pipe")?;
    for val in 1..(44100*3) {
        file.write_f32::<LittleEndian>(0.6*((val as f32)/10.0).sin())?;
    }

  //plot::plot_timbre(plot::gen_timbre_base(4));

    Ok(())
}
