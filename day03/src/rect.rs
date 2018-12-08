use std::cmp::min;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

use itertools::iproduct;
use lazy_static::lazy_static;
use regex::Regex;

use common::Fail;


pub type RectIDType = u32;


#[derive(Debug, PartialEq, Eq)]
pub struct Rect {
    pub id: RectIDType,
    pub x: u32,     // x, y : bottom left corner of the rect
    pub y: u32,
    pub width: u32,
    pub height: u32,
}


impl Rect {
    pub fn top_right(&self) -> (u32, u32) {
        // assuming the coordinate system starts at (0, 0)
        // and 0 sized rects are allowed
        let top_x = (self.x + self.width).saturating_sub(min(1, self.width));
        let top_y = (self.y + self.height).saturating_sub(min(1, self.height));

        (top_x, top_y)
    }

    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    pub fn iter_coords(&self) -> impl Iterator<Item=(u32, u32)> {
        let (max_x, max_y) = self.top_right();
        let empty = self.is_empty();

        // iter over x, then over y
        // e.g. (0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1)
        iproduct!(self.y ..= max_y, self.x ..= max_x)
            .filter(move |(_y, _x)| !empty)
            .map(|(y, x)| (x, y))
    }
}


#[derive(Debug, Fail)]
pub enum ParseRectError {
    #[fail(display = "Error parsing Rect from string: '{}'", string)]
    MalformedString {
        string: String
    },

    #[fail(display = "Error parsing Rect: integer overflow")]
    ParseInt(#[cause] ParseIntError)
}


impl From<ParseIntError> for ParseRectError {
    fn from(error: ParseIntError) -> Self {
        ParseRectError::ParseInt(error)
    }
}


impl FromStr for Rect {
    type Err = ParseRectError;

    fn from_str(s: &str) -> Result<Rect, ParseRectError> {
        lazy_static! {
            static ref PATTERN: Regex = Regex::new(
                r"#(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<width>\d+)x(?P<height>\d+)"
            ).expect("Invalid regex pattern");
        }

        match &PATTERN.captures(s) {
            // whole pattern + groups == 6 matches
            Some(caps) if caps.len() == 6 => {
                let extract = |name: &str| caps[name].parse();

                let rect = Rect {
                    id: extract("id")?, x: extract("x")?, y: extract("y")?,
                    width: extract("width")?, height: extract("height")?,
                };

                Ok(rect)
            }
            _ => {
                Err(ParseRectError::MalformedString { string: s.into() }.into())
            }
        }
    }
}


impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{} @ {},{}: {}x{}", self.id, self.x, self.y, self.width, self.height)
    }
}


#[cfg(test)]
mod tests {
    use super::{Rect};

    #[test]
    fn test_top_right() {
        let rect = Rect { id: 123, x: 3, y: 2, width: 5, height: 4 };

        assert_eq!(rect.top_right(), (7, 5));
    }

    #[test]
    fn test_top_right_size_zero() {
        let rect_at_0_0 = Rect { id: 1, x: 0, y: 0, width: 0, height: 0 };

        assert_eq!(rect_at_0_0.top_right(), (0, 0));

        let rect_at_10_10 = Rect { id: 1, x: 10, y: 10, width: 0, height: 0 };

        assert_eq!(rect_at_10_10.top_right(), (10, 10));
    }

    #[test]
    fn test_iter_coords() {
        let rect = Rect { id: 123, x: 5, y: 7, width: 3, height: 2 };
        let coords: Vec<_> = rect.iter_coords().collect();

        assert_eq!(
            coords,
            vec![(5, 7), (6, 7), (7, 7),
                 (5, 8), (6, 8), (7, 8),]
        );
    }

    #[test]
    fn test_iter_size_zero() {
        let rect = Rect { id: 1, x: 5, y: 7, width: 0, height: 0 };
        let coords: Vec<_> = rect.iter_coords().collect();

        assert!(rect.is_empty());
        assert!(coords.is_empty());
    }

    #[test]
    fn test_parsing() {
        let rect: Rect = "#1 @ 1,5: 9x100".parse().unwrap();

        assert_eq!(rect, Rect { id: 1, x: 1, y: 5, width: 9, height: 100 });
    }

    #[test]
    fn test_parsing_malformed() {
        let parse_result = "Elvish Pants".parse::<Rect>();

        assert!(parse_result.is_err());
        let err = parse_result.unwrap_err();

        assert_eq!(
            format!("{}", err), "Error parsing Rect from string: 'Elvish Pants'"
        );
    }

    #[test]
    fn test_parsing_int_overflow() {
        assert!(999999999 < u32::max_value() as u64);
        assert!(5000000000 > u32::max_value() as u64);

        let parse_no_overflow = "#1 @ 1,5: 8x999999999".parse::<Rect>();
        assert!(parse_no_overflow.is_ok());

        let parse_overflow = "#1 @ 1,5: 8x5000000000".parse::<Rect>();

        assert!(parse_overflow.is_err());
        let err = parse_overflow.unwrap_err();

        assert_eq!(
            format!("{}", err), "Error parsing Rect: integer overflow"
        );
    }

    #[test]
    fn test_parsing_and_display_are_symmetrical() {
        let rect = Rect { id: 42, x: 6, y: 20, width: 10, height: 9001 };

        let as_str = format!("{}", rect);
        assert_eq!(as_str, "#42 @ 6,20: 10x9001");

        assert_eq!(as_str.parse::<Rect>().unwrap(), rect);
    }
}
