#[macro_use]
extern crate cucumber_rust;

use std::cmp::PartialOrd;
use std::clone::Clone;

pub trait Sorter {
    fn sort(&self, source: &[i64]) -> Box<[i64]>;
}

pub struct SortedCopy {}

impl Sorter for SortedCopy {
    fn sort(&self, source: &[i64]) -> Box<[i64]>
    {
        let mut res = Vec::new();
        for outer in 0..source.len() {
            let mut found = false;
            for inner in 1..res.len() {
                if res[inner - 1] <= source[outer] &&
                    res[inner] > source[outer]
                {
                    res.insert(inner, source[outer]);
                    found = true;
                    break;
                }
            }

            if !found
            {
                res.push(source[outer]);
            }
        }

        res.into_boxed_slice()
    }
}

pub struct MyWorld {
    pub thing: Box<Sorter>
}

impl cucumber_rust::World for MyWorld {}

#[cfg(test)]
mod basic {
    steps!(::MyWorld => {
        when regex "thing (\\d+)" (usize) |world, sz, step| {

        };

        when regex "^test (.*) regex$" |_world, matches, _step| {
            println!("{}", matches[1]);
        };

        given "a thing" |_world, _step| {
            assert!(true);
        };

        when "another thing" |_world, _step| {
            assert!(false);
        };

        when "something goes right" |_world, _step| {
            assert!(true);
        };

        when "something goes wrong" |_world, _step| {
            println!("Something to stdout");
            eprintln!("Something to stderr");
            panic!("This is my custom panic message");
        };

        then "another thing" |_world, _step| {
            assert!(true)
        };
    });
}

fn before_thing(step: &cucumber_rust::Scenario) {

}

before!(some_before: "@tag2 and @tag3" => |scenario| {
    println!("{}", "lol");
});

before!(something_great => |scenario| {

});

after!(after_thing => |scenario| {

});

fn setup() {

}

fn setup_world() -> MyWorld {
    let system = SortedCopy {};

    MyWorld{
        thing: Box::new(system),
    }
}

cucumber! {
    features: "./features",
    world: ::MyWorld,
    steps: &[
        basic::steps
    ],
    setup_world: setup_world,
    setup: setup,
    before: &[before_thing, some_before, something_great],
    after: &[after_thing]
}
