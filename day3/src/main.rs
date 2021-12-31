use std::borrow::Borrow;
use proc_macro::bridge::PanicMessage::String;

fn main() {
    let values = include_str!("../input.txt")
        .lines()
        .map(|i| i.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let number_of_values = values.len() as i32;

    let mut sum_of_ones = [0; 12];

    for value in values.to_vec() {
        for (i, char) in value.into_iter().enumerate() {
            match char {
                '1' => {
                    sum_of_ones[i] = sum_of_ones[i] + 1;
                }
                _ => {}
            }
        }
    }

    let gamma_rate_binary = sum_of_ones.into_iter()
        .map(|x| { if x > (number_of_values / 2) { '1' } else { '0' } })
        .collect::<String>();

    let gamma_rate = isize::from_str_radix(&gamma_rate_binary, 2).unwrap();
    let epsilon_rate = !gamma_rate & 0b1111_1111_1111;

    println!("part one: {}*{} = {}", gamma_rate, epsilon_rate, gamma_rate * epsilon_rate);

    let mut values_copy = values.to_vec();

    for (i, bit) in gamma_rate_binary.chars().enumerate() {
        values_copy.retain(|value| {
            return value[i] == bit
        });
        
        println!("size after pos {}: {}", i, values_copy.len())
    }
    let oxygen_genearator_rating = isize::from_str_radix(&values_copy[0], 2).unwrap();
    
    String(values_copy[0])
    
    println!("{}", oxygen_genearator_rating)
}
