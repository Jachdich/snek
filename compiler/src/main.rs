mod grid;

use grid::Grid;
use std::io::Write;

#[derive(PartialEq, Debug)]
enum Dir {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW
}

impl Dir {
    pub fn to_vec(&self) -> Vec2 {
        match self {
            Dir::N  => Vec2::new(0, -1),
            Dir::NE => Vec2::new(1, -1),
            Dir::E  => Vec2::new(1,  0),
            Dir::SE => Vec2::new(1,  1),
            Dir::S  => Vec2::new(0,  1),
            Dir::SW => Vec2::new(-1, 1),
            Dir::W  => Vec2::new(-1, 0),
            Dir::NW => Vec2::new(-1, -1),
        }
    }
}

struct Vec2 {
    pub x: isize,
    pub y: isize,
}

impl Vec2 {
    fn new(x: isize, y: isize) -> Vec2 {
        Vec2 {
            x, y
        }
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}


#[macro_export]
macro_rules! lcat {
    ( $( $x: literal ),* ) => {
        [$($x,)*].join("\n")
    }
}

struct CodeGen {
    pub pos: Vec2,
    pub dir: Dir,
    pub target_dir: Dir,
    pub grid: Grid
}

impl CodeGen {
    pub fn new() -> Self {
        let mut s = Self {
            pos: Vec2::new(1, 0),
            dir: Dir::E,
            target_dir: Dir::E,
            grid: Grid::new()
        };

        s.grid.put(0, 0, '_');
        s
    }

    pub fn put(&mut self, val: char) {
        self.grid.put(self.pos.x as usize, self.pos.y as usize, val);
    }

    pub fn putstr(&mut self, val: &str) {
        let ngrid = Grid::from_string(val);
        self.grid.merge(&ngrid, self.pos.x as usize, self.pos.y as usize);
    }

    fn get_closest_score(&self) -> Dir {
        match self.target_dir {
            Dir::N | Dir::NE | Dir::E | Dir::SE => Dir::E,
            Dir::S | Dir::SW | Dir::W | Dir::NW => Dir::W,
        }
    }

    fn go(&mut self, dir: Dir) {
        self.pos += dir.to_vec();
    }
    
    fn fw(&mut self) {
        self.pos += self.dir.to_vec();
    }

    fn change_dir(&mut self, dir: Dir) {
        if dir == self.dir {
            match dir {
                Dir::N  => { self.put('/'); self.go(Dir::N); }
                Dir::NE => { self.put('_'); self.go(Dir::E); }
                Dir::E  => { self.go(Dir::S); self.put('\\'); self.go(Dir::E); }
                Dir::SE => { self.go(Dir::N); self.put('_'); self.go(Dir::SE); }
                Dir::S  => { self.put('/'); self.go(Dir::S); }
                Dir::SW => { self.go(Dir::N); self.put('_'); self.go(Dir::SE); }
                Dir::W  => { self.put('\\'); self.go(Dir::NW); }
                Dir::NW => { self.put('_'); self.go(Dir::W); }
            }
        }

        match self.dir {
            Dir::N => {
                match dir {
                    Dir::N  => (), //already handled above
                    Dir::NE => (),
                    Dir::E  => { self.put('/'); self.go(Dir::NE); }
                    Dir::SE => { self.change_dir(Dir::E);  self.put('_'); self.go(Dir::SE); }
                    Dir::S  => { self.change_dir(Dir::SE); self.put('\\'); self.go(Dir::S); }
                    Dir::SW => { self.change_dir(Dir::W);  self.put('_');  self.go(Dir::SW); }
                    Dir::W  => { self.put('\\'); self.go(Dir::NW); }
                    Dir::NW => (),
                }
            },
            Dir::NE => {
                match dir {
                    Dir::N  => { self.go(Dir::W); },
                    Dir::NE => (),
                    Dir::E  => (),
                    Dir::SE => { self.put('_'); self.go(Dir::SE); },
                    Dir::S  => { self.change_dir(Dir::SE); self.put('\\'); self.go(Dir::S); },
                    Dir::SW => { self.change_dir(Dir::S);  self.put('|');  self.go(Dir::S); },
                    Dir::W  => { self.change_dir(Dir::NW); self.put('\\'); self.go(Dir::NW); },
                    Dir::NW => { self.change_dir(Dir::N);  self.put('|');  self.go(Dir::N); },
                }
            }
            Dir::E => {
                Dir::N => 

                Dir::E => (),
                Dir::SE => { self.go(Dir::SE); },
                Dir::S  => { self.change_dir(Dir::SE); self.put('\\'); self.go(Dir::W); },
            }
            _ => ()
        }

        self.dir = dir;
    }
    
    pub fn push(&mut self, n: usize) {
        if n == 0 {
            self.pipe_op(5);
        } else {
            /*
            if n + 10 > self.pos.x as usize {
                self.pos.y += 1;
                self.put('\\');
                self.pos.x += 1;
                for _ in 0..(n + 1) {
                    self.put('_');
                    self.pos.x += 1;
                }
            } else {
                self.pos.y += 1;
                self.putstr(&lcat!(
                    r"\ ",
                    r"|",
                    r"/"
                ));
                self.pos += Vec2::new(-1, 2);
                
                for _ in 0..(n + 1) {
                    self.put('_');
                    self.pos.x -= 1;
                }
                
                self.pos.y += 1;
                self.putstr(&lcat!(
                    r"/",
                    r"|",
                    r"\_"
                ));
                self.pos += Vec2::new(2, 2);
            }*/
            let closest_dir = self.get_closest_score();
            self.change_dir(closest_dir);
            for _ in 0..(n + 1) {
                self.put('_');
                self.pos += self.dir.to_vec();
            }
        }
    }

    fn bslash_op(&mut self, n: usize) {
        self.pos.y += 1;
        for _ in 0..n {
            self.put('\\');
            self.pos += Vec2::new(1, 1);
        }
        self.pos.y -= 1;
        self.put('_');
        self.pos.x += 1;
    }
    
    fn pipe_op(&mut self, n: usize) {
        self.pos.y += 1;
        self.put('\\');
        self.pos.y += 1;
        for _ in 0..n {
            self.put('|'); self.pos.y += 1;
        }
        self.putstr(r"\_");
        self.pos.x += 2;
    }

    fn slash_op(&mut self, n: usize) {
        
    }

    pub fn add(&mut self) { self.pipe_op(2); }
    pub fn sub(&mut self) { self.pipe_op(3); }
    pub fn mul(&mut self) { self.pipe_op(4); }
    pub fn div(&mut self) { self.pipe_op(6); }

    pub fn dup(&mut self) { self.bslash_op(2); }
    pub fn swap(&mut self) { self.bslash_op(3); }
    pub fn getch(&mut self) { self.bslash_op(4); }
    pub fn putd(&mut self) { self.bslash_op(5); }
    pub fn putc(&mut self) { self.bslash_op(6); }
    
    pub fn halt(&mut self) {
        self.put('@');
    }

    pub fn src(&self) -> String {
        self.grid.to_string()
    }

    pub fn cond<F: Fn(&mut Self), G: Fn(&mut Self)>(&mut self, tbranch: F, fbranch: G) {
        let ogx = self.pos.x;
        let ogy = self.pos.y;
        self.pos.y += 1;
        self.put('\\');
        self.pos.x += 1;
        self.put('_');
        self.pos.x += 1;
        tbranch(self);
        let tx = self.pos.x;
        let ty = self.pos.y;
        self.pos.x = ogx;
        self.pos.y = ogy;
        self.put('/');
        self.pos.y -= 1;
        self.pos.x += 1;
        self.put('_');
        self.pos.x += 1;
        fbranch(self);
    }
}

fn main() {
    let mut cg = CodeGen::new();
    /*
    cg.push(6);
    cg.push(6);
    cg.sub();
    cg.cond(|cg| {
        cg.push(69);
    }, |cg| {
        cg.push(42);
    });
    cg.putd();
    cg.halt();
    */
    /*cg.push(10);
    cg.push(33);
    cg.push(100);
    cg.push(108);
    cg.push(114);
    cg.push(111);
    cg.push(119);
    cg.push(32);
    cg.push(44);
    cg.push(111);
    cg.push(108);
    cg.push(108);
    cg.push(101);
    cg.push(72);*/
    /*for _ in 0..14 {
        cg.putc();
    }*/

    cg.change_dir(Dir::E);
    cg.put('_');
    cg.fw();
    cg.pos += Vec2::new(10, 10);
    cg.put('/');
    cg.dir = Dir::NE;
    cg.fw();
    cg.change_dir(Dir::W);
    cg.put('_');
    cg.fw();

    cg.halt();
    let src = cg.src();
    print!("{}", src);
    //let mut file_ref = std::fs::File::create("example").expect("create failed");
    //write!(file_ref, "{}", src).unwrap();
}
