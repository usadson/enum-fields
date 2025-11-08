// Copyright (C) 2025 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, enum_fields::EnumFields)]
pub enum Entity {
    Person {
        name: String,
    },

    House {
        name: Option<String>,
    },

    Animal {
        name: Option<String>,
    },
}

fn main() {
    let company = Entity::Person {
        name: "Apple".into(),
    };

    let person = Entity::Person {
        name: "Tim Berners-Lee".into()
    };

    println!("Company named: {:?}",
        company.name()
    );

    println!("Person named: {:?}", person.name());

    // Since [`Entity`] has two variants that both have the `name` field,
    // `Entity::name(&self)` returns the `&String`.
    assert_eq!(company.name(), Some(&"Apple".to_string()));
    assert_eq!(person.name(), Some(&"Tim Berners-Lee".to_string()));
}
