use core::ptr;
use rand::Rng;
pub struct Board {
    board:[[u32; crate::SIZE]; crate::SIZE]
}
// 新建棋盘
pub fn new_board() -> Board{
    let b: [[u32; crate::SIZE]; crate::SIZE] = [[0;4];4];
    let mut result = Board{board:b};
    result.random_one_tile();
    result.random_one_tile();
    return result;
}

impl Board {
    // 随机一个新块
    pub fn random_one_tile(&mut self) -> u32 {
        let mut flag = 0;
        for i in 0..crate::SIZE {
            for j in 0..crate::SIZE {
                if self.board[i][j] == 0 {
                    flag = flag + 1;
                }
            }
        }
        if flag == 0 {
            return 0;
        }
        // 大概率出2 小概率出4
        let mut new_id = 2;
        let mut rng = rand::thread_rng();
        if rng.gen_range(0..100) >= 90 {
            new_id = 4;
        }
        let mut random = rng.gen_range(0..flag);
        for i in 0..crate::SIZE {
            for j in 0..crate::SIZE {
                if self.board[i][j] == 0 {
                    if random == 0 {
                        self.board[i][j]=new_id;
                        return flag;
                    }
                    random -= 1;
                }
            }
        }
        return flag;
    }
    // 棋盘两个位置交换
    fn board_swap(&mut self, a1:usize, a2:usize, b1:usize, b2:usize){
        let pa = &mut self.board[a1][a2] as *mut u32;
        let pb = &mut self.board[b1][b2] as *mut u32;
        unsafe {
            ptr::swap(pa, pb);
        }
    }
    // 棋盘两个位置合并
    fn board_merge(&mut self, a1:usize, a2:usize, b1:usize, b2:usize) -> u32{
        let mut res = 0;
        if self.board[a1][a2] == self.board[b1][b2] {
            res = self.board[a1][a2];
            self.board[a1][a2] = res * 2;
            self.board[b1][b2] = 0;
        }
        return res;
    }
    // 上
    pub fn top (&mut self) -> u32 {
        let mut res = 0;
        for j in 0..crate::SIZE {
            for i in 0..crate::SIZE {
                for k in (i + 1..crate::SIZE).rev() {
                    if self.board[k][j] > 0 && self.board[k - 1][j] == 0 {
                        self.board_swap(k, j, k - 1, j);
                    }
                }
            }
            for i in 0..crate::SIZE - 1 {
                res += self.board_merge(i ,j, i + 1, j);
            }
            for i in 0..crate::SIZE {
                for k in (i + 1..crate::SIZE).rev() {
                    if self.board[k][j] > 0 && self.board[k - 1][j] == 0 {
                        self.board_swap(k, j, k - 1, j);
                    }
                }
            }
        }
        return res;
    }
    // 左
    pub fn left (&mut self) -> u32 {
        let mut res = 0;
        for i in 0..crate::SIZE {
            for j in 0..crate::SIZE {
                for k in (j + 1..crate::SIZE).rev() {
                    if self.board[i][k] > 0 && self.board[i][k - 1] == 0 {
                        self.board_swap(i, k, i, k - 1);
                    }
                }
            }
            for j in 0..crate::SIZE - 1 {
                res += self.board_merge(i ,j, i, j + 1);
            }
            for j in 0..crate::SIZE {
                for k in (j + 1..crate::SIZE).rev() {
                    if self.board[i][k] > 0 && self.board[i][k - 1] == 0 {
                        self.board_swap(i, k, i, k - 1);
                    }
                }
            }
        }
        return res;
    }
    // 下
    pub fn down (&mut self) -> u32 {
        let mut res = 0;
        for j in 0..crate::SIZE {
            for i in (1..crate::SIZE).rev() {
                for k in 0..i {
                    if self.board[k][j] > 0 && self.board[k + 1][j] == 0 {
                        self.board_swap(k, j, k + 1, j);
                    }
                }
            }
            for i in (1..crate::SIZE).rev() {
                res += self.board_merge(i ,j, i - 1, j);
            }
            for i in (1..crate::SIZE).rev() {
                for k in 0..i {
                    if self.board[k][j] > 0 && self.board[k + 1][j] == 0 {
                        self.board_swap(k, j, k + 1, j);
                    }
                }
            }
        }
        return res;
    }
    // 右
    pub fn right (&mut self) -> u32 {
        let mut res = 0;
        for i in 0..crate::SIZE {
            for j in (1..crate::SIZE).rev() {
                for k in 0..j {
                    if self.board[i][k] > 0 && self.board[i][k + 1] == 0 {
                        self.board_swap(i, k, i, k + 1);
                    }
                }
            }
            for j in (1..crate::SIZE).rev() {
                res += self.board_merge(i ,j, i, j - 1);
            }
            for j in (1..crate::SIZE).rev() {
                for k in 0..j {
                    if self.board[i][k] > 0 && self.board[i][k + 1] == 0 {
                        self.board_swap(i, k, i, k + 1);
                    }
                }
            }
        }
        return res;
    }
    pub fn to_string(&self) -> String {
        let mut res = String::from("");
        for i in 0..crate::SIZE {
            for j in 0..crate::SIZE {
                let temp = self.board[i][j].to_string();
                res += &temp;
                res = res + "\t";
            }
            res = res + "\n";
        }
        return res;
    }
}
