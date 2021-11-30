use crate::util::point_2d::Point2d;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
struct ImageTile {
    id: u64,
    up: String,
    right: String,
    down: String,
    left: String,
    data: Vec<String>,
}

impl ImageTile {
    fn new(info: &[String]) -> ImageTile {
        let id = info
            .get(0)
            .expect("No Id for ImageTile")
            .replace("Tile ", "")
            .replace(":", "")
            .parse()
            .expect("Cannot Parse Id for ImageTile");

        let up = info.get(1).expect("No Top Info for ImageTile").to_string();
        let down = info
            .last()
            .expect("No Bottom Info for ImageTile")
            .to_string();

        let left = info
            .iter()
            .skip(1)
            .map(|s| s.chars().nth(0).expect("No Left Info for ImageTile"))
            .collect();
        let right = info
            .iter()
            .skip(1)
            .map(|s| s.chars().nth_back(0).expect("No Right Info for ImageTile"))
            .collect();

        ImageTile {
            id,
            up,
            right,
            down,
            left,
            data: info.iter().skip(1).map(|line| line.clone()).collect(),
        }
    }

    fn rotate_right(&mut self) {
        let temp_up = self.left.chars().rev().collect();
        let temp_right = self.up.clone();
        let temp_down = self.right.chars().rev().collect();
        let temp_left = self.down.clone();

        let temp_data = (0..self.data[0].len())
            .map(|i| {
                self.data
                    .iter()
                    .rev()
                    .map(|line| line.chars().nth(i).unwrap())
                    .collect()
            })
            .collect();

        self.up = temp_up;
        self.right = temp_right;
        self.down = temp_down;
        self.left = temp_left;
        self.data = temp_data;
    }

    fn flip_over_vertical(&mut self) {
        let temp_up = self.up.chars().rev().collect();
        let temp_right = self.left.clone();
        let temp_down = self.down.chars().rev().collect();
        let temp_left = self.right.clone();

        self.up = temp_up;
        self.right = temp_right;
        self.down = temp_down;
        self.left = temp_left;

        self.data = self
            .data
            .iter()
            .map(|line| line.chars().rev().collect())
            .collect();
    }

    fn flip_over_horizontal(&mut self) {
        let temp_up = self.down.clone();
        let temp_right = self.right.chars().rev().collect();
        let temp_down = self.up.clone();
        let temp_left = self.left.chars().rev().collect();

        self.up = temp_up;
        self.right = temp_right;
        self.down = temp_down;
        self.left = temp_left;

        self.data.reverse();
    }

    fn get_all_forms_of_borders(&self) -> Vec<String> {
        vec![
            self.up.clone(),
            self.right.clone(),
            self.down.clone(),
            self.left.clone(),
            self.up.chars().rev().collect(),
            self.right.chars().rev().collect(),
            self.down.chars().rev().collect(),
            self.left.chars().rev().collect(),
        ]
    }

    fn matches_all(&self, up: &Option<String>, left: &Option<String>) -> bool {
        up.clone().unwrap_or(self.up.clone()) == self.up
            && left.clone().unwrap_or(self.left.clone()) == self.left
    }
}

#[derive(Debug, PartialEq)]
pub struct Image {
    unorganized_tiles: HashMap<u64, ImageTile>,
    image: Option<HashMap<Point2d<i32>, ImageTile>>,
}

impl Image {
    const TOP_OFFSET: Point2d<i32> = Point2d { x: 0, y: -1 };
    const LEFT_OFFSET: Point2d<i32> = Point2d { x: -1, y: 0 };

    pub fn new(info: &[String]) -> Image {
        let unorganized_tiles = info
            .split(|s| s.is_empty())
            .filter(|chunk| !chunk.is_empty())
            .map(|chunk| {
                let tile = ImageTile::new(chunk);

                (tile.id, tile)
            })
            .collect();

        let image = None;

        Image {
            unorganized_tiles,
            image,
        }
    }

    pub fn build_image(&mut self) {
        let mut image = HashMap::new();

        let all_borders = self.get_all_forms_of_borders_to_tiles();

        let mut current_point = Point2d::new(0, 0);
        let top_left_tile = self.get_top_left_corner_tile();

        image.insert(current_point, top_left_tile);

        let mut y = 0;

        loop {
            let mut x = 0;

            loop {
                let mut current_tile_id = None;

                if (y == 0) && (x == 0) {
                    x += 1;

                    continue;
                }

                current_point = Point2d::new(x, y);

                let top_neighbor = image.get(&(current_point + Self::TOP_OFFSET));
                let top_neighbor_down_border = top_neighbor.map(|tile| tile.down.clone());
                let top_is_unique = if y == 0 { true } else { false };

                let left_neighbor = image.get(&(current_point + Self::LEFT_OFFSET));
                let left_neighbor_right_border = left_neighbor.map(|tile| tile.right.clone());
                let left_is_unique = if x == 0 { true } else { false };

                if let Some(top_neighbor_tile) = top_neighbor {
                    current_tile_id = self.get_tile_id_that_boarders(
                        top_neighbor_tile.id,
                        &top_neighbor_tile.down,
                        &all_borders,
                    );
                } else if let Some(left_neighbor_tile) = left_neighbor {
                    current_tile_id = self.get_tile_id_that_boarders(
                        left_neighbor_tile.id,
                        &left_neighbor_tile.right,
                        &all_borders,
                    );
                }

                if let Some(id) = current_tile_id {
                    let mut tile = self.unorganized_tiles.get(&id).unwrap().clone();

                    self.orient_tile_to_match_borders(
                        &all_borders,
                        &mut tile,
                        &top_neighbor_down_border,
                        top_is_unique,
                        &left_neighbor_right_border,
                        left_is_unique,
                    );

                    image.insert(current_point, tile.clone());
                } else {
                    break;
                }

                x += 1;
            }

            if image.len() == self.unorganized_tiles.len() {
                break;
            }

            y += 1;
        }

        self.image = Some(image);
    }

    pub fn get_corner_tile_ids(&self) -> Vec<u64> {
        let all_borders = self.get_all_forms_of_borders_to_tiles();

        let mut corners = Vec::new();

        for tile in self.unorganized_tiles.values() {
            let mut number_of_unique_borders = 0;

            if all_borders.get(&tile.up).expect("No Border Found!").len() == 1 {
                number_of_unique_borders += 1;
            }

            if all_borders
                .get(&tile.right)
                .expect("No Border Found!")
                .len()
                == 1
            {
                number_of_unique_borders += 1;
            }

            if all_borders.get(&tile.down).expect("No Border Found!").len() == 1 {
                number_of_unique_borders += 1;
            }

            if all_borders.get(&tile.left).expect("No Border Found!").len() == 1 {
                number_of_unique_borders += 1;
            }

            if number_of_unique_borders == 2 {
                corners.push(tile.id);
            }
        }

        corners.sort();

        corners
    }

    fn get_all_forms_of_borders_to_tiles(&self) -> HashMap<String, Vec<u64>> {
        let mut result = HashMap::new();

        for tile in self.unorganized_tiles.values() {
            for border in tile.get_all_forms_of_borders() {
                result.entry(border).or_insert(Vec::new()).push(tile.id);
            }
        }

        result
    }

    fn get_top_left_corner_tile(&self) -> ImageTile {
        let all_borders = self.get_all_forms_of_borders_to_tiles();
        let corners = self.get_corner_tile_ids();

        let id = corners.get(0).expect("Empty Corner Tiles!");

        let mut tile = self
            .unorganized_tiles
            .get(&id)
            .expect("No Top Left Tile Found")
            .clone();

        self.orient_tile_to_match_borders(&all_borders, &mut tile, &None, true, &None, true);

        tile
    }

    fn get_tile_id_that_boarders(
        &self,
        tile_id: u64,
        border: &str,
        all_borders: &HashMap<String, Vec<u64>>,
    ) -> Option<u64> {
        all_borders
            .get(border)
            .unwrap()
            .iter()
            .filter(|id| **id != tile_id)
            .nth(0)
            .map(|id| *id)
    }

    fn orient_tile_to_match_borders(
        &self,
        all_borders: &HashMap<String, Vec<u64>>,
        tile: &mut ImageTile,
        up: &Option<String>,
        up_is_unique: bool,
        left: &Option<String>,
        left_is_unique: bool,
    ) {
        let mut matches_all = tile.matches_all(up, left);

        // check match
        for _ in 0..4 {
            matches_all = tile.matches_all(up, left);

            if !matches_all {
                tile.flip_over_horizontal();

                matches_all = tile.matches_all(up, left);

                if !matches_all {
                    tile.flip_over_horizontal();
                }
            }

            if !matches_all {
                tile.flip_over_vertical();

                matches_all = tile.matches_all(up, left);

                if !matches_all {
                    tile.flip_over_vertical();
                }
            }

            if matches_all {
                break;
            }

            tile.rotate_right();
        }

        let mut matches_unique =
            Self::tile_matches_unique(tile, all_borders, up_is_unique, left_is_unique);

        // check unique
        for _ in 0..4 {
            matches_unique =
                Self::tile_matches_unique(tile, all_borders, up_is_unique, left_is_unique);

            if !matches_unique {
                tile.flip_over_horizontal();

                matches_unique =
                    Self::tile_matches_unique(tile, all_borders, up_is_unique, left_is_unique);

                if !matches_unique {
                    tile.flip_over_horizontal();
                }
            }

            if !matches_unique {
                tile.flip_over_vertical();

                matches_unique =
                    Self::tile_matches_unique(tile, all_borders, up_is_unique, left_is_unique);

                if !matches_unique {
                    tile.flip_over_vertical();
                }
            }

            if matches_unique {
                break;
            }

            tile.rotate_right();
        }

        if !matches_all || !matches_unique {
            panic!(
                "Tile {} Cannot Match: {:?}, {:?}, {:?}, {:?}",
                tile.id, up, up_is_unique, left, left_is_unique
            );
        }
    }

    fn tile_matches_unique(
        tile: &ImageTile,
        all_borders: &HashMap<String, Vec<u64>>,
        up_is_unique: bool,
        left_is_unique: bool,
    ) -> bool {
        let mut matches = true;

        if up_is_unique {
            matches &= all_borders.get(&tile.up).unwrap().len() == 1;
        }

        if left_is_unique {
            matches &= all_borders.get(&tile.left).unwrap().len() == 1;
        }

        matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [&str; 108] = [
        "Tile 2311:",
        "..##.#..#.",
        "##..#.....",
        "#...##..#.",
        "####.#...#",
        "##.##.###.",
        "##...#.###",
        ".#.#.#..##",
        "..#....#..",
        "###...#.#.",
        "..###..###",
        "",
        "Tile 1951:",
        "#.##...##.",
        "#.####...#",
        ".....#..##",
        "#...######",
        ".##.#....#",
        ".###.#####",
        "###.##.##.",
        ".###....#.",
        "..#.#..#.#",
        "#...##.#..",
        "",
        "Tile 1171:",
        "####...##.",
        "#..##.#..#",
        "##.#..#.#.",
        ".###.####.",
        "..###.####",
        ".##....##.",
        ".#...####.",
        "#.##.####.",
        "####..#...",
        ".....##...",
        "",
        "Tile 1427:",
        "###.##.#..",
        ".#..#.##..",
        ".#.##.#..#",
        "#.#.#.##.#",
        "....#...##",
        "...##..##.",
        "...#.#####",
        ".#.####.#.",
        "..#..###.#",
        "..##.#..#.",
        "",
        "Tile 1489:",
        "##.#.#....",
        "..##...#..",
        ".##..##...",
        "..#...#...",
        "#####...#.",
        "#..#.#.#.#",
        "...#.#.#..",
        "##.#...##.",
        "..##.##.##",
        "###.##.#..",
        "",
        "Tile 2473:",
        "#....####.",
        "#..#.##...",
        "#.##..#...",
        "######.#.#",
        ".#...#.#.#",
        ".#########",
        ".###.#..#.",
        "########.#",
        "##...##.#.",
        "..###.#.#.",
        "",
        "Tile 2971:",
        "..#.#....#",
        "#...###...",
        "#.#.###...",
        "##.##..#..",
        ".#####..##",
        ".#..####.#",
        "#..#.#..#.",
        "..####.###",
        "..#.#.###.",
        "...#.#.#.#",
        "",
        "Tile 2729:",
        "...#.#.#.#",
        "####.#....",
        "..#.#.....",
        "....#..#.#",
        ".##..##.#.",
        ".#.####...",
        "####.#.#..",
        "##.####...",
        "##..#.##..",
        "#.##...##.",
        "",
        "Tile 3079:",
        "#.#.#####.",
        ".#..######",
        "..#.......",
        "######....",
        "####.#..#.",
        ".#...#.##.",
        "#.#####.##",
        "..#.###...",
        "..#.......",
        "..#.###...",
        "",
    ];

    #[test]
    fn test_image_tile_new() {
        let input = to_vec_string(&TEST_DATA[0..11]);

        let result = ImageTile::new(&input);

        let expected_data = to_vec_string(&TEST_DATA[1..11]);

        let expected = ImageTile {
            id: 2311,
            up: "..##.#..#.".to_string(),
            right: "...#.##..#".to_string(),
            down: "..###..###".to_string(),
            left: ".#####..#.".to_string(),
            data: expected_data,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_image_tile_rotate_right() {
        let input = to_vec_string(&TEST_DATA[0..11]);

        let mut tile = ImageTile::new(&input);

        tile.rotate_right();

        let expected_data = vec![
            ".#..#####.".to_string(),
            ".#.####.#.".to_string(),
            "###...#..#".to_string(),
            "#..#.##..#".to_string(),
            "#....#.##.".to_string(),
            "...##.##.#".to_string(),
            ".#...#....".to_string(),
            "#.#.##....".to_string(),
            "##.###.#.#".to_string(),
            "#..##.#...".to_string(),
        ];

        let expected = ImageTile {
            id: 2311,
            up: ".#####..#.".chars().rev().collect(),
            right: "..##.#..#.".to_string(),
            down: "...#.##..#".chars().rev().collect(),
            left: "..###..###".to_string(),
            data: expected_data,
        };

        tile.rotate_right();
        tile.rotate_right();
        tile.rotate_right();
        tile.rotate_right();

        assert_eq!(tile, expected);
    }

    #[test]
    fn test_image_tile_flip_over_vertical() {
        let input = to_vec_string(&TEST_DATA[0..11]);

        let mut tile = ImageTile::new(&input);

        tile.flip_over_vertical();

        let expected_data = to_vec_string(&TEST_DATA[1..11])
            .into_iter()
            .map(|line| line.chars().rev().collect())
            .collect();

        let expected = ImageTile {
            id: 2311,
            up: "..##.#..#.".chars().rev().collect(),
            right: ".#####..#.".to_string(),
            down: "..###..###".chars().rev().collect(),
            left: "...#.##..#".to_string(),
            data: expected_data,
        };

        tile.flip_over_vertical();
        tile.flip_over_vertical();

        assert_eq!(tile, expected);
    }

    #[test]
    fn test_image_tile_flip_over_horizontal() {
        let input = to_vec_string(&TEST_DATA[0..11]);

        let mut tile = ImageTile::new(&input);

        tile.flip_over_horizontal();

        let mut expected_data = to_vec_string(&TEST_DATA[1..11]);

        expected_data.reverse();

        let expected = ImageTile {
            id: 2311,
            up: "..###..###".to_string(),
            right: "...#.##..#".chars().rev().collect(),
            down: "..##.#..#.".to_string(),
            left: ".#####..#.".chars().rev().collect(),
            data: expected_data,
        };

        tile.flip_over_horizontal();
        tile.flip_over_horizontal();

        assert_eq!(tile, expected);
    }

    #[test]
    fn test_image_tile_get_all_forms_of_borders() {
        let input = to_vec_string(&TEST_DATA[0..11]);

        let tile = ImageTile::new(&input);

        let result = tile.get_all_forms_of_borders();

        let expected = vec![
            "..##.#..#.".to_string(),
            "...#.##..#".to_string(),
            "..###..###".to_string(),
            ".#####..#.".to_string(),
            "..##.#..#.".chars().rev().collect(),
            "...#.##..#".chars().rev().collect(),
            "..###..###".chars().rev().collect(),
            ".#####..#.".chars().rev().collect(),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_image_new() {
        let input = to_vec_string(&TEST_DATA);

        let result = Image::new(&input);

        let result_tile = result.unorganized_tiles.get(&2311).unwrap().clone();

        let expected_tiles = input
            .split(|s| s.is_empty())
            .filter(|chunk| !chunk.is_empty())
            .map(|chunk| {
                let tile = ImageTile::new(chunk);

                (tile.id, tile)
            })
            .collect();

        let expected = Image {
            unorganized_tiles: expected_tiles,
            image: None,
        };

        let expected_tile = ImageTile::new(&to_vec_string(&TEST_DATA[0..11]));

        assert_eq!(result, expected);
        assert_eq!(result_tile, expected_tile);
    }

    #[test]
    fn test_image_build_image() {
        let input = to_vec_string(&TEST_DATA);

        let mut image = Image::new(&input);

        image.build_image();

        let result = image_id_based(&image);

        let expected: HashMap<Point2d<i32>, u64> = vec![
            (Point2d::new(0, 0), 1171),
            (Point2d::new(1, 0), 1489),
            (Point2d::new(2, 0), 2971),
            (Point2d::new(0, 1), 2473),
            (Point2d::new(1, 1), 1427),
            (Point2d::new(2, 1), 2729),
            (Point2d::new(0, 2), 3079),
            (Point2d::new(1, 2), 2311),
            (Point2d::new(2, 2), 1951),
        ]
        .into_iter()
        .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_image_get_corner_tiles() {
        let input = to_vec_string(&TEST_DATA);

        let image = Image::new(&input);

        let result = image.get_corner_tile_ids();

        let expected = vec![1171, 1951, 2971, 3079];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_image_get_top_left_corner_tile_id() {
        let input = to_vec_string(&TEST_DATA);

        let image = Image::new(&input);

        let result = image.get_top_left_corner_tile();

        let mut expected_data = to_vec_string(&TEST_DATA[25..35]);

        expected_data.reverse();

        let expected = ImageTile {
            id: 1171,
            up: ".....##...".to_string(),
            right: ".....#..#.".to_string(),
            down: "####...##.".to_string(),
            left: ".##....###".to_string(),
            data: expected_data,
        };

        assert_eq!(result, expected);
    }

    fn to_vec_string(input: &[&str]) -> Vec<String> {
        input.iter().map(|s| s.to_string()).collect()
    }

    fn image_id_based(image: &Image) -> HashMap<Point2d<i32>, u64> {
        if let Some(built_image) = &image.image {
            built_image.iter().map(|(k, v)| (k.clone(), v.id)).collect()
        } else {
            HashMap::new()
        }
    }
}
