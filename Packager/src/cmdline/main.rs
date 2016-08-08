#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
mod dependency;

extern crate serde;
extern crate serde_json;

fn main() {
    let my_thing = &dependency::InstanceDep::File {
          file_type: dependency::FileType::Arbitrary {
              location: String::from("othe.png")
          },
        name: String::from("t.txt"),
        location: String::from("test/g.png")
    };

    println!("{}", serde_json::to_string_pretty(my_thing).unwrap());
}