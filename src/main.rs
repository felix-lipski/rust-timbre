use std::fs::File;
use std::io::prelude::*;
mod plot;

use byteorder::WriteBytesExt;
use byteorder::LittleEndian;

use interpolation::Lerp;

use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let timbre = plot::gen_timbre_base(4);

    let mut wave = Vec::<f32>::new();

    for i in 0..44100 {
        let frame_pos = (i as f32 * 126.0)/(44100.0);
      //println!("pos {:.2}", frame_pos);
        let frame1 = &timbre[(frame_pos) as usize];
        let frame2 = &timbre[(frame_pos) as usize + 1_usize];
        let mut val = 0.0;
        for harm in 0..126 {
            let harmf = harm as f32;
            let amp1 = (frame1[harm] as f32 + 0.5001);
            let amp2 = (frame2[harm] as f32 + 0.5001);
            let amp = amp1.lerp(&amp2, &(frame_pos % 1.0));
          //let amp = 1.0;
            let phase = (harmf*(i as f32)*3.1415*2.0*2.0/256.0).sin();
          //val += amp * phase * (1.0 / (1.0 + harmf)) * 0.7;
            val += amp * phase * (1.0 / (2f32.powi(harm as i32))) * 0.7;
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
    

    let root = BitMapBackend::new("wave.png", (1366, 768)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..2000f32, -1.0f32..1.0f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (0..2000).map(|x| (x as f32, wave[x as usize] as f32)),
            &RED,
        ))?
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;


    Ok(())
}
