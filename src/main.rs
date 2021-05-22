use std::fs::File;
use std::io::prelude::*;
mod plot;

use byteorder::WriteBytesExt;
use byteorder::LittleEndian;


fn main() -> std::io::Result<()> {
    
    let timbre = plot::gen_timbre_base(4);

    let mut wave = Vec::<f32>::new();

    for i in 0..44100 {
        let frame = &timbre[i/348 as usize];
        let mut val = 0.0;
        for harm in 0..127 {
            let harmf = harm as f32;
            let amp = (frame[harm] as f32 + 0.5001) * (128.0 / (1.0 + harmf)) * 0.7;
            let phase = (harmf*(i as f32)*3.1415*2.0*2.0/256.0).sin();
            val += amp * phase;
        }
        wave.push(val as f32);
    }

    let mut file = File::create("master.pipe")?;
    for val in wave.iter() {
        file.write_f32::<LittleEndian>(*val)?;
    }

  //for val in 1..(44100*3) {
  //    file.write_f32::<LittleEndian>(0.6*((val as f32)/10.0).sin())?;
  //}

  //plot::plot_timbre(plot::gen_timbre_base(4));

    Ok(())
}
