use std::io::*;
use fixedbitset::FixedBitSet;

struct Life {
    grid: FixedBitSet,
    grid_new: FixedBitSet,
    width: u32,
    height: u32,
}

impl Life {
    fn new(width: u32, height: u32) -> Life {
        Life {
            grid: FixedBitSet::with_capacity(width.wrapping_mul(height) as usize),
            grid_new: FixedBitSet::with_capacity(width.wrapping_mul(height) as usize),
            width: width,
            height: height,
        }
    }

    fn coord(&self, x:u32, y:u32) -> usize {
        y.wrapping_mul(self.width).wrapping_add(x) as usize
    }

    /// Out-of-bound writes have no effect
    fn set(&mut self, x: u32, y: u32, val: bool) {
        if x < self.width && y < self.height {
            self.grid_new.set(self.coord(x, y), val);
        }
    }

    /// Out-of-bound reads return false
    fn get(&self, x: u32, y: u32) -> bool {
        if x < self.width && y < self.height {
            self.grid.contains(self.coord(x, y))
        } else {
            false
        }
    }

    fn step(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let mut live_neighbor_count = 0;
                macro_rules! neigh {
                    ($x_off:expr, $y_off:expr) => {
                        let x_off: i32 = $x_off;
                        let y_off: i32 = $y_off;
                        if self.get(x.wrapping_add(x_off as u32), y.wrapping_add(y_off as u32)) {
                            live_neighbor_count += 1
                        }
                    }
                }

                neigh!(1, 0);
                neigh!(-1, 0);
                neigh!(0, 1);
                neigh!(0, -1);
                neigh!(1, 1);
                neigh!(-1, 1);
                neigh!(1, -1);
                neigh!(-1, -1);
                
                if self.get(x, y) {
                    if live_neighbor_count == 2 || live_neighbor_count == 3 {
                        self.set(x, y, true);
                    }
                } else {
                    if live_neighbor_count == 3 {
                        self.set(x, y, true);
                    }
                }
            }
        }
        self.swap();
    }

    fn swap(&mut self) {
        std::mem::swap(&mut self.grid, &mut self.grid_new);
        self.grid_new = FixedBitSet::with_capacity(self.width.wrapping_mul(self.height) as usize);
    }

    fn dump(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) {
                    print!("█");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    fn dump_106(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) {
                    println!("{} {}", x, y);
                }
            }
        }
    }
}

fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    let count: u32 = args.next().expect("first arg should be count").parse().expect("count should be u32");
    let x: u32 = args.next().expect("second arg should be x").parse().expect("x should be u32");
    let y: u32 = args.next().expect("third arg should be y").parse().expect("y should be u32");
    let coords = std::io::stdin().lock().lines().map(|l| {
            let l = l.unwrap();
            let mut split = l.trim().split(' ');
            let x = split.next().unwrap().parse::<u32>().expect("need u32 x");
            let y = split.next().unwrap().parse::<u32>().expect("need u32 y");
            (x, y)
    }).collect::<Vec<_>>();
    assert!(y.checked_mul(x).is_some()); // justify wrapping arithmetic (x*y doesn't overflow)
    println!("running for {} steps on {}x{} grid, {} live cells at start", count, x, y, coords.len());
    let mut l = Life::new(x, y);
    for (x, y) in coords {
        l.set(x, y, true);
    }
    l.swap();
    l.dump();
    let start = std::time::Instant::now();
    for _ in 0..count {
        l.step();
        //l.dump();
    }
    l.dump();
    let end = std::time::Instant::now();
    println!("Took {:?}", end - start);
    l.dump_106();
}
