use noise::{OpenSimplex, NoiseFn, Seedable};
use plotters::prelude::*;
use plotters::style::text_anchor::*;
use plotters_backend::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // CONFIGURE CHART
    let root = BitMapBackend::new("out.png", (1366, 768)).into_drawing_area();
    root.fill(&BLACK).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .build_cartesian_3d(0.0..127.0, -2.0..2.0, 0.0..127.0)
        .unwrap();
    chart.with_projection(|mut pb| {
        pb.pitch = 0.5;
        pb.yaw = 0.5;
        pb.scale = 0.7;
        pb.into_matrix()
    });
    let fd = FontDesc::new(FontFamily::Monospace, 10.0 as f64, FontStyle::Normal);
    chart.configure_axes()
        .bold_grid_style(ShapeStyle {color: WHITE.to_rgba(), filled: true, stroke_width: 1})
        .label_style(TextStyle {
            color: BackendColor {alpha: 1.0 as f64, rgb: (255, 255, 255) }, 
            pos: Pos {h_pos: HPos::Center, v_pos: VPos::Center}, 
            font: fd
        })
        .draw().unwrap();




    let mut display_timbre = |timbre: Vec<Vec<f64>>| {
        for z in 0..timbre.len() {
            chart.draw_series(LineSeries::new(
                (0..127).map(|x| (x as f64, timbre[x][z], z as f64) ),
                &(RGBColor(0 as u8, 128 as u8, z as u8))
            )).unwrap();
        }
    };

    fn gen_timbre_base(octaves: i32) -> Vec<Vec<f64>> {
        let osxx = OpenSimplex::new().set_seed(9);
        (1..128).map(|z| (1..128).map(|x| {
            (0..octaves).map(|oct| {
                let oct_coeff: f64 = (2.0_f64).powi(oct);
                let coeff = (1.0/64.0)*oct_coeff as f64;
                osxx.get([(x as f64)*coeff, (z as f64)*coeff])/oct_coeff
            }).fold(0.0 as f64, |a, b| a + b)
        }).collect()).collect()
    }

    display_timbre(gen_timbre_base(4));

    Ok(())
}
