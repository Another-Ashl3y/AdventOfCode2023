use std::str::from_utf8;



fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> usize {

    let numbers = ["0","1","2","3","4","5","6","7","8","9"];

    let data = input.split("\r\n");
    let mut out = 0;

    for i in data {
        println!("-----------------------------------------------------");
        println!("Word: {}", i);

        let mut x = 0;

        'search: for j in 0..i.len()+1 {

            let word = from_utf8(&i
                .as_bytes()[0..j])
                .expect("Something happened ig"); // Make the word first to last

            for num in 0..numbers.len() {
                if word.contains(numbers[num]) {
                    x += num*10; // Add the first number in position 1
                    break 'search; // Break the loop as the number has been found
                }
            }
        }

        'reverse_search : for j in (0..i.len()+1).rev() {

            let word = from_utf8(&i
                .as_bytes()[j..i.len()])
                .expect("Something happened ig"); // Make the word from as 10..10 then 9..10 then 8..10

            for num in 0..numbers.len() {
                if word.contains(numbers[num]) {
                    x += num; // Add the second number in position 0
                    break 'reverse_search; // Break the loop as the number has been found
                }
            }
        }
        println!("Number: {}", x);
        out += x;
    }
    println!("Stonks: {}", out);
    out
}



#[cfg(test)]
mod tests {
    use crate::part1;
    #[test]
    fn it_works() {
        let input = include_str!("./test.txt");
        let result = part1(input);
        assert_eq!(result, 142);
    }
}

