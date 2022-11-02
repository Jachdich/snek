
struct Grid {
    data: Vec<Vec<char>>,
}

impl Grid {
    pub fn from_string(txt: String) -> Self {
    	let mut ret: Vec<Vec<char>> = Vec::new();
    	let arr_1d: Vec<char> = txt.chars().collect::<Vec<char>>();
    	ret.push(Vec::new());
    	for ch in arr_1d {
    		if ch == '\n' {
    			ret.push(Vec::new());
    		} else {
    			ret.last_mut().unwrap().push(ch);
    		}
    	}
    	while ret.len() > 0 && ret.last().unwrap().len() == 0 {
    		ret.pop();
    	}
    	Grid {
    	    data: ret
    	}
    }

    pub fn get(&self, x: isize, y: isize) -> char {
        if x < 0 || y < 0 {
            ' '
        } else {
            let x = x as usize;
            let y = y as usize;
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
    }
}

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

struct Interpreter {
    grid: Grid,
    x: isize,
    y: isize,
    dir: Dir,
    curr_ch_count: usize,
    stack: Vec<isize>,
    program: String,
}

fn highlight(n: &str, x: isize, y: isize) -> String {
    let x = x as usize;
    let y = y as usize;
    let mut buf = String::new();
    let mut ny = 0;
    for line in n.split("\n") {
        if y == ny {
            let by = line.as_bytes();
            buf.push_str(&format!("{}\x1b[7m{}\x1b[0m{}\n", std::str::from_utf8(&by[0..x]).unwrap(), by[x] as char, std::str::from_utf8(&by[(x+1)..]).unwrap()));
        } else {
            buf.push_str(line);
            buf.push('\n');
        }
        ny += 1;
    }
    buf
}

impl Interpreter {
    fn new(program: String) -> Self {
        Interpreter {
            grid: Grid::from_string(program.clone()),
            x: 0,
            y: 0,
            curr_ch_count: 0,
            dir: Dir::E,
            stack: Vec::new(),
            program,
        }
    }

    fn move_from_score(&mut self) {
/*     \___/
       /   \   */

        if self.dir == Dir::E {
            //if statement
            if self.grid.get(self.x + 1, self.y) == '/' && self.grid.get(self.x + 1, self.y + 1) == '\\' {
                if self.stack.pop() == Some(0) {
                    self.x += 1; self.y += 1; self.dir = Dir::SE;
                } else {
                    self.x += 1; self.dir = Dir::NE;
                }
            }

            else if self.grid.get(self.x + 1, self.y) == '_' { self.x += 1; }
            else if self.grid.get(self.x + 1, self.y) == '@' { self.x += 1; }
            else if self.grid.get(self.x + 1, self.y) == '/' { self.x += 1; self.dir = Dir::NE; }
            else if self.grid.get(self.x + 1, self.y + 1) == '\\' { self.x += 1; self.y += 1; self.dir = Dir::SE; }
            else { panic!("Unexpected end of path at {}, {} (from score E)", self.x, self.y); }
        } else if self.dir == Dir::W {
            //if statement
            if self.grid.get(self.x - 1, self.y) == '\\' && self.grid.get(self.x - 1, self.y + 1) == '/' {
                if self.stack.pop() == Some(0) {
                    self.x -= 1; self.y += 1; self.dir = Dir::SW;
                } else {
                    self.x -= 1; self.dir = Dir::NW;
                }
            }
            
            else if self.grid.get(self.x - 1, self.y) == '_' { self.x -= 1; }
            else if self.grid.get(self.x - 1, self.y) == '@' { self.x -= 1; }
            else if self.grid.get(self.x - 1, self.y + 1) == '/' { self.x -= 1; self.y += 1; self.dir = Dir::SW; }
            else if self.grid.get(self.x - 1, self.y) == '\\' { self.x -= 1; self.dir = Dir::NW; }
            else { panic!("Unexpected end of path at {}, {} (from score W)", self.x, self.y); }
        } else {
            panic!("What");
        }
    }
    
    fn move_from_slash(&mut self) {
        /*   |  |    |   _
            /   /   /  _/
            |  |   |     
          */  
        if self.dir == Dir::NE {
            //if statement
            //  |_
            //  /
            // /
            if self.grid.get(self.x, self.y - 1) == '|' && self.grid.get(self.x + 1, self.y - 1) == '_' {
                if self.stack.pop() == Some(0) {
                    self.x += 1; self.y -= 1; self.dir = Dir::E;
                } else {
                    self.y -= 1; self.dir = Dir::N;
                }
            }
            
            else if self.grid.get(self.x + 1, self.y - 1)      == '/' { self.x += 1; self.y -= 1; }
            else if self.grid.get(self.x + 1, self.y - 1)      == '@' { self.x += 1; self.y -= 1; }
            else if self.grid.get(self.x, self.y - 1)     == '|' { self.y -= 1; self.dir = Dir::N; }
            else if self.grid.get(self.x + 1, self.y - 1) == '|' { self.x += 1; self.y -= 1; self.dir = Dir::N; }
            else if self.grid.get(self.x + 1, self.y - 1) == '_' { self.x += 1; self.y -= 1; self.dir = Dir::E; }
            else { panic!("Unexpected end of path at {}, {} (from slash NE)", self.x, self.y); }
        } else if self.dir == Dir::SW {
            //if statement
            //   /
            // _/
            //  |
            if self.grid.get(self.x, self.y + 1) == '|' && self.grid.get(self.x - 1, self.y) == '_' {
                if self.stack.pop() == Some(0) {
                    self.x -= 1; self.dir = Dir::W;
                } else {
                    self.y += 1; self.dir = Dir::S;
                }
            }
            
            else if self.grid.get(self.x - 1, self.y + 1)      == '/' { self.x -= 1; self.y += 1; }
            else if self.grid.get(self.x - 1, self.y + 1)      == '@' { self.x -= 1; self.y += 1; }
            else if self.grid.get(self.x, self.y + 1)     == '|' { self.y += 1; self.dir = Dir::S; }
            else if self.grid.get(self.x - 1, self.y + 1) == '|' { self.x -= 1; self.y+= 1; self.dir = Dir::S; }
            else if self.grid.get(self.x - 1, self.y)     == '_' { self.x -= 1; self.dir = Dir::W; }
            else { panic!("Unexpected end of path at {}, {} (from slash SW)", self.x, self.y); }
        } else {
            panic!("What");
        }
    }
    
    fn move_from_backslash(&mut self) {
    /*    |   |  |  _
           \  \   \  \_
           |   |   |  
    */
        if self.dir == Dir::SE {
            //if statement
            // \
            //  \_
            //  |
            if self.grid.get(self.x + 1, self.y) == '_' && self.grid.get(self.x, self.y + 1) == '|' {
                if self.stack.pop() == Some(0) {
                    self.y += 1; self.dir = Dir::S;
                } else {
                    self.x += 1; self.dir = Dir::E;
                }
            }
            
            else if self.grid.get(self.x + 1, self.y + 1)      == '\\'{ self.x += 1; self.y += 1; }
            else if self.grid.get(self.x + 1, self.y + 1)      == '@' { self.x += 1; self.y += 1; }
            else if self.grid.get(self.x, self.y + 1)     == '|' { self.y += 1; self.dir = Dir::S; }
            else if self.grid.get(self.x + 1, self.y + 1) == '|' { self.x += 1; self.y += 1; self.dir = Dir::S; }
            else if self.grid.get(self.x + 1, self.y)     == '_' { self.x += 1; self.dir = Dir::E; }
            else { panic!("Unexpected end of path at {}, {} (from backslash SE)", self.x, self.y); }
        } else if self.dir == Dir::NW {

            //if statement
            //  _|
            //   \
            //    \
            if self.grid.get(self.x - 1, self.y - 1) == '_' && self.grid.get(self.x, self.y - 1) == '|' {
                if self.stack.pop() == Some(0) {
                    self.y -= 1; self.dir = Dir::N;
                } else {
                    self.x -= 1; self.y -= 1; self.dir = Dir::W;
                }
            }
        
            else if self.grid.get(self.x - 1, self.y - 1)      == '\\'{ self.x -= 1; self.y -= 1; }
            else if self.grid.get(self.x - 1, self.y - 1)      == '@' { self.x -= 1; self.y -= 1; }
            else if self.grid.get(self.x, self.y - 1)     == '|' { self.y -= 1; self.dir = Dir::N; }
            else if self.grid.get(self.x - 1, self.y - 1) == '|' { self.x -= 1; self.y -= 1; self.dir = Dir::N; }
            else if self.grid.get(self.x - 1, self.y - 1) == '_' { self.x -= 1; self.y -= 1; self.dir = Dir::W; }
            else { panic!("Unexpected end of path at {}, {} (from backslash NW)", self.x, self.y); }
        } else {
            panic!("What");
        }
    }
    
    fn move_from_pipe(&mut self) {
    /*      \ \  \      /  /    /  |
            |  |  |    |   |   |   |
             \ \   \   /  /   /    |
    */
        if self.dir == Dir::N {
            //if statement
            // \ /
            //  |
            //  |
            if self.grid.get(self.x - 1, self.y - 1) == '\\' && self.grid.get(self.x + 1, self.y - 1) == '/' {
                if self.stack.pop() == Some(0) {
                    self.y -= 1; self.x += 1; self.dir = Dir::NE;
                } else {
                    self.y -= 1; self.x -= 1; self.dir = Dir::NW;
                }
            }
            
            else if self.grid.get(self.x, self.y - 1)     == '|' { self.y -= 1; }
            else if self.grid.get(self.x, self.y - 1)     == '@' { self.y -= 1; }
            
            else if self.grid.get(self.x, self.y - 1)     == '\\'{ self.y -= 1; self.dir = Dir::NW; }
            else if self.grid.get(self.x - 1, self.y - 1) == '\\'{ self.x -= 1; self.y -= 1; self.dir = Dir::NW; }
            
            else if self.grid.get(self.x, self.y - 1)     == '/' { self.y -= 1; self.dir = Dir::NE; }
            else if self.grid.get(self.x + 1, self.y - 1) == '/' { self.x += 1; self.y -= 1; self.dir = Dir::NE; }
            else { panic!("Unexpected end of path at {}, {} (from pipe N)", self.x, self.y); }
        } else if self.dir == Dir::S {
            //if statement
            //  |
            //  |
            // / \
            if self.grid.get(self.x - 1, self.y + 1) == '/' && self.grid.get(self.x + 1, self.y + 1) == '\\' {
                if self.stack.pop() == Some(0) {
                    self.y += 1; self.x -= 1; self.dir = Dir::SW;
                } else {
                    self.y += 1; self.x += 1; self.dir = Dir::SE;
                }
            }
            
            else if self.grid.get(self.x, self.y + 1)     == '|' { self.y += 1; }
            else if self.grid.get(self.x, self.y + 1)     == '@' { self.y += 1; }
            
            else if self.grid.get(self.x, self.y + 1)     == '\\'{ self.y += 1; self.dir = Dir::SE; }
            else if self.grid.get(self.x + 1, self.y + 1) == '\\'{ self.x += 1; self.y += 1; self.dir = Dir::SE; }
            
            else if self.grid.get(self.x, self.y + 1)     == '/' { self.y += 1; self.dir = Dir::SW; }
            else if self.grid.get(self.x - 1, self.y + 1) == '/' { self.x -= 1; self.y += 1; self.dir = Dir::SW; }
            else { panic!("Unexpected end of path at {}, {} (from pipe S)", self.x, self.y); }
        } else { panic!("What"); }
        
    }

    fn execute_command(&mut self, ch: char, count: usize) {
        if count == 0 || count == 1 { return; }
        match ch {
            '_' => self.stack.push(count as isize - 1),
            '\\' => {
                if count == 5 {
                    println!("{}", self.stack.pop().unwrap());
                } else if count == 6 {
                    print!("{}", char::from_u32(self.stack.pop().unwrap() as u32).unwrap());
                } else if count == 4 {
                    let mut s = String::new();
                    std::io::stdin().read_line(&mut s).expect("some kind of IO error");
                    let mut data: Vec<u8> = s.as_bytes().iter().cloned().collect();
                    data.reverse();
                    for byte in data {
                        self.stack.push(byte as isize);
                    }
                } else if count == 2 {
                    let val = self.stack.pop().unwrap();
                    self.stack.push(val);
                    self.stack.push(val);
                } else if count == 3 {
                    let val_1 = self.stack.pop().unwrap();
                    let val_2 = self.stack.pop().unwrap();
                    self.stack.push(val_1);
                    self.stack.push(val_2);
                }
            }
            '|' => {
                if count == 2 { let val = self.stack.pop().unwrap() + self.stack.pop().unwrap(); self.stack.push(val); }
                if count == 3 { let val = self.stack.pop().unwrap() - self.stack.pop().unwrap(); self.stack.push(val); }
                if count == 4 { let val = self.stack.pop().unwrap() * self.stack.pop().unwrap(); self.stack.push(val); }
                if count == 5 { self.stack.push(0); }
                if count == 6 { let val = self.stack.pop().unwrap() / self.stack.pop().unwrap(); self.stack.push(val); }
            }

            '/' => {
                if count == 2 { self.stack.pop(); }
                if count == 3 { 
                    let idx = self.stack.pop().unwrap() as usize;
                    let val = self.stack.pop().unwrap();
                    self.stack[idx] = val;
                }
                if count == 4 {
                    let idx = self.stack.pop().unwrap() as usize;
                    let val = self.stack[idx];
                    self.stack.push(val);
                }
            }
            _ => (),
        }
    }

    fn run(&mut self) {
        loop {
            let curr_ch = self.grid.get(self.x, self.y);
            //println!("{}, {}, {}, {:?}, {:?}", highlight(&self.program, self.x, self.y), self.x, self.y, self.dir, self.stack);
            match curr_ch {
                '_' => self.move_from_score(),
                '/' => self.move_from_slash(),
                '\\'=> self.move_from_backslash(),
                '|' => self.move_from_pipe(),
                ch => panic!("Unexpected char '{}' in path", ch),
            }

            if self.grid.get(self.x, self.y) == curr_ch {
                self.curr_ch_count += 1;
            } else {
                self.execute_command(curr_ch, self.curr_ch_count);
                self.curr_ch_count = 1;
            }

            if self.grid.get(self.x, self.y) == '@' {
                break;
            }
        }
    }

}

fn main() {
let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }
    let program = std::fs::read_to_string(&args[1]).unwrap();
    let mut interp = Interpreter::new(program);
    interp.run();
}
