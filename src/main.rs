extern crate web_server;
mod enum_book;
mod search;
mod trait_book;
mod lamdba;
mod iterators;
mod file;
mod threads;

fn main() {
    println!("---------ENUM-------------");
    enum_book::Enums::server();
    println!("---------Trees-------------");
    search::Trees::bst_tree();
    println!("---------Trait-------------");
    trait_book::bank::starts();
    println!("---------Lamdba-------------");
    lamdba::Lamdba::start();
    println!("---------Iterators-------------");
    iterators::Iters::start();
    println!("---------Files-------------");
    file::Files_book::start();
    println!("---------Thread-------------");
    threads::Threads::start();
}
