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
        struct Person {
            name: String,
            age: u32,
        }

        let name = "John Doe".to_string();
        let age = 42;

        let person = Person { age, name };

        assert_eq!(std::mem::size_of::<Person>(), 32);
        assert_eq!(person.name, "John Doe");
        assert_eq!(person.age, 42);
    }

    #[test]
    fn basic_tuple_struct_example() {
        // Add `name` and `age` fields to this tuple struct, of type `String` and `u32` respectively.
        struct Person(String, u32);

        let person = Person("John Doe".to_string(), 42);

        let name = "John Doe".to_string();
        let age = 42;
        let person2 = Person(name.clone(), age);

        assert_eq!(std::mem::size_of::<Person>(), 32);
        assert_eq!(person.0, "John Doe");
        assert_eq!(person.1, 42);
        assert_eq!(person2.0, name);
        assert_eq!(person2.1, age);
    }

    #[test]
    fn struct_debug() {
        // Derive the `Debug` trait for this struct, so that it can be printed.
        #[derive(Debug)]
        struct Person {
            name: &'static str,
            age: u32,
        }

        let person = Person {
            name: "John Doe",
            age: 42,
        };

        assert_eq!(
            format!("{:?}", person),
            "Person { name: \"John Doe\", age: 42 }"
        );
    }

    #[test]
    fn struct_eq() {
        // Derive the `PartialEq` and `Eq` traits for this struct, so that it can be compared for
        // equality.
        #[derive(Debug, PartialEq, Eq)]
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

        assert_eq!(person1, person2);
    }

    #[test]
    fn struct_clone() {
        // Derive the `Clone` trait for this struct, so that it can be cloned.
        #[derive(Debug, PartialEq, Eq, Clone)]
        struct Person {
            name: &'static str,
            age: u32,
        }

        let person1 = Person {
            name: "John Doe",
            age: 42,
        };
        let person2 = person1.clone();

        assert_eq!(person1, person2);
    }

    #[test]
    fn struct_clone_deep() {
        // Derive the `Clone` trait for this struct, so that it can be cloned.
        #[derive(Debug, PartialEq, Eq, Clone)]
        struct Person {
            address: Address,
        }

        #[derive(Debug, PartialEq, Eq, Clone)]
        struct Address {
            street: u32,
        }

        let mut old_person: Person = Person {
            address: Address { street: 42 },
        };
        let new_person: Person = old_person.clone();

        old_person.address.street = 0;

        assert_eq!(new_person.address.street, 42);
        assert_eq!(old_person.address.street, 0);
    }

    #[test]
    fn struct_copy() {
        // Derive the `Copy` trait for these structs, so that they can be copied.
        #[derive(Debug, PartialEq, Eq, Copy, Clone)]
        struct Person {
            name: &'static str,
            age: u32,
        }

        let person1 = Person {
            name: "John Doe",
            age: 42,
        };
        // copy happen implicitly and is shallow. Clone is explicit and deep.
        let person2 = person1;

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

        person_to_address.insert(sherlock.clone(), "221B Baker Street");

        let gotten: Option<&&str> = person_to_address.get(&sherlock);

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

        impl Default for Person {
            fn default() -> Self {
                Person {
                    name: "Doe",
                    age: 1,
                }
            }
        }

        let person: Person = Default::default();

        assert_eq!(
            person,
            Person {
                name: "Doe",
                age: 1
            }
        );
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
        let Person { name, age } = person;

        assert_eq!(name, "John Doe");
        assert_eq!(age, 42);
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
        let (name, age) = match person {
            Person { name, age } => (name, age),
        };

        assert_eq!(name, "John Doe");
        assert_eq!(age, 42);
    }

    #[test]
    fn struct_pattern_matching_by_ref() {
        #[derive(Debug, PartialEq, Eq)]
        struct Person {
            name: String,
            age: u32,
        }

        let person = Person {
            name: "John Doe".to_owned(),
            age: 42,
        };

        match person {
            Person {
                name: ref n,
                age: a,
            } => {
                println!("Name: {}", n);
                println!("Age: {}", a);
            }
        }

        // Try the following code, note the problem, and fix it using the `ref` keyword in the pattern
        // match (right before `n`), or by using `match &person` instead of `match person`.
        assert_eq!(person.name, "John Doe");
    }

    /// We can attach constructors for and methods on structs using the `impl` keyword, which must
    /// be in the same module as the struct itself. The `impl` block can be used to define helper
    /// functions that are related to the data type. These functions are called "methods" and are
    /// invoked using the `.` operator, similar to methods in object-oriented languages (even
    /// though Rust is not an object-oriented programming language). Methods provide a very useful
    /// tool to organizing code, aiding comprehension and discovery.
    ///
    /// Methods accept `self` as the first argument, which is a reference to the struct itself,
    /// which the method is being added to.
    ///
    /// The common patterns you will see for the `self` parameter are:
    ///
    /// - `&self` - a shared reference to the struct, useful for read-only methods
    /// - `&mut self` - a mutable reference to the struct, useful for read-write methods
    /// - `self` - takes ownership of the struct, useful for methods that consume the struct
    ///
    /// We will return to this topic in our next section on memory, but for now, just think in terms
    /// of the above patterns when you are writing methods.
    #[test]
    fn struct_impl() {
        #[derive(Debug, PartialEq, Eq)]
        struct Person {
            name: String,
            age: u32,
        }

        impl Person {
            // Define a constructor for Person that takes a name and uses a default age of 0.
            fn newborn(name: String) -> Person {
                Person { name, age: 0 }
            }

            // Define a method on Person that returns the name of the person.
            // This method is a "getter" for the name field, so takes a shared reference to self.
            fn name(&self) -> &str {
                &self.name
            }

            // Define a method on Person that returns the age of the person.
            // This method is a "getter" for the age field, so takes a shared reference to self.
            fn age(&self) -> u32 {
                self.age
            }

            // Define a method on Person that increments the age of the person by 1.
            // This method is a "setter" for the age field, so takes a mutable reference to self.
            fn birthday(&mut self) {
                self.age += 1;
            }
        }

        let mut person = Person::newborn("John Doe".to_owned());

        person.birthday();

        assert_eq!(person.name(), "John Doe");
        assert_eq!(person.age(), 1);
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
        enum Direction {
            North,
            South,
            East,
            West,
        }

        // Define a function that takes a `Direction` and returns a `bool` indicating whether the
        // direction is `North`.
        fn is_north(d: Direction) -> bool {
            d == Direction::North
        }

        assert_eq!(is_north(Direction::North), true);
        assert_eq!(is_north(Direction::South), false);
        assert_eq!(is_north(Direction::East), false);
        assert_eq!(is_north(Direction::West), false);
    }

    #[test]
    fn enum_with_data_example() {
        // Define an enum called `Movement` with the variants `North`, `South`, `East`, and `West`,
        // each of which has an associated `u32` value, which indicates the number of spaces to move
        // in that direction.
        #[derive(Debug, PartialEq, Eq)]
        enum Movement {
            North(u32),
            South(u32),
            East(u32),
            West(u32),
        }

        // Define a function that takes a `Movement` and returns a `u32` indicating how many spaces
        // the movement should move.
        fn spaces(m: Movement) -> u32 {
            #[allow(unreachable_patterns)]
            match m {
                Movement::North(n) => n,
                Movement::South(n) => n,
                Movement::East(n) => n,
                Movement::West(n) => n,
            }
        }

        assert_eq!(spaces(Movement::North(42)), 42);
        assert_eq!(spaces(Movement::South(42)), 42);
        assert_eq!(spaces(Movement::East(42)), 42);
        assert_eq!(spaces(Movement::West(42)), 42);
    }

    #[test]
    fn enum_debug() {
        // Derive the `Debug` trait for this enum, so that it can be printed.
        #[derive(Debug)]
        enum Direction {
            North,
            South,
            East,
            West,
        }

        assert_eq!(format!("{:?}", Direction::North), "North");
        assert_eq!(format!("{:?}", Direction::South), "South");
        assert_eq!(format!("{:?}", Direction::East), "East");
        assert_eq!(format!("{:?}", Direction::West), "West");
    }

    #[test]
    fn enum_eq() {
        // Derive the `PartialEq` and `Eq` traits for this enum, so that it can be compared for
        // equality.
        #[derive(Debug, PartialEq, Eq)]
        enum Direction {
            North,
            South,
            East,
            West,
        }

        assert_eq!(Direction::North == Direction::North, true);
        assert_eq!(Direction::North == Direction::South, false);
        assert_eq!(Direction::North == Direction::East, false);
        assert_eq!(Direction::North == Direction::West, false);
    }

    #[test]
    fn enum_clone() {
        // Derive the `Clone` trait for this enum, so that it can be cloned.
        #[derive(Debug, PartialEq, Eq, Clone)]
        enum Direction {
            North,
            South,
            East,
            West,
        }

        let north = Direction::North;
        let north2 = north.clone();

        assert_eq!(north, north2);
    }

    #[test]
    fn enum_copy() {
        // Derive the `Copy` trait for this enum, so that it can be copied.
        #[derive(Debug, PartialEq, Eq, Copy, Clone)]
        enum Movement {
            North(u32),
            South(u32),
            East(u32),
            West(u32),
        }

        let north = Movement::North(42);

        let north2 = north;

        assert_eq!(north, north2);
    }

    #[test]
    fn enum_hash() {
        #![allow(unused_mut)]

        // Derive the `Hash` trait for this enum, so that it can be used in a `HashMap`.
        #[derive(Debug, PartialEq, Eq, Clone, Hash)]
        enum Detective {
            SherlockHolmes,
            HerculePoirot,
            PhilipMarlowe,
            CAugusteDupin,
        }

        use std::collections::HashMap;

        let mut detective_to_address: HashMap<Detective, &str> = HashMap::new();

        let sherlock = Detective::SherlockHolmes;

        detective_to_address.insert(sherlock.clone(), "221B Baker Street");

        let gotten: Option<&&str> = detective_to_address.get(&sherlock);

        assert_eq!(gotten, Some(&"221B Baker Street"));
    }

    #[test]
    fn enum_default() {
        // Derive the `Default` trait for this enum, so that it can be created with `Default::default()`.
        // Note that you will have to choose which unit to make default with the `#[default]` attribute.
        #[derive(Debug, PartialEq, Eq, Default)]
        enum Direction {
            South,
            East,
            #[default]
            North,
            West,
        }

        let direction: Direction = Default::default();

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
        let result: bool = if let Direction::North = direction {
            true
        } else {
            false
        };

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

        let JobTitle::Engineer { level: level2 } = title else {
            panic!("...")
        };

        assert_eq!(level, 3);
        assert_eq!(level2, 3);
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
        let result: bool = match direction {
            Direction::North => true,
            _ => false,
        };

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
            match c {
                CharacterClass::Thief { power: Power::High } => true,
                _ => false,
            }
        }

        let thief = CharacterClass::Thief { power: Power::High };

        assert_eq!(is_high_powered_thief(thief), true);
    }

    /// We can attach constructors for and methods on enums using the `impl` keyword, which must
    /// be in the same module as the enum itself. The `impl` block can be used to define helper
    /// functions that are related to the data type. As with structs, methods accept `self` as the
    /// first argument, which is a reference to the enum itself, which the method is being added to.
    #[test]
    fn enum_impl() {
        #[derive(Debug, PartialEq, Eq)]
        enum Direction {
            North,
            South,
            East,
            West,
        }

        impl Direction {
            // Define a method on Direction that returns a `bool` indicating whether the direction is
            // `North`.
            fn is_north(&self) -> bool {
                match self {
                    Self::North => true,
                    _ => false,
                }
            }
        }

        let north = Direction::North;

        assert_eq!(north.is_north(), true);
    }
}

/// GENERIC TYPES
///
/// Rust provides a powerful mechanism for abstracting over types, called generics. Generics allow
/// you to define a type that has one or more type parameters, which are filled in with concrete
/// types when the generic type is used. Generics are used extensively in the Rust standard library
/// to provide generic data structures and algorithms.
///
/// In this section, you will learn how to make generic structs and enums.
mod generics {
    #[test]
    fn struct_generic() {
        // Define a struct called `Pair` that has two type parameters, `A` and `B`,
        // and two fields, `a` and `b`, of type `A` and `B` respectively.
        #[derive(Debug, PartialEq, Eq)]
        struct Pair<A, B> {
            a: A,
            b: B,
        }

        let pair = Pair { a: 42, b: "foo" };

        assert_eq!(pair.a, 42);
        assert_eq!(pair.b, "foo");
    }

    #[test]
    fn enum_generic() {
        // Define an enum called `Either` that has two type parameters, `A` and `B`,
        // and two variants, `Left` and `Right`, each of which holds a value of type
        // `A` or `B` respectively.
        #[derive(Debug, PartialEq, Eq)]
        enum Either<A, B> {
            Left(A),
            Right(B),
        }

        let left: Either<i32, &str> = Either::Left(42);
        let right: Either<i32, &str> = Either::Right("foo");

        // with this line, no need to explicitly have the types above.
        assert_ne!(left, right);

        assert_eq!(left, Either::Left(42));
        assert_eq!(right, Either::Right("foo"));
    }
}

/// STANDARD TYPES
///
/// The Rust standard library defines a number of data types that are frequently used in Rust
/// programs. These include `String`, `Vec`, and `HashMap`. In this section, you will learn how
/// to use these types.
mod standard {
    #[test]
    fn string_type() {
        // Create a `String` from a string literal.
        let s: String = "Hello, world!".to_owned();

        assert_eq!(s, "Hello, world!".to_owned());
    }

    #[test]
    fn string_slice() {
        // Create a read-only substring from the following string slice.
        let s: &str = "Hello, world!";

        assert_eq!(&s[0..5], "Hello");
    }

    #[test]
    fn vector_type() {
        // Create a `Vec<i32>` from a list of numbers.
        let v: Vec<i32> = vec![1, 2, 3];

        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn vector_slice() {
        // Create a read-only slice from the following vector.
        let v: Vec<i32> = vec![1, 2, 3];

        assert_eq!(&v[0..2], &[1, 2]);
    }

    #[test]
    fn hash_map_type() {
        // Create a `HashMap<&str, i32>` from a list of key-value pairs.
        use std::collections::HashMap;

        // Define the map with a vec of tuples, and then using `into_iter().collect`,
        // convert the vec into a HashMap.
        let map: HashMap<&str, i32> = vec![("foo", 42), ("bar", 43), ("baz", 44)]
            .into_iter()
            .collect();

        assert_eq!(map.get("foo"), Some(&42));
        assert_eq!(map.get("bar"), Some(&43));
        assert_eq!(map.get("baz"), Some(&44));
    }
}
