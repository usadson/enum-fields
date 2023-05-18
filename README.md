# enum-fields
&emsp; [![Build Status]][actions] [![Latest Version]][crates.io]

[Build Status]: https://img.shields.io/github/actions/workflow/status/usadson/enum-fields/rust.yml?branch=main
[actions]: https://github.com/usadson/enum-fields/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/enum-fields.svg
[crates.io]: https://crates.io/crates/enum-fields

Quickly access shared enum fields in Rust.

## Installation
Add the `enum-fields` crate to your `Cargo.toml` file:
```toml
[dependencies]
enum-fields = "*"
```

Let your `enum` derive from `enum_fields::EnumFields` like this:
```rs
#[derive(enum_fields::EnumFields)]
pub enum MyEnum {
    ...
}
```

## Usage
The following example showcases an enum `Entity`, which contains two
variants: `Company` and `Person`.

```rs
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
```

Since `Entity` derives from [`enum_fields::EnumFields`], it now contains
two field accessor functions (getters): `Entity::name()` and
`Entity::ceo()`.

```rs
let company = Entity::Company {
    name: "Apple".into(),
    ceo: "Tim Cook".into()
};

let person = Entity::Person {
    name: "Tim Berners-Lee".into()
};

println!("Company with CEO: {} named: {}",
    company.ceo().unwrap(),
    company.name()
);

println!("Person named: {}", person.name());
```

Note that both `Company` and `Person` have a field named `name`. This
enforces `enum-fields` to let `Entity::name()` return the type directly.

```rs
// Since [`Entity`] has two variants that both have the `name` field,
// `Entity::name(&self)` returns the `&String`.
assert_eq!(company.name(), "Apple");
assert_eq!(person.name(), "Tim Berners-Lee");
```

However, only `Company` has field `ceo`, which therefore makes
`Entity::ceo()` return an optional getter: `Option<&String>`.

```rs
// Only `Company` has field `ceo`, so it returns an `Option<&String>`,
// since a `Person` returns [`None`].
assert_eq!(company.ceo(), Some(&"Tim Cook".into()));
assert_eq!(person.ceo(), None);
```

## License
Licensed under either <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

<br>

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in EnumFields by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
