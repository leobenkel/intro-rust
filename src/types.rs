#![allow(dead_code, unreachable_code, unused_variables)]

// 01 - TYPES
//
// Rust is a statically typed programming language, which means that every expression, function
// input, and function output has a "type" that is known statically at compile-time. The type
// determines a set of values which are permissible at this point in the program, and can be
// thought of as a compile-time description of how the memory that holds a value is to be
// interpreted. Static types allow you to prevent invalid use of memory, preventing a large
// class of runtime errors from ever occurring.
//
// In this section, you will learn about how to declare your own types and how to use those
// types in very simple ways. You will also learn about common types in the Rust standard library.

/// STRUCTS
///
/// A struct is a type that you can define, which has a name and a set of fields. A
/// struct can be thought of as a record or object, similar to a class in object-oriented
/// languages, but without any methods (structs are pure data).
///
/// The fields of a struct may be anonymous, in which case the struct is called a tuple struct,
/// and the fields of the struct are accessed by index. An ordinary struct, however, has named
/// fields, and the fields of the struct are accessed by name.
mod structs {
    #[test]
    fn basic_struct_example() {
        // Add `name` and `age` fields to this struct, of type `String` and `u32` respectively.
        struct Person {}

        assert_eq!(std::mem::size_of::<Person>(), 32);
    }

    #[test]
    fn basic_tuple_struct_example() {
        // Add `name` and `age` fields to this tuple struct, of type `String` and `u32` respectively.
        struct Person();

        assert_eq!(std::mem::size_of::<Person>(), 32);
    }

    #[test]
    fn struct_debug() {
        // Derive the `Debug` trait for this struct, so that it can be printed.
        struct Person {
            name: &'static str,
            age: u32,
        }

        let person = Person {
            name: "John Doe",
            age: 42,
        };

        assert_eq!(
            format!("{:?}", todo!("person")),
            "Person { name: \"John Doe\", age: 42 }"
        );
    }

    #[test]
    fn struct_eq() {
        // Derive the `PartialEq` and `Eq` traits for this struct, so that it can be compared for
        // equality.
        struct Person {
            name: &'static str,
            age: u32,
        }

        let person1 = Person {
            name: "John Doe",
            age: 42,
        };
        let person2 = Person {
            name: "John Doe",
            age: 42,
        };

        assert_eq!(todo!("person1"), todo!("person2"));
    }

    #[test]
    fn struct_clone() {
        // Derive the `Clone` trait for this struct, so that it can be cloned.
        #[derive(Debug, PartialEq, Eq)]
        struct Person {
            name: &'static str,
            age: u32,
        }

        let person1 = Person {
            name: "John Doe",
            age: 42,
        };
        let person2 = todo!("person1.clone()");

        assert_eq!(person1, person2);
    }

    #[test]
    fn struct_clone_deep() {
        // Derive the `Clone` trait for this struct, so that it can be cloned.
        struct Person {
            address: Address,
        }

        struct Address {
            street: u32,
        }

        let old_person: Person = Person {
            address: Address { street: 42 },
        };
        let new_person: Person = todo!("old_person.clone()");

        old_person.address.street = 0;

        assert_eq!(new_person.address.street, 42);
    }

    #[test]
    fn struct_copy() {
        // Derive the `Copy` trait for these structs, so that they can be copied.
        #[derive(Debug, PartialEq, Eq)]
        struct Person {
            name: &'static str,
            age: u32,
        }

        let person1 = Person {
            name: "John Doe",
            age: 42,
        };
        let person2 = todo!("person1");

        assert_eq!(person1, person2);
    }

    #[test]
    fn struct_hash() {
        #![allow(unused_mut)]

        // Derive the `Hash` trait for this struct, so that it can be used in a `HashMap`.
        #[derive(Debug, Hash, PartialEq, Eq, Clone)]
        struct Person {
            name: &'static str,
            age: u32,
        }

        use std::collections::HashMap;

        let mut person_to_address: HashMap<Person, &str> = HashMap::new();

        let sherlock = Person {
            name: "Sherlock Holmes",
            age: 42,
        };

        // person_to_address.insert(sherlock.clone(), "221B Baker Street");
        let gotten: Option<&&str> = todo!("person_to_address.get(&sherlock)");

        assert_eq!(gotten, Some(&"221B Baker Street"));
    }

    #[test]
    fn struct_default() {
        // Derive the `Default` trait for this struct, so that it can be created with `Default::default()`.
        #[derive(Debug, PartialEq, Eq)]
        struct Person {
            name: &'static str,
            age: u32,
        }

        let person: Person = todo!("Default::default()");

        assert_eq!(person, Person { name: "", age: 0 });
    }

    #[test]
    fn struct_destructuring() {
        #[derive(Debug, PartialEq, Eq)]
        struct Person {
            name: &'static str,
            age: u32,
        }

        let person = Person {
            name: "John Doe",
            age: 42,
        };

        // Destructure the person struct into its fields, so that the test passes.
        // The syntax is the same as for construction, except the field names are
        // on the left-hand side of the `=` sign, and you use `let` to create the
        // variables that are bound to the fields.

        assert_eq!(todo!("name") as &'static str, "John Doe");
        assert_eq!(todo!("age") as u32, 42);
    }

    #[test]
    fn struct_pattern_matching() {
        #[derive(Debug, PartialEq, Eq)]
        struct Person {
            name: &'static str,
            age: u32,
        }

        let person = Person {
            name: "John Doe",
            age: 42,
        };

        // Match on the person struct and extract out the name and age fields,
        // and put them into a tuple.
        let (name, age) = todo!("match person ...") as (&'static str, u32);

        assert_eq!(todo!("name") as &'static str, "John Doe");
        assert_eq!(todo!("age") as u32, 42);
    }
}

/// ENUMS
///
/// An enum is another data type that you can define, which has a name and a set of variants. An
/// enum corresponds to a "sum type" in functional programming, and can be thought of as a tagged
/// union. An enum value is constructed using one of the variants, and can be destructed using
/// pattern matching. Rust ensures that pattern matches against enums are exhaustive, meaning that
/// you must handle every possible variant of the enum.
///
/// Enums are used in the Rust standard library for a variety of purposes, including (and notably)
/// error handling.
mod enums {
    #[test]
    fn basic_enum_example() {
        // Define an enum called `Direction` with the variants `North`, `South`, `East`, and `West`.
        #[derive(Debug, PartialEq, Eq)]
        enum Direction {}

        // Define a function that takes a `Direction` and returns a `bool` indicating whether the
        // direction is `North`.
        fn is_north(d: Direction) -> bool {
            todo!("Use equality to compare `d` to `Direction::North`")
        }

        assert_eq!(todo!("is_north(Direction::North)") as bool, true);
        assert_eq!(todo!("is_north(Direction::South)") as bool, false);
        assert_eq!(todo!("is_north(Direction::East)") as bool, false);
        assert_eq!(todo!("is_north(Direction::West)") as bool, false);
    }

    #[test]
    fn enum_with_data_example() {
        // Define an enum called `Movement` with the variants `North`, `South`, `East`, and `West`,
        // each of which has an associated `u32` value, which indicates the number of spaces to move
        // in that direction.
        #[derive(Debug, PartialEq, Eq)]
        enum Movement {}

        // Define a function that takes a `Movement` and returns a `u32` indicating how many spaces
        // the movement should move.
        fn spaces(m: Movement) -> u32 {
            #[allow(unreachable_patterns)]
            match m {
                _ => todo!("Match on `m` and return the associated value for each variant"),
            }
        }

        assert_eq!(todo!("spaces(Movement::North(42))") as u32, 42);
        assert_eq!(todo!("spaces(Movement::South(42))") as u32, 42);
        assert_eq!(todo!("spaces(Movement::East(42))") as u32, 42);
        assert_eq!(todo!("spaces(Movement::West(42))") as u32, 42);
    }

    #[test]
    fn enum_debug() {
        // Derive the `Debug` trait for this enum, so that it can be printed.
        enum Direction {
            North,
            South,
            East,
            West,
        }

        assert_eq!(
            todo!("format!(\"{{:?}}\", Direction::North)") as &str,
            "North"
        );
        assert_eq!(
            todo!("format!(\"{{:?}}\", Direction::South)") as &str,
            "South"
        );
        assert_eq!(
            todo!("format!(\"{{:?}}\", Direction::East)") as &str,
            "East"
        );
        assert_eq!(
            todo!("format!(\"{{:?}}\", Direction::West)") as &str,
            "West"
        );
    }

    #[test]
    fn enum_eq() {
        // Derive the `PartialEq` and `Eq` traits for this enum, so that it can be compared for
        // equality.
        #[derive(Debug)]
        enum Direction {
            North,
            South,
            East,
            West,
        }

        assert_eq!(todo!("Direction::North == Direction::North") as bool, true);
        assert_eq!(todo!("Direction::North == Direction::South") as bool, false);
        assert_eq!(todo!("Direction::North == Direction::East") as bool, false);
        assert_eq!(todo!("Direction::North == Direction::West") as bool, false);
    }

    #[test]
    fn enum_clone() {
        // Derive the `Clone` trait for this enum, so that it can be cloned.
        #[derive(Debug, PartialEq, Eq)]
        enum Direction {
            North,
            South,
            East,
            West,
        }

        let north = Direction::North;
        let north2 = todo!("north.clone()");

        assert_eq!(north, north2);
    }

    #[test]
    fn enum_copy() {
        // Derive the `Copy` trait for this enum, so that it can be copied.
        #[derive(Debug, PartialEq, Eq)]
        enum Movement {
            North(u32),
            South(u32),
            East(u32),
            West(u32),
        }

        let north = Movement::North(42);

        let north2 = todo!("north");

        assert_eq!(north, north2);
    }

    #[test]
    fn enum_hash() {
        #![allow(unused_mut)]

        // Derive the `Hash` trait for this enum, so that it can be used in a `HashMap`.
        #[derive(Debug, PartialEq, Eq, Clone)]
        enum Detective {
            SherlockHolmes,
            HerculePoirot,
            PhilipMarlowe,
            CAugusteDupin,
        }

        use std::collections::HashMap;

        let mut detective_to_address: HashMap<Detective, &str> = HashMap::new();

        let sherlock = Detective::SherlockHolmes;

        // detective_to_address.insert(sherlock.clone(), "221B Baker Street");

        let gotten: Option<&&str> = todo!("detective_to_address.get(&sherlock)");

        assert_eq!(gotten, Some(&"221B Baker Street"));
    }

    #[test]
    fn enum_default() {
        // Derive the `Default` trait for this enum, so that it can be created with `Default::default()`.
        // Note that you will have to choose which unit to make default with the `#[default]` attribute.
        #[derive(Debug, PartialEq, Eq)]
        enum Direction {
            North,
            South,
            East,
            West,
        }

        let direction: Direction = todo!("Default::default()");

        assert_eq!(direction, Direction::North);
    }

    #[test]
    fn enum_destructuring_if_let() {
        #[derive(Debug, PartialEq, Eq)]
        enum Direction {
            North,
            South,
            East,
            West,
        }

        let direction = Direction::North;

        // Unlike structs, you cannot always destructure an enum into one of its variants, because it
        // may have been constructed with a different variant. However, for conditional destructuring,
        // you can use the `if let` syntax.

        // Use `if let` to destructure `direction` into `Direction::North`, and return true,
        // otherwise return false.
        let result: bool = todo!("if let ...");

        assert_eq!(result, true);
    }

    #[test]
    fn enum_destructuring_let_else() {
        #[derive(Debug, PartialEq, Eq)]
        enum JobTitle {
            Engineer { level: u32 },
            Manager,
        }

        let title = JobTitle::Engineer { level: 3 };

        // Use let-else to destructure `title` into `Engineer`, extracting out the level or instead
        // calling `panic!` with an error message.
        let level = if let JobTitle::Engineer { level } = title {
            level
        } else {
            0
        };

        assert_eq!(level, 3);
    }

    #[test]
    fn enum_pattern_matching() {
        #[derive(Debug, PartialEq, Eq)]
        enum Direction {
            North,
            South,
            East,
            West,
        }

        let direction = Direction::North;

        // Pattern match on the direction enum and return true if it is `Direction::North`, otherwise
        // return false. Experiment with omitting variant cases, or using wildcards.
        let result: bool = todo!("match direction ...");

        assert_eq!(result, true);
    }

    #[test]
    fn enum_deep_pattern_matching() {
        #[derive(Debug, PartialEq, Eq)]
        enum CharacterClass {
            Fighter { power: Power },
            Thief { power: Power },
            Wizard { power: Power },
        }

        #[derive(Debug, PartialEq, Eq)]
        enum Power {
            Low,
            Medium,
            High,
        }

        // Pattern match on the character class and return true if it is a high-powered thief,
        // otherwise return false.
        fn is_high_powered_thief(c: CharacterClass) -> bool {
            todo!("match c ...")
        }

        let thief = CharacterClass::Thief { power: Power::High };

        assert_eq!(is_high_powered_thief(thief), true);
    }
}

mod generics {
    #[test]
    fn struct_generic() {
        // Define a struct called `Pair` that has two type parameters, `A` and `B`,
        // and two fields, `a` and `b`, of type `A` and `B` respectively.
        #[derive(Debug, PartialEq, Eq)]
        struct Pair {}

        // let pair = Pair { a: 42, b: "foo" };

        assert_eq!(todo!("pair.a") as i32, 42);
        assert_eq!(todo!("pair.a") as &str, "foo");
    }

    #[test]
    fn enum_generic() {
        // Define an enum called `Either` that has two type parameters, `A` and `B`,
        // and two variants, `Left` and `Right`, each of which holds a value of type
        // `A` or `B` respectively.
        #[derive(Debug, PartialEq, Eq)]
        enum Either {}

        // let left = Either::Left(42);
        // let right = Either::Right("foo");

        assert_eq!(todo!("left") as Either, todo!("Either::Left(42)"));
        assert_eq!(todo!("right") as Either, todo!("Either::Right(\"foo\")"));
    }
}

mod standard {
    #[test]
    fn string_type() {
        // Create a `String` from a string literal.
        let s: String = todo!("\"Hello, world!\"");

        assert_eq!(s, "Hello, world!".to_owned());
    }

    #[test]
    fn string_slice() {
        // Create a read-only substring from the following string slice.
        let s: &str = "Hello, world!";

        assert_eq!(todo!("&s[0..5]") as &str, "Hello");
    }

    #[test]
    fn vector_type() {
        // Create a `Vec<i32>` from a list of numbers.
        let v: Vec<i32> = todo!("vec![1, 2, 3]");

        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn vector_slice() {
        // Create a read-only slice from the following vector.
        let v: Vec<i32> = vec![1, 2, 3];

        assert_eq!(todo!("&v[0..2]") as &[i32], &[1, 2]);
    }
}
