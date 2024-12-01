use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::str::SplitWhitespace;

struct State {
    employees: BTreeMap<String, BTreeSet<String>>,
}

fn add_user_to_company(state: &mut State, input: &mut SplitWhitespace) -> () {
    let user = String::from(input.next().expect("no user"));
    input.next().expect("bad format");
    let company: String = String::from(input.next().expect("no company"));
    println!("Added {user} to {company}");
    state
        .employees
        .entry(company)
        .or_insert(BTreeSet::new())
        .insert(user);
}

fn list_peoples_of_company(state: &State, company: &String) {
    let empty = BTreeSet::new();
    let peoples = state.employees.get(company).unwrap_or(&empty);
    println!("{peoples:#?}")
}

pub fn hash_companies() {
    let mut state: State = State {
        employees: BTreeMap::new(),
    };
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("no input");
        let mut split = input.split_whitespace();
        match split.next().expect("need command") {
            "Add" => add_user_to_company(&mut state, &mut split),
            "List" => match split.next() {
                Some(company) => list_peoples_of_company(&state, &String::from(company)),
                None => println!("{:#?}", &state.employees),
            },
            _ => println!("invalid command"),
        }
    }
}
