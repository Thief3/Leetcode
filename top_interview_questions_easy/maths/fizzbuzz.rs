/*******************************************************************************

Fizzbuzz

Write a program that outputs the string representation of numbers from 1 to n.

But for multiples of three it should output “Fizz” instead of the number and for
the multiples of five output “Buzz”. For numbers which are multiples of both
three and five output “FizzBuzz”.

 ******************************************************************************/

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut strings: Vec<String> = Vec::new();
        for i in 1..(n + 1) {
            if i % 15 == 0 {
                strings.push("FizzBuzz".to_string());
            }
            else if i % 5 == 0 {
                strings.push("Buzz".to_string());
            }
            else if i % 3 == 0 {
                strings.push("Fizz".to_string());
            }
            else {
                strings.push(i.to_string());
            }
        }
        return strings;
    }
}
