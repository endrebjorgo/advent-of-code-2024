use std::fs::File;
use std::io::Read;
use std::time::Instant;

#[derive(Debug, Default)]
struct Maze {
    start_pos: (usize, usize),
    start_dir: (i32, i32),
    goal: (usize, usize),
    walls: Vec<(usize, usize)>,
}

impl Maze {
    fn from_file(file_path: &str) -> Self {
        let mut maze = Maze::default();

        let mut buf = Vec::new();
        let mut f = File::open(file_path).unwrap();
        f.read_to_end(&mut buf).unwrap();

        let mut row = 0;
        let mut col = 0;

        for &c in buf.iter() {
            if c == b'\n' {
                row += 1;
                col = 0;
                continue;
            }

            match c {
                b'#' => { maze.walls.push((col, row)); },
                b'S' => { maze.start_pos = (col, row); },
                b'E' => { maze.goal = (col, row); },
                b'.' => {},
                _ => unreachable!(),
            }
            col += 1;
        }
        maze.start_dir = (1, 0);
        maze
    }

    fn heuristic(&self, pos: (usize, usize)) -> usize {
        let dx = (self.goal.0 as i32 - pos.0 as i32).abs() as usize;
        let dy = (self.goal.1 as i32 - pos.1 as i32).abs() as usize;
        return dx + dy + 2000; // Manhattan distance plus 2 turns
    }

    fn get_cheapest_path(&self) -> Option<usize> {
        let mut pq: PQueue<((usize, usize), (i32, i32))> = PQueue::new();
        pq.insert(
            (self.start_pos, self.start_dir),
            0,
            self.heuristic(self.start_pos)
        );
        
        let mut visited: Vec<((usize, usize), (i32, i32))> = Vec::new();

        while let Some((e, cost)) = pq.pop() {
            let curr_pos = e.0;
            let curr_dir = e.1;

            if curr_pos == self.goal { return Some(cost); }

            for x_off in -1..=1 {
                for y_off in -1..=1 {
                    if (x_off == 0) == (y_off == 0) { continue; }

                    // Should not panic as pos is never on edge
                    let new_x = (curr_pos.0 as i32 + x_off) as usize;
                    let new_y = (curr_pos.1 as i32 + y_off) as usize;
                    let new_pos = (new_x, new_y);

                    if self.walls.contains(&new_pos) { continue; }

                    let new_dir = (x_off, y_off);

                    let turns = if curr_dir == new_dir { 0 } 
                    else if curr_dir.0 == -new_dir.0 { 2 }
                    else { 1 };

                    let new_cost = cost + 1 + turns*1000;
                    let new_heur = self.heuristic(new_pos);
                    
                    if visited.contains(&(new_pos, new_dir)) { continue; }

                    visited.push((new_pos, new_dir));
                    pq.insert(
                        (new_pos, new_dir),
                        new_cost,
                        new_heur
                    );
                }
            } 
        }
        return None;
    }
}

// (0,1) -> (-1,0) -> (0, -1) -> (1,0)

#[derive(Debug, Default)]
struct PQueue<T> {
    es: Vec<T>,
    gs: Vec<usize>,
    hs: Vec<usize>,
}

impl<T> PQueue<T> {
    fn new() -> Self {
        Self {
            es: Vec::new(),
            gs: Vec::new(),
            hs: Vec::new(),
        }
    }

    fn insert(&mut self, e: T, g: usize, h: usize) {
        for i in 0..self.es.len() {
            if self.gs[i] + self.hs[i] < g + h {
                self.es.insert(i, e);
                self.gs.insert(i, g);
                self.hs.insert(i, h);
                return;
            }
        }
        self.es.push(e);
        self.gs.push(g);
        self.hs.push(h);
    }

    fn pop(&mut self) -> Option<(T, usize)> {
        if self.es.len() == 0 { return None; }
        let e = self.es.pop().unwrap();
        let g = self.gs.pop().unwrap();
        self.hs.pop();
        Some((e, g))
    }
}

fn part1(file_path: &str) -> usize {
    let m = Maze::from_file(file_path);
    m.get_cheapest_path().unwrap()
}

fn main() {
    let mut start = Instant::now();
    println!(
        "Lowest possible score: {} (Duration: {:.2?})",
        part1("src/day16/input.txt"),
        start.elapsed()
    );

    start = Instant::now();
    println!(
        "Lowest possible score: {} (Duration: {:.2?})",
        part2("src/day16/input.txt"),
        start.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(11048, part1("src/day16/test.txt"));
    }
}
