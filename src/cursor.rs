use std::io::Write;
pub fn up(n: usize) {
    print!("\x1b[{}A", n);
    std::io::stdout().flush().unwrap();
}

pub fn down(n: usize) {
    print!("\x1b[{}B", n);
    std::io::stdout().flush().unwrap();
}

pub fn left(n: usize) {
    print!("\x1b[{}D", n);
    std::io::stdout().flush().unwrap();
}

pub fn right(n: usize) {
    print!("\x1b[{}C", n);
    std::io::stdout().flush().unwrap();
}
/// (1,1) (2, 1) (3, 1)<br/>
/// (1,2) (2, 2) (3, 2)<br/>
/// (1,3) (2, 3) (3, 3)<br/>
pub fn goto(x: usize, y: usize) {
    print!("\x1b[{y};{x}f");
    std::io::stdout().flush().unwrap();
}

pub fn save_pos() {
    print!("\x1b7");
    std::io::stdout().flush().unwrap();
}

pub fn move_save() {
    print!("\x1b8");
    std::io::stdout().flush().unwrap();
}
