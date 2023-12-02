advent_of_code::solution!(11);

type SeatLayout = Vec<Vec<char>>;

fn parse(input: &str) -> SeatLayout {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn count_occupied_seats(seat_layout: &SeatLayout) -> usize {
    seat_layout.iter().flatten().filter(|&&c| c == '#').count()
}

fn count_occupied_adjacent_seats(seat_layout: &SeatLayout, row: usize, col: usize) -> usize {
    let rows = seat_layout.len();
    let cols = seat_layout[0].len();
    let mut occupied_count = 0;

    for r in row.saturating_sub(1)..std::cmp::min(row + 2, rows) {
        for c in col.saturating_sub(1)..std::cmp::min(col + 2, cols) {
            if r == row && c == col {
                continue;
            }
            if seat_layout[r][c] == '#' {
                occupied_count += 1;
            }
        }
    }

    occupied_count
}

fn count_occupied_visible_seats(seat_layout: &SeatLayout, row: usize, col: usize) -> usize {
    #[rustfmt::skip]
    let directions = [(-1,-1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    let row = row as isize;
    let col = col as isize;
    let rows = seat_layout.len() as isize;
    let cols = seat_layout[0].len() as isize;
    let mut occupied_count = 0;

    for (dir_r, dir_c) in directions.iter() {
        let mut r = row + dir_r;
        let mut c = col + dir_c;

        while r >= 0 && r < rows && c >= 0 && c < cols {
            let ri = r as usize;
            let ci = c as usize;

            if seat_layout[ri][ci] == '#' {
                occupied_count += 1;
                break;
            } else if seat_layout[ri][ci] == 'L' {
                break;
            }

            r += dir_r;
            c += dir_c;
        }
    }

    occupied_count
}

fn step_simulation_1(seat_layout: &SeatLayout) -> Option<SeatLayout> {
    let rows = seat_layout.len();
    let cols = seat_layout[0].len();
    let mut new_layout = seat_layout.clone();
    let mut modified = false;

    for r in 0..rows {
        for c in 0..cols {
            if seat_layout[r][c] == 'L' && count_occupied_adjacent_seats(seat_layout, r, c) == 0 {
                new_layout[r][c] = '#';
                modified = true;
            } else if seat_layout[r][c] == '#'
                && count_occupied_adjacent_seats(seat_layout, r, c) >= 4
            {
                new_layout[r][c] = 'L';
                modified = true;
            }
        }
    }

    if modified {
        Some(new_layout)
    } else {
        None
    }
}

fn simulate_seating_1(seat_layout: &SeatLayout) -> usize {
    let mut current_layout = seat_layout.clone();

    while let Some(new_layout) = step_simulation_1(&current_layout) {
        current_layout = new_layout;
    }

    count_occupied_seats(&current_layout)
}

fn step_simulation_2(seat_layout: &SeatLayout) -> Option<SeatLayout> {
    let rows = seat_layout.len();
    let cols = seat_layout[0].len();
    let mut new_layout = seat_layout.clone();
    let mut modified = false;

    for r in 0..rows {
        for c in 0..cols {
            if seat_layout[r][c] == 'L' && count_occupied_visible_seats(seat_layout, r, c) == 0 {
                new_layout[r][c] = '#';
                modified = true;
            } else if seat_layout[r][c] == '#'
                && count_occupied_visible_seats(seat_layout, r, c) >= 5
            {
                new_layout[r][c] = 'L';
                modified = true;
            }
        }
    }

    if modified {
        Some(new_layout)
    } else {
        None
    }
}

fn simulate_seating_2(seat_layout: &SeatLayout) -> usize {
    let mut current_layout = seat_layout.clone();

    while let Some(new_layout) = step_simulation_2(&current_layout) {
        current_layout = new_layout;
    }

    count_occupied_seats(&current_layout)
}

pub fn part_one(input: &str) -> Option<String> {
    let seat_layout = parse(input);
    let occupied_seats = simulate_seating_1(&seat_layout);
    occupied_seats.to_string().into()
}

pub fn part_two(input: &str) -> Option<String> {
    let seat_layout = parse(input);
    let occupied_seats = simulate_seating_2(&seat_layout);
    occupied_seats.to_string().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part_one() {
        const INPUT: &str = indoc! {"
            L.LL.LL.LL
            LLLLLLL.LL
            L.L.L..L..
            LLLL.LL.LL
            L.LL.LL.LL
            L.LLLLL.LL
            ..L.L.....
            LLLLLLLLLL
            L.LLLLLL.L
            L.LLLLL.LL
        "};
        let result = part_one(INPUT).unwrap();
        assert_eq!(result, "37");
    }

    #[test]
    fn test_part_two() {
        const INPUT: &str = indoc! {"
            L.LL.LL.LL
            LLLLLLL.LL
            L.L.L..L..
            LLLL.LL.LL
            L.LL.LL.LL
            L.LLLLL.LL
            ..L.L.....
            LLLLLLLLLL
            L.LLLLLL.L
            L.LLLLL.LL
        "};
        let result = part_two(INPUT).unwrap();
        assert_eq!(result, "26");
    }
}
