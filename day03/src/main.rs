use std::env;

use common::{get_input, Error};

mod rect;
mod overlaps;

use crate::rect::Rect;
use crate::overlaps::RectOverlaps;


fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let input = get_input(&args);

    if input.is_err() {
        eprintln!("Error: {}. Aborting.", input.unwrap_err());
        std::process::exit(1);
    }

    let rects: Vec<Rect> = input.unwrap();

    let overlaps = RectOverlaps::new(&rects);

    // part 1

    println!("Overlap area: {}", overlaps.overlap_area());

    // part 2
    let non_overlap_ids: Vec<_> = overlaps
        .iter_non_overlapping_rects()
        .map(|rect| rect.id)
        .collect();

    match non_overlap_ids.len() {
        0 => println!("No overlaps"),
        1 => println!("Non overlapping claim ID: {}", non_overlap_ids[0]),
        _ => println!("Non overlapping claim IDs: {:?}", non_overlap_ids),
    }

    Ok(())
}
