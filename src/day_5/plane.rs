#[derive(Debug, PartialEq)]
pub struct BoardingPass {
    info: String,
}

impl BoardingPass {
    pub fn new(info: &str) -> BoardingPass {
        BoardingPass {
            info: String::from(info),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Seat {
    row: u32,
    col: u32,
    id: u32,
    occupied: bool,
}

impl Seat {
    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn is_occupied(&self) -> bool {
        self.occupied
    }

    fn new(row: u32, col: u32, number_of_cols: u32) -> Seat {
        Seat {
            row: row,
            col: col,
            id: row * number_of_cols + col,
            occupied: false,
        }
    }

    fn occupy(&mut self) {
        self.occupied = true;
    }
}

#[derive(Debug, PartialEq)]
pub struct Plane {
    seats: Vec<Vec<Seat>>,
    rows: u32,
    cols: u32,
}

impl Plane {
    pub fn new(rows: u32, cols: u32) -> Plane {
        let mut seats = Vec::new();

        for row in 0..rows {
            let mut temp_row = Vec::new();

            for col in 0..cols {
                temp_row.push(Seat::new(row, col, cols));
            }

            seats.push(temp_row);
        }

        Plane {
            seats: seats,
            rows: rows,
            cols: cols,
        }
    }

    pub fn get_seat_for(&self, boarding_pass: &BoardingPass) -> &Seat {
        let (row, col) = self.get_seat_location_for(boarding_pass);

        let row_error = format!("No Row At: {}", row);
        let col_error = format!("No Col At: {}", col);

        self.seats
            .get(row)
            .expect(row_error.as_str())
            .get(col)
            .expect(col_error.as_str())
    }

    pub fn fill(&mut self, boarding_passes: &[BoardingPass]) {
        for boarding_pass in boarding_passes {
            let (row, col) = self.get_seat_location_for(boarding_pass);

            let row_error = format!("No Row At: {}", row);
            let col_error = format!("No Col At: {}", col);

            self.seats
                .get_mut(row)
                .expect(row_error.as_str())
                .get_mut(col)
                .expect(col_error.as_str())
                .occupy();
        }
    }

    pub fn missing_seats(&self) -> Vec<&Seat> {
        self.seats
            .iter()
            .flatten()
            .filter(|seat| !seat.is_occupied())
            .collect()
    }

    pub fn surrounding_seats_occupied(&self, seat: &Seat) -> bool {
        let seat_id = seat.get_id();

        let left_seat_id = seat_id.checked_sub(1).unwrap_or(0);
        let (left_seat_row, left_seat_col) = self.get_seat_location_from_id(left_seat_id);
        let left_seat_occupied = if let Some(row) = self.seats.get(left_seat_row) {
            if let Some(left_seat) = row.get(left_seat_col) {
                left_seat.is_occupied()
            } else {
                false
            }
        } else {
            false
        };

        let right_seat_id = seat_id + 1;
        let (right_seat_row, right_seat_col) = self.get_seat_location_from_id(right_seat_id);
        let right_seat_occupied = if let Some(row) = self.seats.get(right_seat_row) {
            if let Some(right_seat) = row.get(right_seat_col) {
                right_seat.is_occupied()
            } else {
                false
            }
        } else {
            false
        };

        left_seat_occupied && right_seat_occupied
    }

    fn get_seat_location_from_id(&self, seat_id: u32) -> (usize, usize) {
        let row = seat_id / self.cols;
        let col = seat_id % self.cols;

        (row as usize, col as usize)
    }

    fn get_seat_location_for(&self, boarding_pass: &BoardingPass) -> (usize, usize) {
        let mut row_start = 0;
        let mut row_end = self.rows - 1;
        let mut col_start = 0;
        let mut col_end = self.cols - 1;

        for section in boarding_pass.info.chars() {
            match section {
                'F' => row_end = (row_end + row_start) / 2,
                'B' => row_start = (row_end + row_start) / 2 + 1,
                'R' => col_start = (col_end + col_start) / 2 + 1,
                'L' => col_end = (col_end + col_start) / 2,
                _ => panic!("Unknown Section: {}", section),
            }
        }

        (row_start as usize, col_end as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_BOARDING_PASSES: [&str; 4] =
        ["FBFBBFFRLR", "BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"];

    #[test]
    fn test_plane_new() {
        let plane = Plane::new(128, 8);

        let result_seat = plane.seats.get(44).unwrap().get(5).unwrap();

        let expected_seat = Seat {
            row: 44,
            col: 5,
            id: 357,
            occupied: false,
        };

        assert_eq!(plane.seats.len(), 128);
        assert_eq!(plane.seats.get(0).unwrap().len(), 8);
        assert_eq!(*result_seat, expected_seat);
    }

    #[test]
    fn test_plane_get_seat_for() {
        let plane = Plane::new(128, 8);

        let boarding_passes: Vec<BoardingPass> = TEST_BOARDING_PASSES
            .iter()
            .map(|info| BoardingPass::new(info))
            .collect();

        let result: Vec<Seat> = boarding_passes
            .iter()
            .map(|boarding_pass| plane.get_seat_for(boarding_pass).clone())
            .collect();

        let expected = vec![
            Seat::new(44, 5, 8),
            Seat::new(70, 7, 8),
            Seat::new(14, 7, 8),
            Seat::new(102, 4, 8),
        ];

        assert_eq!(result, expected);
    }
}
