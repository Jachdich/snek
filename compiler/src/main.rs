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
macro_rules! lconcat {
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

    pub fn putstr(&mut self, val: String) {
        
    }
    
    pub fn push(&mut self, n: usize) {
        if n == 0 {
            self.s_op(5);
        } else {

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
                self.putstr(lconcat!(
                    r#"\"#,
                    r#"|"#,
                    r#"/"#
                ));
                self.pos += Vec2::new(-1, 2);
                
                for _ in 0..(n + 1) {
                    self.put('_');
                    self.pos.x -= 1;
                }
                
                self.pos.y += 1;
                self.putstr(lconcat!(
                    r"/",
                    r"|",
                    r"\_"
                ));
                self.pos += Vec2::new(2, 3);
            }
        }
    }

    fn se_op(&mut self, n: usize) {
        self.pos.y += 1;
        for _ in 0..n {
            self.put('\\');
            self.pos.x += 1;
            self.pos.y += 1;
        }
        self.pos.y -= 1;
        self.put('_');
        self.pos.x += 1;
    }
    
    fn s_op(&mut self, n: usize) {
        self.pos.y += 1;
        self.put('\\');
        self.pos.y += 1;
        for i in 0..n {
            self.put('|'); self.pos.y += 1;
        }
        self.put('\\');
        self.pos.x += 1;
        self.put('_');
        self.pos.x += 1;
    }

    pub fn add(&mut self) { self.s_op(2); }
    pub fn sub(&mut self) { self.s_op(3); }
    pub fn mul(&mut self) { self.s_op(4); }
    pub fn div(&mut self) { self.s_op(6); }

    pub fn dup(&mut self) { self.se_op(2); }
    pub fn swap(&mut self) { self.se_op(3); }
    pub fn getch(&mut self) { self.se_op(4); }
    pub fn putd(&mut self) { self.se_op(5); }
    pub fn putc(&mut self) { self.se_op(6); }
    
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
    cg.push(10);
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
    cg.push(72);
    for i in 0..14 {
        cg.putc();
    }
    cg.halt();
    let src = cg.src();
    let mut file_ref = std::fs::File::create("example").expect("create failed");
    write!(file_ref, "{}", src);
}
