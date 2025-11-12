use polars::prelude::*;

fn main() {
    let lf = LazyCsvReader::new(PlPath::new("dpwh_flood_control_projects.csv"))
        .with_has_header(true)
        .finish()?;
    //println!("{:?}", lf);
}