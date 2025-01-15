use std::{
    fmt::format,
    fs::File,
    io::{self, stdin, Write},
    vec,
};

struct Person {
    name: String,
    phone: String,
    age: u8,
}

impl Person {
    fn new() -> Person {
        let mut input = String::new();
        let mut person: Person = Person {
            name: String::new(),
            phone: String::new(),
            age: 0,
        };
        println!("Enter your name: ");
        stdin()
            .read_line(&mut input)
            .expect("Filed to read command");
        person.name = input.trim().to_string();
        input.clear();
        println!("Enter your age: ");
        stdin().read_line(&mut input).expect("Failed to read age!");
        person.age = input.trim().parse().unwrap();
        input.clear();
        println!("your phone: ");
        stdin()
            .read_line(&mut input)
            .expect("Faield to read phone!");
        person.phone = input.trim().to_string();
        input.clear();
        person
    }

    fn formatted(&self) -> String {
        format!(
            "Name:{:<10} Age:{:<10} Phone:{:<10}\n",
            self.name, self.age, self.phone,
        )
    }
}

struct NoteBook {
    persons: Vec<Person>,
}

fn clear() {
    print!("\x1B[2J\x1B[1;1H"); // Clear the terminal and moves the cursor to the top-left corner
}
fn main() -> io::Result<()> {
    let mut cmd = String::new();
    let mut note_book = NoteBook { persons: vec![] };
    println!("Welcome to my phone book!");
    println!("\n");
    loop {
        println!("0- Exit!");
        println!("1- Add Person");
        println!("2- Persons list");
        println!("3- Write list to a file");
        stdin()
            .read_line(&mut cmd)
            .expect("Faield to read command!");
        match cmd.trim() {
            "0" => {
                println!("Good bye");
                clear();
                break;
            }
            "1" => {
                let person = Person::new();
                println!("-----------------");
                println!("{}", person.formatted());
                note_book.persons.push(person);
            }
            "2" => {
                println!("--------------------------------------------");
                for p in &note_book.persons {
                    println!("{}", p.formatted());
                }
            }
            "3" => {
                let mut file = File::create("list.txt").expect("Filead to create file!");
                for p in &note_book.persons {
                    file.write(p.formatted().as_bytes())
                        .expect("Unable to write file");
                }
                return Ok(());
            }
            other => {
                println!("Panic error! {other}");
                clear();
                break;
            }
        }
        cmd.clear();
    }
    Ok(())
}
