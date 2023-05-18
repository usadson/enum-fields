// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! The following example showcases an enum [`Entity`], which contains two
//! variants: `Company` and `Person`.
//!
//! Since [`Entity`] derives from [`enum_fields::EnumFields`], it now contains
//! two field accessor functions (getters): `Entity::name()` and
//! `Entity::ceo()`.
//!
//! Note that both `Company` and `Person` have a field named `name`. This
//! enforces `enum-fields` to let `Entity::name()` return the type directly.
//!
//! However, only `Company` has field `ceo`, which therefore makes
//! `Entity::ceo()` return an optional getter: `Option<&String>`.

/// An entity that can be either a `Company` or a `Person`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, enum_fields::EnumFields)]
pub enum Entity {
    Company {
        name: String,
        ceo: String,
    },

    Person {
        name: String,
    }
}

fn main() {
    let company = Entity::Company {
        name: "Apple".into(),
        ceo: "Tim Cook".into()
    };

    let person = Entity::Person {
        name: "Tim Berners-Lee".into()
    };

    println!("Company with CEO: {:?} named: {}",
        company.ceo().unwrap(),
        company.name()
    );

    println!("Person named: {}", person.name());

    // Since [`Entity`] has two variants that both have the `name` field,
    // `Entity::name(&self)` returns the `&String`.
    assert_eq!(company.name(), "Apple");
    assert_eq!(person.name(), "Tim Berners-Lee");

    // Only `Company` has field `ceo`, so it returns an `Option<&String>`,
    // since a `Person` returns [`None`].
    assert_eq!(company.ceo(), Some(&"Tim Cook".into()));
    assert_eq!(person.ceo(), None);
}
