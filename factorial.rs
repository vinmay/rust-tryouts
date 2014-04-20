use std::io::BufferedReader;
    use std::io::println;
    use std::io;

    fn main() {
    	let mut i = 1;
    	let mut f = 1;    	
    	let mut mynum = 0;
        println("Enter a number:");
        let mut reader = BufferedReader::new(io::stdin());
        let input = reader.read_line().unwrap();
        let num = from_str::<int>(input.slice_to(input.len() - 1));

        match num {
                Some(number) => {
                    mynum = number
                }
                None         => println("Hey, put in a number.")
            }
        while i <= mynum {
        	f = f * i;
        	i += 1;
        }
        println("Factorial is:");
        println(f.to_str());
    }