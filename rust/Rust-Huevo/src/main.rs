use rand::prelude::*;
use std::io::{self};
use std::sync::mpsc;
use std::{thread, time};

fn main() {
    let frames = [
        r#"
        /\_/\
       / o o \
       \¨ ^ ¨/
        /   \   \
       /|_|_|\__/
    "#,
        r#"
        /\_/\
       / = = \
       \¨ ^ ¨/
        /   \    _
       /|_|_|\__/
    "#,
        r#"
        /\_/\
       / o o \
       \¨ ^ ¨/
        /   \    _
       /|_|_|\__/
    "#,
        r#"
        /\_/\
       / = = \
       \¨ w ¨/
        /   \   \
       /|_|_|\__/
    "#,
        r#"
        /\_/\
       / o o \
       \¨ w ¨/
        /   \    _
       /|_|_|\__/
    "#,
    ];

    let mut _msg = String::default();
    const MILLIS: u64 = 500;
    let mut _num = u8::default();
    let menu1 = Menu {
        options: String::from(
            r#"
        +---------------+
        |   Options     |
        +---------------+ 
        | 1. Feed       |
        | 2. Pet        |
        | 3. Kick       |
        | 0. Exit       |
        +---------------+ 
    "#,
        ),
    };

    let mut cat1 = Cat {
        name: String::from("pedrito"),
        status: CatStatus::Alive,
        stats: Stats::new(), // pero no se usar los metodos xs
    };

    let (tx, rx) = mpsc::channel();

    let input_thread = thread::spawn(move || {
        println!("Que quieres hacer?");
        let result = input(false); // stdin:
        tx.send(result).unwrap();
    });

    for frame in frames.iter().cycle() {
        if cat1.status == CatStatus::Death {
            print!(
                r#"
            /\_/\
           / x x \
           \¨ ^ ¨/
            /   \    _
           /|_|_|\__/
        "#,
            );
            break;
        }
        _num = randnum();
        // En cada siclo sube el hambre
        cat1.live();
        if _num % 2u8 == 0 {
            _msg = String::from("..");
        } else {
            _msg = String::from("meaw");
        }

        print!("{}", menu1.options);
        print!("{}", frame);
        println!("\n{}", _msg);

        // let valorrecivido = rx.try_recv();

        let seleccion = rx.try_recv().unwrap_or("MELAPELAS".to_string());
        print!("Input -> {:?} \n", seleccion);

        if seleccion == "3" {
            cat1.stats.health -= 25;
        }

        if seleccion == "1" {
            // Al aumentar hambre disminnuye vida
            cat1.stats.feed();
        }

        print!("Vida -> {}\n", cat1.stats.health);
        print!("Hambre -> {}\n", cat1.stats.hambre);
        cat1.check_status(); // Checkea el estado del Kato en base a la vida

        match cat1.status {
            CatStatus::Alive => {
                println!("Tu Kato esta vivo! :)")
            }
            CatStatus::Sick => {
                println!("Tu Kato esta enfermo! :(")
            }
            CatStatus::Death => {
                println!("Tu Kato se fallecio! :(")
            }
        }
        thread::sleep(time::Duration::from_millis(MILLIS));
        // clear_terminal();
        // no se espera
    }
    input_thread.join().unwrap();
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

fn randnum() -> u8 {
    let rand_number: u8 = rand::thread_rng().gen_range(1..=10);

    rand_number
}

fn input(enter: bool) -> String {
    let mut user_input: String = String::default();

    if enter == true {
        println!("Press Enter to continue ");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        user_input = trim_with_carriagereturn(user_input);

        while user_input != "" {
            println!("Press Enter to continue ");
            //User Input with \r\n trimming
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");
            user_input = trim_with_carriagereturn(user_input);
        }

        return user_input;
    }

    //User Input
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    user_input = trim_with_carriagereturn(user_input);

    user_input
}

pub fn trim_with_carriagereturn(input: String) -> String {
    input.replace("\r", "").trim().to_string()
}

struct Menu {
    options: String, // "
                     //     +---------------+
                     //     |   Options     |
                     //     +---------------+
                     //     | 1. Feed       |
                     //     | 2. Pet        |
                     //     | 2. Kick       |
                     //     | 0. Exit       |
                     //     +---------------+
                     //     "

                     //     println!();

                     //     let selection: String = input(false);

                     //     match selection.as_str(){
                     //         "1" => { println!("Selected option -> {} - Play", selection); game() },
                     //         "2" => { println!("Selected option -> {} - Scoreboards", selection)},
                     //         "0" => { println!("Selected option -> {} - Exit", selection); finish() },
                     //         _ =>   { println!("Invalid Option")}
                     //     }
                     // }
} // wtf!? pregunto a ia
#[derive(Debug, PartialEq, Eq)]
enum CatStatus {
    Alive,
    Sick,
    Death,
}

// impl PartialEq for CatStatus{
//     fn eq(&self, _: &CatStatus) -> bool { todo!() }
// }

struct Cat {
    name: String,
    // status: String,
    status: CatStatus, // pero agregar aqui
    stats: Stats,      // cuando se implemente
                       // alive: bool
}

// trait Status {
//     fn calculatestatus(&self) -> String{
//         let yomama = String::default();
//         yomama
//     }
// }

impl Cat {
    fn check_status(&mut self) {
        // donde deberia estar el tick en esto o en el loop arriba?
        let health = self.stats.health; // no
        if health == 0 {
            // self.status = String::from("death"); // no necesita returns
            self.status = CatStatus::Death;
        } else if health > 0 && health < 50 {
            // self.status = String::from("sick"); // no necesita returns
            self.status = CatStatus::Sick;
        } else {
            // self.status = String::from("Alive"); // *carita sonrojada*
            self.status = CatStatus::Alive;
        }
    }
    // como es vivir?
    fn live(&mut self) {
        // da hambre sip esa es la idea
        self.stats.tick(); // tenes copilot o que es lo que auto completa?
    }
}

struct Stats {
    health: u8,
    hambre: u8,
    // dream on: u8,
    // tired: u8,
}

impl Stats {
    fn new() -> Stats {
        // por ahora eso
        Stats {
            health: 20, // si com lo estopeamos? si llega a cero bueno
            hambre: 10,  // no
        }
    }
    fn feed(&mut self) {
        self.hambre -= 5;
    }
    fn tick(&mut self) {
        if self.health > 0 {
            self.health -= 1;
            let _num = randnum();
            if _num % 2u8 == 0 {
                self.hambre += 1;
            }
            // self.hambre += 1;
        }
    }
}

// use std::io;
// use std::thread;

// fn main() {
//     // Spawn a separate thread to handle user input
//     let input_thread = thread::spawn(|| {
//         let mut input = String::new();
//         println!("Please enter something:");

//         if let Ok(_) = io::stdin().read_line(&mut input) {
//             println!("You entered: {}", input);
//         } else {
//             println!("Error reading input.");
//         }
//     });

//     // Your main thread can continue executing other tasks
//     for i in 1..=10 {
//         println!("Main thread doing work... {}", i);
//         thread::sleep(std::time::Duration::from_secs(1));
//     }

//     // Wait for the input thread to finish
//     input_thread.join().unwrap();
// }
