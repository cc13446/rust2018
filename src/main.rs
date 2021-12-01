use crate::board::new_board;
mod board;
const SIZE: usize = 4;

fn main() {
    let mut board = new_board();
    let mut is_over = false;
    let mut score = 0;
    while !is_over {
        println!("你当前的分数为{}\n", score);
        println!("{}", board.to_string());
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        score += match line.as_str() {
            "w\n" => board.top(),
            "a\n" => board.left(),
            "s\n" => board.down(),
            "d\n" => board.right(),
            _ => {0}
        };
        is_over = (board.random_one_tile() == 0);
    }

}
