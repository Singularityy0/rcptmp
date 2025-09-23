// Singu Rcp IO template , finally !!!
mod rcp {
    use std::{
        fs::File,
        io::{self, BufRead, BufReader},
    };

    pub fn read<T: std::str::FromStr>(reader: &mut impl BufRead) -> T {
        let mut line = String::new();
        reader.read_line(&mut line).expect("read_line failed");
        line.trim().parse().ok().expect("failed to parse")
    }

    pub fn read_vec<T: std::str::FromStr>(reader: &mut impl BufRead) -> Vec<T> {
        let mut line = String::new();
        reader.read_line(&mut line).expect("read_line failed");
        line.split_whitespace()
            .map(|s| s.parse().ok().expect("failed to parse"))
            .collect()
    }

    pub fn init_reader() -> Box<dyn BufRead> {
        if std::env::args().any(|arg| arg == "--file") {
            Box::new(BufReader::new(
                File::open("input.txt").expect("failed to open 'input.txt'"),
            ))
        } else {
            Box::new(BufReader::new(io::stdin()))
        }
    }
}
// okay write from here , should be fairly simple (or not)
fn solve(){
    
}
fn main() {
    
}
