use std::cmp::max;
use std::ops::AddAssign;

use ndarray::{Array2, s, ScalarOperand};
use num_traits::identities::One;
use num_traits::int::PrimInt;

use crate::rect::Rect;


pub struct Overlaps<'a, T> {
    count_grid: Array2<T>,
    rects: &'a [Rect],
}

pub type RectOverlaps<'a> = Overlaps<'a, u8>;


impl<'a, T> Overlaps<'a, T>
where
    T: 'a + AddAssign + One + PrimInt + ScalarOperand
{
    pub fn new(rects: &[Rect]) -> Overlaps<T> {
        // first, find the grid size required to fit in all the rects
        let mut grid_width: usize = 0;
        let mut grid_height: usize = 0;

        for rect in rects {
            let (x, y) = rect.top_right();

            // offset the fact that we start coords at 0
            grid_width = max(x as usize + 1, grid_width);
            grid_height = max(y as usize + 1, grid_height);
        }

        let mut overlaps = Overlaps {
            count_grid: Overlaps::make_count_grid(grid_width, grid_height),
            rects
        };

        overlaps.fill_grid(&rects);

        overlaps
    }

    fn make_count_grid(width: usize, height: usize) -> Array2<T> {
        Array2::<T>::zeros((width, height))
    }

    pub fn overlap_area(&self) -> usize {
        self.count_grid
            .map(|v: &T| if *v > T::one() { 1 } else { 0 })
            .sum()
    }

    pub fn iter_non_overlapping_rects(&self) -> impl Iterator<Item=&Rect> {
        // move = take ownership of enclosing scope
        let count = move |x, y| *self.count_grid.get((x as usize, y as usize)).unwrap();

        self.rects
            .iter()
            .filter(
                move |rect| {
                    rect.iter_coords()
                        .all(|(x, y)| count(x, y) == T::one())
                }
            )
    }

    fn fill_grid(&mut self, rects: &[Rect]) -> () {
        let one = T::one();

        for rect in rects {
            if rect.is_empty() {
                continue;
            }

            // s! indexer macro only accepts usize
            let (top_x, top_y) = rect.top_right();

            let rect_slice = s![rect.x as usize ..= top_x as usize,
                                rect.y as usize ..= top_y as usize];

            let mut slice = self.count_grid.slice_mut(rect_slice);

            // modify slice in-place, avoid potential overflows
            slice.map_mut(|v| *v = v.saturating_add(one));
        }
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::rect::{Rect, RectIDType};

    use super::RectOverlaps;

    #[test]
    fn test_overlaps() {
        let rects = [
            Rect { id: 1, x: 1, y: 3, width: 4, height: 4 },
            Rect { id: 2, x: 3, y: 1, width: 4, height: 4 },
            Rect { id: 3, x: 5, y: 5, width: 2, height: 2 },
        ];

        let overlaps = RectOverlaps::new(&rects);

        let area = overlaps.overlap_area();
        assert_eq!(area, 4);

        let expected: HashSet<RectIDType> = [3].iter().cloned().collect();

        let non_overlapping_ids: HashSet<RectIDType> =
            overlaps
            .iter_non_overlapping_rects()
            .map(|rect| rect.id)
            .collect();

        assert_eq!(non_overlapping_ids, expected);
    }

    #[test]
    fn test_overlaps_empty() {
        let rects = vec![];

        let overlaps = RectOverlaps::new(&rects);
        let area = overlaps.overlap_area();

        assert_eq!(area, 0);
    }

    #[test]
    fn test_overlaps_zero_size_rects() {
        let rects = vec![
            Rect { id: 1, x: 1, y: 1, width: 1, height: 1 },
            Rect { id: 2, x: 1, y: 1, width: 0, height: 0 },
        ];

        let overlaps = RectOverlaps::new(&rects);
        let area = overlaps.overlap_area();

        assert_eq!(area, 0);

        let non_overlapping: Vec<_> = overlaps
            .iter_non_overlapping_rects()
            .collect();

        assert_eq!(non_overlapping.len(), 2);
    }
}