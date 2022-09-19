use plotters::prelude::*;
use wasm_bindgen::prelude::*;

fn pdf(x: f64, y: f64) -> f64 {
    const SDX: f64 = 0.1;
    const SDY: f64 = 0.1;
    const A: f64 = 5.0;
    let x = x as f64 / 10.0;
    let y = y as f64 / 10.0;
    A * (-x * x / 2.0 / SDX / SDX - y * y / 2.0 / SDY / SDY).exp()
}

#[wasm_bindgen(start)]
pub async fn req() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let target_elem = document.get_element_by_id("target").unwrap();

    let mut buf = String::new();
    {
        let area = SVGBackend::with_string(&mut buf, (1024, 760)).into_drawing_area();
        area.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&area)
            .caption("2D Gaussian PDF", ("sans-serif", 20))
            .build_cartesian_3d(-3.0..3.0, 0.0..6.0, -3.0..3.0)
            .unwrap();

        chart.with_projection(|mut p| {
            p.pitch = 1.57 - (1.57 - 20f64 / 50.0).abs();
            p.scale = 0.7;
            p.into_matrix()
        });

        chart
            .configure_axes()
            .light_grid_style(BLACK.mix(0.15))
            .max_light_lines(3)
            .draw()
            .unwrap();

        chart.draw_series(
            SurfaceSeries::xoz(
                (-15..=15).map(|x| x as f64 / 5.0),
                (-15..=15).map(|x| x as f64 / 5.0),
                pdf,
            )
            .style_func(&|&v| {
                (&HSLColor(240.0 / 360.0 - 240.0 / 360.0 * v / 5.0, 1.0, 0.7)).into()
            }),
        ).unwrap();

        area.present().unwrap();
    }

    target_elem.set_inner_html(&buf);
}
