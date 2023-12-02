advent_of_code::solution!(12);

struct Ship {
    x: i32,
    y: i32,
}

impl Ship {
    pub fn new(x: i32, y: i32) -> Ship {
        Ship { x, y }
    }

    pub fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn rotate(&self, angle: i32) -> Direction {
        Direction::from_angle(self.to_angle() + angle)
    }

    fn from_angle(angle: i32) -> Direction {
        match angle % 360 {
            0 => Direction::East,
            90 => Direction::South,
            180 => Direction::West,
            270 => Direction::North,
            _ => panic!("invalid angle"),
        }
    }

    fn to_angle(&self) -> i32 {
        match self {
            Direction::East => 0,
            Direction::South => 90,
            Direction::West => 180,
            Direction::North => 270,
        }
    }
}

struct Waypoint {
    x: i32,
    y: i32,
}

impl Waypoint {
    pub fn new(x: i32, y: i32) -> Waypoint {
        Waypoint { x, y }
    }

    pub fn rotate(&self, angle: i32) -> Waypoint {
        match angle {
            90 => Waypoint {
                x: self.y,
                y: -self.x,
            },
            180 => Waypoint {
                x: -self.x,
                y: -self.y,
            },
            270 => Waypoint {
                x: -self.y,
                y: self.x,
            },
            _ => unreachable!(),
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut ship = Ship::new(0, 0);
    let mut direction = Direction::East;

    for (inst, value) in input.lines().map(|line| {
        let (inst, value) = line.split_at(1);
        let value = value.parse().unwrap();
        (inst, value)
    }) {
        match inst {
            "N" => ship.y += value,
            "S" => ship.y -= value,
            "E" => ship.x += value,
            "W" => ship.x -= value,
            "L" => direction = direction.rotate(360 - value),
            "R" => direction = direction.rotate(value),
            "F" => match direction {
                Direction::North => ship.y += value,
                Direction::South => ship.y -= value,
                Direction::East => ship.x += value,
                Direction::West => ship.x -= value,
            },
            _ => unreachable!(),
        }
    }

    ship.manhattan_distance().to_string().into()
}

pub fn part_two(input: &str) -> Option<String> {
    let mut ship = Ship::new(0, 0);
    let mut waypoint = Waypoint::new(10, 1);

    for (inst, value) in input.lines().map(|line| {
        let (inst, value) = line.split_at(1);
        let value = value.parse().unwrap();
        (inst, value)
    }) {
        match inst {
            "N" => waypoint.y += value,
            "S" => waypoint.y -= value,
            "E" => waypoint.x += value,
            "W" => waypoint.x -= value,
            "L" => waypoint = waypoint.rotate(360 - value),
            "R" => waypoint = waypoint.rotate(value),
            "F" => {
                ship.x += value * waypoint.x;
                ship.y += value * waypoint.y;
            }
            _ => unreachable!(),
        }
    }

    ship.manhattan_distance().to_string().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part_one() {
        const INPUT: &str = indoc! {"
            F10
            N3
            F7
            R90
            F11
        "};
        let result = part_one(INPUT).unwrap();
        assert_eq!(result, "25");
    }

    #[test]
    fn test_part_two() {
        const INPUT: &str = indoc! {"
            F10
            N3
            F7
            R90
            F11
        "};
        let result = part_two(INPUT).unwrap();
        assert_eq!(result, "286");
    }
}
