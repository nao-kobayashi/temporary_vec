pub mod temporary_vec;
use crate::temporary_vec::TemporaryVec;


fn main() {
    println!("moveする場合");
    sample1();

    println!("cloneする場合");
    sample2();
}

fn sample2() {
    let mut v = vec![0, 1];
    let values = vec![100, 200, 300, 0, 1, 10, 11, 4];
    display(&v);

    let copy = TemporaryVec::onto_from_clone(&mut v, &values);
    display(&copy.to_vec());
    display(&*copy);
    copy.revert();
    
    display(&v);
    display(&values);
}


fn sample1() {
    let mut v = vec![0, 1];
    let values = vec![100, 200, 300, 0, 1, 10, 11, 4];
    display(&v);

    let copy = TemporaryVec::onto(&mut v, values);
    display(&copy.to_vec());
    display(&*copy);
    copy.revert();
    
    display(&v);
}

fn display<T>(disp: &[T]) where T: std::fmt::Debug {
    println!("{:?}", disp);
}