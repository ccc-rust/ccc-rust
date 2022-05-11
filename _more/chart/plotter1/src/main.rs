use plotters::prelude::*;

fn curve(fname:&str, title:&str, f:fn(f32)->f32, x0:f32,x1:f32,y0:f32,y1:f32)->Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(fname, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(x0..x1, y0..y1)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (-500..=500).map(|x| x as f32 / 50.0).map(|x| (x, f(x))),
            &RED,
        ))?
        .label(title)
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));


    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

fn main() {
    let _=curve("images/x2.png", "y=x^2", |x| x*x, -1.0,1.0,-0.5,1.5);
    let _=curve("images/sin.png", "y=sin(x)", |x| x.sin(), -10.0,10.0,-1.5,1.5);
}
