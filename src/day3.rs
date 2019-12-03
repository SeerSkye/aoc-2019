use std::error::Error;
use std::fs;
use std::ops::{Add, Sub};

// A little sloppy today, but I also did most of this before I'd really woken up
pub fn day_3() {
    let file = fs::read_to_string("input/day3.txt").expect("Could not read file!");

    let input: Vec<_> = file.lines().collect();

    let path_1: Vec<_> = input[0]
        .split(',')
        .map(|x| Vector2d::from_problem_input(x).unwrap())
        .collect();
    let path_2: Vec<_> = input[1]
        .split(',')
        .map(|x| Vector2d::from_problem_input(x).unwrap())
        .collect();

    let segments_1 = segments(&path_1);
    let segments_2 = segments(&path_2);

    let mut intersections: Vec<Vector2d> = Vec::new();

    for p1_seg in &segments_1 {
        for p2_seg in &segments_2 {
            if let Some(vector) = p1_seg.intersection(p2_seg) {
                if vector.manhattan_distance() > 0 {
                    intersections.push(vector);
                }
            }
        }
    }

    let min_intersection = intersections
        .iter()
        .min_by(|x, y| x.manhattan_distance().cmp(&y.manhattan_distance()))
        .unwrap();

    println!(
        "The distance to the closest intersection is: {}",
        min_intersection.manhattan_distance()
    );

    let min_path_distance = intersections
        .iter()
        .map(|x| {
            distance_to_intersection(&segments_1, *x) + distance_to_intersection(&segments_2, *x)
        })
        .min()
        .unwrap();

    println!(
        "The path distance to the closest intersection is: {}",
        min_path_distance
    );
}

fn segments(path: &[Vector2d]) -> Vec<LineSegment> {
    let mut prev_point = Vector2d { x: 0, y: 0 };

    let mut result = Vec::new();

    for step in path {
        result.push(LineSegment {
            origin: prev_point,
            direction: *step,
        });
        prev_point = *step + prev_point;
    }

    result
}

fn distance_to_intersection(segments: &[LineSegment], intersection: Vector2d) -> i32 {
    let mut distance = 0;

    for segment in segments {
        match segment.intersection(&LineSegment {
            origin: intersection,
            direction: Vector2d { x: 0, y: 0 },
        }) {
            Some(_) => return distance + (intersection - segment.origin).manhattan_distance(),
            None => distance += segment.direction.manhattan_distance(),
        }
    }
    panic!(
        "Didn't reach intersection at ({}, {})!",
        intersection.x, intersection.y
    );
}

#[derive(Debug, PartialEq)]
struct LineSegment {
    origin: Vector2d,
    direction: Vector2d, // will always be axis aligned.
}

impl LineSegment {
    fn end_point(&self) -> Vector2d {
        self.origin + self.direction
    }

    fn intersection(&self, other: &LineSegment) -> Option<Vector2d> {
        // we can assume that intersections only happen at 90deg angles

        if self.direction.x == 0
            && other.direction.y == 0
            && self.origin.x >= other.origin.x.min(other.end_point().x)
            && self.origin.x <= other.origin.x.max(other.end_point().x)
            && other.origin.y >= self.origin.y.min(self.end_point().y)
            && other.origin.y <= self.origin.y.max(self.end_point().y)
        {
            return Some(Vector2d {
                x: self.origin.x,
                y: other.origin.y,
            });
        } else if self.direction.y == 0
            && other.direction.x == 0
            && self.origin.y >= other.origin.y.min(other.end_point().y)
            && self.origin.y <= other.origin.y.max(other.end_point().y)
            && other.origin.x >= self.origin.x.min(self.end_point().x)
            && other.origin.x <= self.origin.x.max(self.end_point().x)
        {
            return Some(Vector2d {
                x: other.origin.x,
                y: self.origin.y,
            });
        }
        None
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Vector2d {
    x: i32,
    y: i32,
}

impl Vector2d {
    fn manhattan_distance(self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn scale(self, scalar: i32) -> Vector2d {
        Vector2d {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    fn from_problem_input(input: &str) -> Result<Vector2d, Box<dyn Error>> {
        let mut chars = input.chars();

        let direction = chars.next().unwrap();

        let length: i32 = chars.as_str().parse()?;

        let dir_vec = match direction {
            'U' => Vector2d { x: 0, y: 1 },
            'D' => Vector2d { x: 0, y: -1 },
            'R' => Vector2d { x: 1, y: 0 },
            'L' => Vector2d { x: -1, y: 0 },
            _ => panic!("Invalid Direction"),
        };

        Ok(dir_vec.scale(length))
    }
}

impl Add for Vector2d {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector2d {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

//these tests were largely for debugging issues partway through
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test_1() {
        assert_eq!(
            Vector2d::from_problem_input("U234").unwrap(),
            Vector2d { x: 0, y: 234 }
        );
    }

    #[test]
    fn intersection_test() {
        let segment = LineSegment {
            origin: Vector2d { x: 217, y: 567 },
            direction: Vector2d { x: 0, y: -585 },
        };
        assert_eq!(
            segment.intersection(&LineSegment {
                origin: Vector2d { x: 217, y: 0 },
                direction: Vector2d { x: 0, y: 0 }
            }),
            Some(Vector2d { x: 217, y: 0 })
        );
    }
}
