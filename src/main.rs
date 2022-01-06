pub mod traits;
pub mod iters;
pub mod inner;
pub mod random_seq;


/// Get a random list, randomly distributed from 0 to n-1, no repeat. 
/// 获得一个随即列表，元素随即分布在0 ~ n-1，元素不能重复
pub fn random(n: u32) -> Vec<u32> {
    let mut vec = Vec::new();
    let mut r = random_seq::RandomSequenceOfUnique::new(0, 1);
    for _ in 1..101 {
        vec.push(r.next());
    }
    vec
}

fn main() {
    // this is a problem of basic algorithm.
    let random = random(1000);
    println!("{:?}", &random);
}
