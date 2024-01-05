use crate::point::Point;
use itertools::Itertools;
use num::{Integer, ToPrimitive};

/// Calculates the area enclosed by the loop defined by `loop_points`.
/// Requires that`loop_points` contains all integer points on the
/// loop's perimeter.
pub fn enclosed_area<V, I>(loop_points: &[V]) -> i32
where
    V: Sized + Point<I>,
    I: Integer + ToPrimitive,
{
    // Calculate the area enclosed by the loop
    // https://en.wikipedia.org/wiki/Shoelace_formula
    let mut area = loop_points
        .iter()
        .tuple_windows()
        .map(|(a, b)| {
            let x1 = a.x();
            let y1 = a.y();
            let x2 = b.x();
            let y2 = b.y();
            let p1 = x1 * y2;
            let p2 = y1 * x2;
            (p1 - p2).to_i32().unwrap()
        })
        .sum::<i32>()
        / 2;
    if area < 0 {
        area *= -1;
    }

    // Then calculate the number of points enclosed by the loop
    // loopArea - (boundaryPointsCount / 2) + 1
    // https://en.wikipedia.org/wiki/Pick's_theorem
    area - (loop_points.len() as i32 / 2) + 1
}

#[cfg(test)]
mod test {
    use super::*;

    struct Coord(i32, i32);

    impl Point<i32> for Coord {
        fn x(&self) -> i32 {
            self.0
        }
        fn y(&self) -> i32 {
            self.1
        }
    }

    #[test]
    fn test_it_calculates_enclosed_area_when_nothing_is_enclosed() {
        let points = vec![Coord(0, 0), Coord(1, 0), Coord(1, 1), Coord(0, 1)];
        assert_eq!(enclosed_area(&points), 0);
    }

    #[test]
    fn test_it_calculates_enclosed_area() {
        let points = vec![
            Coord(0, 0),
            Coord(1, 0),
            Coord(2, 0),
            Coord(2, 1),
            Coord(2, 2),
            Coord(1, 2),
            Coord(0, 2),
            Coord(0, 1),
        ];
        assert_eq!(enclosed_area(&points), 1);
    }
}
