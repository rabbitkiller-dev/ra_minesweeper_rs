fn main() {
    println!("Hello, world!");
    let mut vv = vec![0; 2];
    let val = &vv[0];
    println!("vv: {:?}", vv);
    println!("val: {val}");
}






fn main2() {
    println!("Hello, world!");
    let mut vv = vec![0; 2];
    let val = &mut vv[0];
    *val = 1;
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
        let old = &self.vv[0][0];
        {
            add(&mut (self.vv[0][0]), 1);
        }
        let new = self.getVal();
        println!("{} -- {}", old, new);
    }
}

fn add(a: &mut i32, b: i32) {
    *a += b;
}
