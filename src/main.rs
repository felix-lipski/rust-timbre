use noise::{OpenSimplex, NoiseFn, Seedable};
use plotters::prelude::*;
use plotters::style::text_anchor::*;
use plotters_backend::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let osx = OpenSimplex::new().set_seed(793874);
    println!("{:.4}", osx.get([0.1, 97.0]));
    println!("{:.4}", osx.get([0.1, 97.0, 0.0]));
    println!("{:.4}", osx.get([0.1, 97.0, 0.0, 0.0]));

    let root = BitMapBackend::new("out.png", (1366, 768)).into_drawing_area();

    root.fill(&BLACK).unwrap();

    let fd = FontDesc::new(FontFamily::Monospace, 10.0 as f64, FontStyle::Normal);

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .build_cartesian_3d(-3.0..3.0, -3.0..3.0, -3.0..3.0)
        .unwrap();

    chart.configure_axes()
        .bold_grid_style(ShapeStyle {color: WHITE.to_rgba(), filled: true, stroke_width: 1})
    //  .light_grid_style(ShapeStyle {color: WHITE.to_rgba(), filled: true, stroke_width: 1})
    //  .axis_panel_style(ShapeStyle {color: BLACK.to_rgba(), filled: true, stroke_width: 1})
        .label_style(TextStyle {
            color: BackendColor {alpha: 1.0 as f64, rgb: (255, 255, 255) }, 
            pos: Pos {h_pos: HPos::Center, v_pos: VPos::Center}, 
            font: fd
        })
        .draw().unwrap();

    let mut data = vec![];

    for x in (-25..25).map(|v| v as f64 / 10.0) {
        let mut row = vec![];
        for z in (-25..25).map(|v| v as f64 / 10.0) {
            row.push((x, osx.get([x as f64, z as f64]) as f64, z));
        }
        data.push(row);
    }

    chart.draw_series(
        (0..49)
            .map(|x| std::iter::repeat(x).zip(0..49))
            .flatten()
            .map(|(x,z)| {
                Polygon::new(vec![
                    data[x][z],
                    data[x+1][z],
                    data[x+1][z+1],
                    data[x][z+1],
                ], &GREEN.mix(0.3))
            })
    ).unwrap();

    Ok(())
}
