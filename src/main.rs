use std::fs::OpenOptions;
use std::io::{self, Write};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()> {
    loop {
        let mut inputmenu = String::new();
        clear_screen();
        header();
        show_menu();

        io::stdin().read_line(&mut inputmenu)?;

        match inputmenu.trim() {
            "1" => show_tasks()?,
            "2" => write_on_file()?,
            "3" => delete_task()?,
            "Q" => return Ok(()),
            _ => println!("Invalid Choice !"),
        };
    }

    //Ok(())
}

fn show_menu() {
    println!("1. Show tasks");
    println!("2. New task");
    println!("3. Delete Task");
    println!("Q. Quit");
}

fn header() {
    println!("-------------------------------------");
    println!("|            SIMPLE TASKER          |");
    println!("-------------------------------------\n\n");
}

// Used for clear screen
fn clear_screen() {
    print!("{}[2J", 27 as char);
}

fn show_tasks() -> io::Result<()> {
    clear_screen();
    let mut enter = String::new();
    let file_name = "tasker.txt";
    let file = File::open(&file_name)?;
    let reader = BufReader::new(file);
    let mut i = 1; 

    for line in reader.lines() {
        println!("{} - {}", i, line?);
        i += 1;
    }

    println!("\n\nEnter to continue...");
    io::stdin().read_line(&mut enter)?;
    Ok(())
}

fn write_on_file() -> io::Result<()> {
    clear_screen();
    println!("Add a new Task: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let file_name = "tasker.txt";
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_name)?;

    file.write_all(input.as_bytes())?;

    Ok(())
}


fn delete_task() -> io::Result<()> {
    // Show Tasks
    clear_screen();
    let _enter = String::new();
    let file_name = "tasker.txt";
    let file = File::open(&file_name)?;
    let reader = BufReader::new(file);
    let mut i = 1; 

    for line in reader.lines() {
        println!("{} - {}", i, line?);
        i += 1;
    }

    // Action
    println!("Enter the line number of the task to delete: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let line_number: usize = input.trim().parse().expect("Please enter a valid line number");

    let file_name = "tasker.txt";
    let file = File::open(&file_name)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut file = File::create(&file_name)?;

    for (i, line) in lines.iter().enumerate() {
        if i + 1 != line_number {
            writeln!(file, "{}", line)?;
        }
    }

    println!("Task at line {} deleted successfully.", line_number);

    Ok(())
}
