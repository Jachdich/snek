pub struct Grid {
    data: Vec<Vec<char>>,
}

impl Grid {
    pub fn new() -> Self {
    	Grid {
    	    data: Vec::new()
    	}
    }

    pub fn from_string(txt: &str) -> Self {
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

    pub fn merge(&mut self, grid: &Grid, x: usize, y: usize) {
        for gy in 0..grid.data.len() {
            for gx in 0..grid.data[gy].len() {
                self.put(gx + x, gy + y, grid.data[gy][gx]);
            }
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
