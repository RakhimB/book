enum Publication {
    Book { title: String, author: String, page_count: u32 },
    Magazine { title: String, issue: u32, topic: String },
}

impl Publication {
    
    fn print_details(&self) {
        match self {
            Publication::Book { title, author, page_count } => {
                println!("Book: {} Author: {}, {} pages", title, author, page_count);
            }
            Publication::Magazine { title, issue, topic } => {
                println!("Magazine: {} - Issue: {}, Topic: {}", title, issue, topic);
            }
        }
    }
}

fn main() {
    
    let publications: Vec<Publication> = vec![
        Publication::Book { title: String::from("The Rust Programming Language"), author: String::from("Steve Klabnik and Carol Nichols"), page_count: 560 },
        Publication::Magazine { title: String::from("National Geographic"), issue: 150, topic: String::from("Nature and Science") },
    ];

    
    for publication in publications.iter() {
        publication.print_details();
    }
}
