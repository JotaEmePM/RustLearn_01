use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: string },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_lowercase(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn whats_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("steve", "Hello Steve, your milk is in the fridge."),
        Visitor::new("bert", "WOW!!, Who invited Fred?"),
    ];

    loop {
        println!("Hi, what's your name?");
        let name = whats_is_your_name();
        let know_visitor = visitor_list
            .iter()
            .find(|visitor| visitor.name == name);

        match know_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{}, You're not on the visitor list. Please leave.", name);
                    visitor_list.push(Visitor::new(&name, "New friend"));
                }
            }
        }    
    }

    // let mut allow_them_in: bool = false;

    // for visitor in &visitor_list {
    //     if visitor.name == &name {
    //         allow_them_in = true
    //     }
    // }

    // if allow_them_in {
    //     println!("Welcome to the Treehouse, {}", name);
    // } else {
    //     println!("Sorry, you aren't on the list.");
    // }
        println!("The final list of visitors:");
        println!("{:#?}", visitor_list);
    
}
