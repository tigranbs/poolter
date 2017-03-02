#![crate_name="poolter"]
#![allow(dead_code)]
mod poolter;
mod service;

pub use poolter::PoolTer;

#[test]
fn concurrent_tasks() {
    let mut poolter = PoolTer::init();
    poolter.exec(Box::new(move || {
        println!("Printing First !!");
    })).then(Box::new(move || {
        println!("Second");
    }));

    poolter.exec(Box::new(move || {
        println!("Printing First3 !!");
    })).then(Box::new(move || {
        println!("Second 2");
    }));
}
