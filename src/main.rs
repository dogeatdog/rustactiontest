#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: i8,
}

impl Person {
    pub fn make_important() -> Self {
        Self {
            first_name: "Important".to_owned(),
            last_name: "".to_owned(),
            age: 20,
        }
    }

    pub fn new(name: String) -> Self {
        Self {
            first_name: name,
            last_name: "".to_owned(),
            age: 0,
        }
    }

    pub fn default() -> Self {
        Self {
            first_name: "John".to_owned(),
            last_name: "Doe".to_owned(),
            age: 32,
        }
    }
}

fn main() {
    println!("Hello, world!");

    let var_one = "dennis";
    println!("Hello {}", var_one);

    let dennis_person = Person {
        first_name: "dennis".to_owned(),
        last_name: "meerveld".to_owned(),
        age: 46,
    };

    let important_person = Person::make_important();

    println!("{:?}", important_person);

    let eline = Person::new("Eline".to_owned());

    let john = Person::default();

    println!("{:?}", dennis_person);

    println!("{:?}", eline);

    println!("{:?}", john);
}
