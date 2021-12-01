fn main() {
    let numbers = include_str!("../input.txt")
        .lines()
        .map(|i| i.parse::<i32>().unwrap_or(0));

    let numbers1 = numbers.clone().skip(1);
    let numbers2 = numbers.clone().skip(2);

    let sums = numbers
        .zip(numbers1)
        .zip(numbers2)
        .map(|((a, b), c)| { a + b + c });

  
    let mut sum = 0;
    let mut last: Option<i32> = None;

    for number in sums {
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

    println!("{:?}", sum);
}
