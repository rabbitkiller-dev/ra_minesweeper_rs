use rand::distributions::uniform::SampleBorrow;

fn main() {
    let y = 0;
    let li: [i32; 3] = [-1, 0, 1];
    for i in li {
        println!("{i}");
        if y == 0 {
            continue;
        }
        println!("{i}");
    }
}

fn main4() {
    println!("Hello, world!");
    let mut vv = vec![0; 2];
    let val = &vv[0];
    println!("vv: {:?}", vv);
    println!("val: {val}");
}

use std::cell::RefCell;
use std::rc::Rc;
fn main3() {
    let s = Rc::new(RefCell::new("我很善变，还拥有多个主人".to_string()));

    let s1 = s.clone();
    let s2 = s.clone();
    // let mut s2 = s.borrow_mut();
    s2.borrow_mut().push_str(", on yeah!");

    println!("{:?}\n{:?}\n{:?}", s, s1, s2);

}

fn main2() {
    println!("Hello, world!");
    let mut vv = vec![0; 2];
    let val = &mut vv[0];
    *val = 1;
    let val = val.borrow();
    println!("val: {val}");
    println!("vv: {:?}", vv);
}

struct AAA {
    vv: Vec<Vec<i32>>,
}

impl AAA {
    fn getVal(&self) -> i32 {
        self.vv[0][0]
    }

    fn mutVal(&mut self) {
        let old = &mut (self.vv[0][0]);
        add(old, 1);
        let new = self.getVal();
        // println!("{} -> {}", old, new);
    }
}

fn add(a: &mut i32, b: i32) {
    *a += b;
}
