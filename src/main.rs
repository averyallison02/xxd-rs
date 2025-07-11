use std::env;
use std::fs::File;
use std::path::Path;

use colored::Colorize;

mod reader;

const BYTES_PER_LINE: usize = 32;
const BYTES_PER_BLOCK: usize = BYTES_PER_LINE / 8;

const START_ADDR: usize = 0;

fn print_data(data: &String)
{
    print!("{:08x}: ", START_ADDR);

    for (i, ch) in data.chars().enumerate()
    {
        print!("{}", ch.to_string().bold());

        let current_addr: usize = i / 2;

        if i % BYTES_PER_BLOCK == BYTES_PER_BLOCK - 1
        {
            print!(" ");
        }

        if i % BYTES_PER_LINE == BYTES_PER_LINE - 1
        {
            println!();
            if i != data.len() - 1 {print!("{:08x}: ", (current_addr + 1))}
        }
    }

    if data.len() % BYTES_PER_LINE != 0
    {
        println!();
    }
}

fn main()
{
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {panic!("Usage: xxd-rs <filename>")};
    let file_name: &String = &args[1];
    let file_path: &Path = Path::new(file_name);

    let mut file: File = match File::open(&file_path)
    {
        Err(why) => panic!("Couldn't open {}: {}", file_path.display(), why),
        Ok(file) => file,
    };

    let data: String = reader::read_file(&mut file, file_path);

    print_data(&data);
}
