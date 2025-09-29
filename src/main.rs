fn main() {
    let answer = p001();
    println!("{answer}")
}

fn p001() -> i32 {
    let mut sum = 0;
    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum = sum + i;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p001() {
        assert_eq!(p001(), 233168)
    }
}
