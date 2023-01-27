use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },    
    Probation,    
    Refuse,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the treehouse, {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            },
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
        }
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
        Visitor::new("bert", VisitorAction::Accept, 45),
        Visitor::new("steve", VisitorAction::AcceptWithNote {
            note: String::from("Lactose free milk is in the Fridge")
        }, 45),
        Visitor::new("bert", VisitorAction::Refuse, 30),
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
                    visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
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
