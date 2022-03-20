use std::fs::File;
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

struct Grid {
    data: Vec<Vec<char>>,
}

impl Grid {
    pub fn new() -> Self {
    	Grid {
    	    data: Vec::new()
    	}
    }

    pub fn get(&self, x: usize, y: usize) -> char {
        if y < self.data.len() {
            if x < self.data[y].len() {
                self.data[y][x]
            } else {
                ' '
            }
        } else {
            ' '
        }
    }

    pub fn put(&mut self, x: usize, y: usize, val: char) {
        if y >= self.data.len() {
            self.data.resize(y + 1, Vec::new());
        }
        if x >= self.data[y].len() {
            self.data[y].resize(x + 1, ' ');
        }

        self.data[y][x] = val;
    }

    pub fn to_string(&self) -> String {
        let mut ret = String::new();
        for line in &self.data {
            ret.push_str(&line.iter().cloned().collect::<String>());
            ret.push('\n');
        }
        ret
    }
}

struct CodeGen {
    pub x: usize,
    pub y: usize,
    pub dir: Dir,
    pub grid: Grid
}

impl CodeGen {
    pub fn new() -> Self {
        let mut s = Self {
            x: 1,
            y: 0,
            dir: Dir::E,
            grid: Grid::new()
        };

        s.grid.put(0, 0, '_');
        s
    }

    pub fn put(&mut self, val: char) {
        self.grid.put(self.x, self.y, val);
    }
    
    pub fn push(&mut self, n: usize) {
        if n == 0 {
            self.s_op(5);
        } else {
            self.y += 1;
            self.put('\\');
            self.x += 1;
            for _ in 0..(n + 1) {
                self.put('_');
                self.x += 1;
            }
        }
    }

    fn se_op(&mut self, n: usize) {
        self.y += 1;
        for _ in 0..n {
            self.put('\\');
            self.x += 1;
            self.y += 1;
        }
        self.y -= 1;
        self.put('_');
        self.x += 1;
    }
    
    fn s_op(&mut self, n: usize) {
        self.y += 1;
        self.put('\\');
        self.y += 1;
        for i in 0..n {
            self.put('|'); self.y += 1;
        }
        self.put('\\');
        self.x += 1;
        self.put('_');
        self.x += 1;
    }

    pub fn add(&mut self) { self.s_op(2); }
    pub fn sub(&mut self) { self.s_op(3); }
    pub fn mul(&mut self) { self.s_op(4); }
    pub fn div(&mut self) { self.s_op(6); }

    pub fn dup(&mut self) { self.se_op(2); }
    pub fn swap(&mut self) { self.se_op(3); }
    pub fn getch(&mut self) { self.se_op(4); }
    pub fn putd(&mut self) { self.se_op(5); }
    pub fn putc(&mut self) { self.se_op(5); }
    
    pub fn halt(&mut self) {
        self.put('@');
    }

    pub fn src(&self) -> String {
        self.grid.to_string()
    }

    pub fn cond<F: Fn(&mut Self), G: Fn(&mut Self)>(&mut self, tbranch: F, fbranch: G) {
        let ogx = self.x;
        let ogy = self.y;
        self.y += 1;
        self.put('\\');
        self.x += 1;
        self.put('_');
        self.x += 1;
        tbranch(self);
        let tx = self.x;
        let ty = self.y;
        self.x = ogx;
        self.y = ogy;
        self.put('/');
        self.y -= 1;
        self.x += 1;
        self.put('_');
        self.x += 1;
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
    cg_pushstr("Hello, world!");
    let src = cg.src();
    let mut fileRef = std::fs::File::create("example").expect("create failed");
    write!(fileRef, "{}", src);
}
