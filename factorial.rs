use std::io::BufferedReader;
use std::io::println;
use std::io;


fn main() {
    println("Enter something:");
    let mut reader = BufferedReader::new(io::stdin());

    let input = reader.read_line().unwrap();
    println("Something that you typed:");
    println(input);
}