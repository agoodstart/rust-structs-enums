use console::Term;

fn main() {
    let mut me = create_character(String::from("Joris"), 24);
    println!("Hi, I am {} and I am {} years old", me.username, me.age);

    loop {
        let term = Term::stdout();
        let key = term.read_key();
    
        let input = loop {
                match key {
                    Ok(key) => { break key},
                    Err(_) => { println!("Nothing")} 
                }
        };

        if input == console::Key::Escape {
            break;
        }
    
        me.walk(input);
    
        match &me.movement {
            Some(dir) => println!("I am walking in direction: {:?}", dir),
            None => println!("I am standing still"),
        }
    }
}

struct Character {
    username: String,
    movement: Option<Movement>,
    age: u8,
}

#[derive(Debug)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
}

fn create_character(username: String, age: u8) -> Character {
    Character {
        username,
        movement: None,
        age,
    }
}

impl Character {
    fn walk(&mut self, dir: console::Key) {
        match dir {
            console::Key::ArrowUp => self.movement = Some(Movement::Up),
            console::Key::ArrowDown => self.movement = Some(Movement::Down),
            console::Key::ArrowLeft => self.movement = Some(Movement::Left),
            console::Key::ArrowRight => self.movement = Some(Movement::Right),
            _ => self.movement = None,
        }
    }
}