use std::io::{self, BufRead, Write};

pub fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let stdout = io::stdout();
    solve(stdin, stdout);
} 

fn solve<R, W>(stdin: R, mut stdout: W) where R: BufRead, W: Write {
    let parts: Vec<f64> = stdin.lines().next().unwrap().unwrap()
                               .split(' ')
                               .map(|v| v.parse().unwrap())
                               .collect();
    let x = parts[0];
    let y = parts[1];
    let s = parts[2];

    let result = (x / s).ceil() * (y / s).ceil();
    writeln!(&mut stdout, "{}", &result).expect("Error writing to stdout");
}

#[cfg(test)]
mod test {
    use super::solve;

    #[test]
    fn test_1_a () {
        let input = b"6 6 4";
        let mut output = Vec::new();
        solve(&input[..], &mut output);
        assert_eq!(output, b"4\n");
    }
}