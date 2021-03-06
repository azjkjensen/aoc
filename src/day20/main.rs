use eyre::Result;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
enum TileSide {
    Top,
    Bottom,
    Left,
    Right,
    None,
}

impl TileSide {
    fn opposite(&self) -> Self {
        match self {
            TileSide::Top => TileSide::Bottom,
            TileSide::Bottom => TileSide::Top,
            TileSide::Left => TileSide::Right,
            TileSide::Right => TileSide::Left,
            TileSide::None => TileSide::None,
        }
    }
}

#[derive(Debug, Clone)]
struct Tile {
    tile_num: usize,
    vals: Vec<Vec<char>>,
    /// matching side, other tile number, other tile matching side
    matches: HashMap<TileSide, (usize, TileSide)>,
}

impl Tile {
    fn new(tile_num: usize, vals: Vec<Vec<char>>) -> Self {
        Tile {
            tile_num,
            vals,
            matches: HashMap::new(),
        }
    }

    /// remove edges where there are matches, return the resulting 2d array
    fn strip_edges(&self) -> Vec<Vec<char>> {
        let mut new_vals = self.vals.clone();
        for match_side in self.matches.keys() {
            new_vals = match match_side {
                TileSide::Top => new_vals[1..].to_vec(),
                TileSide::Bottom => new_vals[..new_vals.len() - 1].to_vec(),
                TileSide::Left => new_vals.iter().map(|line| line[1..].to_vec()).collect(),
                TileSide::Right => new_vals
                    .iter()
                    .map(|line| line[..line.len() - 1].to_vec())
                    .collect(),
                _ => new_vals,
            }
        }
        new_vals
    }

    fn side_vals(&self) -> Vec<char> {
        self.vals.iter().map(|x| *x.last().unwrap()).collect()
    }

    fn rotate(&mut self) -> &mut Self {
        let mut new_vals = self.vals.clone();
        for i in 0..self.vals.len() {
            for j in 0..self.vals[0].len() {
                new_vals[j][self.vals[0].len() - i - 1] = self.vals[i][j];
            }
        }
        self.vals = new_vals;
        self
    }

    fn flip(&mut self) -> &mut Self {
        let mut new_vals = vec![];
        for v in self.vals.iter().rev() {
            new_vals.push(v.clone());
        }
        self.vals = new_vals;
        self
    }

    fn variants(&mut self) -> Vec<Tile> {
        let mut flipped = self.clone();
        flipped.flip();
        vec![
            self.clone(),
            self.rotate().clone(),
            self.rotate().clone(),
            self.rotate().clone(),
            self.rotate().clone(),
            flipped.clone(),
            flipped.rotate().clone(),
            flipped.rotate().clone(),
            flipped.rotate().clone(),
            flipped.rotate().clone(),
        ]
    }
}

impl IntoIterator for Tile {
    type Item = (TileSide, Vec<char>);
    type IntoIter = TileSidesIter;

    fn into_iter(self) -> Self::IntoIter {
        TileSidesIter {
            tile: self,
            side: TileSide::Top,
        }
    }
}

struct TileSidesIter {
    tile: Tile,
    side: TileSide,
}

impl Iterator for TileSidesIter {
    type Item = (TileSide, Vec<char>);

    fn next(&mut self) -> Option<(TileSide, Vec<char>)> {
        match self.side {
            TileSide::Top => {
                self.side = TileSide::Bottom;
                Some((TileSide::Top, self.tile.vals[0].clone()))
            }
            TileSide::Bottom => {
                self.side = TileSide::Left;
                self.tile
                    .vals
                    .iter()
                    .map(|x| x.clone())
                    .last()
                    .map(|res| (TileSide::Bottom, res))
            }
            TileSide::Left => {
                self.side = TileSide::Right;
                let left_vals = self.tile.vals.iter().map(|x| x[0]).collect();
                Some((TileSide::Left, left_vals))
            }
            TileSide::Right => {
                self.side = TileSide::None;
                let right_vals = self.tile.vals.iter().map(|x| *x.last().unwrap()).collect();
                Some((TileSide::Right, right_vals))
            }
            TileSide::None => None,
        }
    }
}

fn find_matching_side(tile1: &mut Tile, tile2: &mut Tile) -> Option<()> {
    let side_vals = tile1.side_vals();
    for _ in 0..4 {
        if side_vals == tile2.side_vals() {
            println!("ffound reg");
            return Some(());
        }
        tile2.rotate();
    }
    tile2.flip();
    if side_vals == tile2.side_vals() {
        println!("ffound ffliipped");
        return Some(());
    }
    for _ in 0..4 {
        if side_vals == tile2.side_vals() {
            println!("ffound ffliipped");
            return Some(());
        }
        tile2.rotate();
    }
    None
}

//fn assemble_from_corner(tiles: &HashMap<usize, Tile>, starting_corner: &Tile) -> Vec<Vec<char>> {
//    let mut first_tile_in_row = starting_corner;
//    let mut tile_matches_iter = starting_corner.matches.values();
//
//    let (mut right_tile_num, mut right_direction) = tile_matches_iter.next().unwrap();
//    let (mut down_tile_num, mut down_direction) = tile_matches_iter.next().unwrap();
//    let mut current_tile = first_tile_in_row;
//
//    let mut result: Vec<Vec<_>> = starting_corner.strip_edges();
//    let mut corner_count = 0;
//    while corner_count < 4 {
//        println!(
//            "hitting tile {} and going {:?}",
//            current_tile.tile_num, right_direction
//        );
//        if current_tile.matches.len() == 2 {
//            corner_count += 1;
//        }
//        if let Some((next_tile_num, next_tile_side)) = current_tile
//            .matches
//            .values()
//            .filter(|(_, side)| *side == right_direction)
//            .next()
//        {
//            println!("matched with {}:{:?}", next_tile_num, next_tile_side);
//            // keep going in that direction
//            right_direction = next_tile_side.opposite();
//            current_tile = tiles.get(&next_tile_num).unwrap();
//            right_tile_num = *next_tile_num;
//        } else {
//            // next row
//            if let Some((next_tile_num, side_moved)) = first_tile_in_row
//                .matches
//                .values()
//                .filter(|(_, side)| *side == down_direction)
//                .next()
//            {
//                current_tile = tiles.get(&next_tile_num).unwrap();
//                first_tile_in_row = current_tile;
//                let right_direction = current_tile
//                    .matches
//                    .values()
//                    .filter(|(_, side)| {
//                        *side != down_direction && *side != down_direction.opposite()
//                    })
//                    .next()
//                    .unwrap();
//                down_direction = side_moved.opposite();
//            } else {
//                // fin
//                break;
//            }
//        }
//    }
//    result
//}

fn main() -> Result<()> {
    let input = read_to_string("src/day20/input-sample.txt")?;
    let mut tiles = HashMap::new();
    for piece_def in input.split("\n\n") {
        let mut piece_lines = piece_def.lines().filter(|&x| !x.is_empty());
        let tile_num = piece_lines
            .next()
            .unwrap()
            .strip_prefix("Tile ")
            .unwrap()
            .strip_suffix(":")
            .unwrap()
            .parse::<usize>()?;
        //println!("tile num: {}", tile_num);
        let tile: Vec<Vec<char>> = piece_lines.map(|line| line.chars().collect()).collect();
        //println!("tile: {:?}", tile);
        tiles.insert(tile_num, Tile::new(tile_num, tile));
    }
    //println!("all tiles: {:?}", tiles);
    let final_count = tiles
        .clone()
        .iter()
        .map(|(tile_num, tile)| {
            let mut count = 0;
            for (other_tile_num, mut other_tile) in &mut tiles {
                if *tile_num == *other_tile_num {
                    continue;
                }
                if let Some(_) = find_matching_side(&mut tile.clone(), &mut other_tile) {
                    count += 1;
                }
            }
            (*tile_num, count)
        })
        .filter(|(_, count)| *count == 2)
        .fold(1, |acc, (x, _)| acc * x);
    println!("final: {:?}", final_count);

    // part 2
    //
    // compose actual board, removing borders
    let corners: HashMap<usize, usize> = tiles
        .clone()
        .iter_mut()
        .map(|(tile_num, tile)| {
            let mut count = 0;
            for (other_tile_num, other_tile) in &mut tiles.clone() {
                if *tile_num == *other_tile_num {
                    continue;
                }
                for mut variant in tile.variants() {
                    if let Some(_) = find_matching_side(&mut variant, other_tile) {
                        count += 1;
                        tiles.insert(*tile_num, variant.clone());
                        tiles.insert(*other_tile_num, other_tile.clone());
                        continue;
                    }
                }
                //if *tile_num == *other_tile_num {
                //    continue;
                //}
                //for _ in 0..4 {
                //    if let Some(_) = find_matching_side(&mut tile.clone(), other_tile) {
                //        count += 1;
                //        tiles.insert(*tile_num, tile.clone());
                //        tiles.insert(*other_tile_num, other_tile.clone());
                //        continue;
                //    }
                //    tile.rotate();
                //}
                //tile.flip();
                //if let Some(_) = find_matching_side(&mut tile.clone(), other_tile) {
                //    count += 1;
                //    continue;
                //}
                //for _ in 0..4 {
                //    if let Some(_) = find_matching_side(&mut tile.clone(), other_tile) {
                //        count += 1;
                //        continue;
                //    }
                //    tile.rotate();
                //}
            }
            (*tile_num, count)
        })
        .filter(|(_, count)| *count == 2)
        .collect();

    println!("tiles: {:?}", tiles);
    //println!(" {:?}", tiles.get(&tile_num).unwrap());
    //let assembled_puzzle = assemble_from_corner(&tiles, tiles.get(&tile_num).unwrap());
    for piece in tiles {
        println!("{}", piece.0);
        for l in piece.1.vals {
            println!("{:?}", l);
        }
        println!();
    }

    Ok(())
}
