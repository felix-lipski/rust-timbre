use noise::{OpenSimplex, NoiseFn, Seedable};
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let osx = OpenSimplex::new().set_seed(793874);
    println!("{:.4}", osx.get([0.1, 97.0]));
    println!("{:.4}", osx.get([0.1, 97.0, 0.0]));
    println!("{:.4}", osx.get([0.1, 97.0, 0.0, 0.0]));

    let root = BitMapBackend::new("out.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(-1f32..1f32, -1f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, osx.get([x as f64, 1.0]) as f32 )),
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
