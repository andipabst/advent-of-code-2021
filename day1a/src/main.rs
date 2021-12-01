fn main() {
    let numbers = include_str!("../input.txt")
        .lines()
        .map(|i| i.parse::<i32>().unwrap_or(0));

    let mut sum = 0;
    let mut last: Option<i32> = None;

    for number in numbers {
        match last {
            Some(l) => {
                if number > l {
                    sum += 1;
                }
            }
            _ => {}
        }
        last = Some(number)
    }

    println!("{}", sum);
}
