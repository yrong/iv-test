pub mod traits;
pub mod iters;
pub mod inner;

/// Get a random list, randomly distributed from 0 to n-1, no repeat. 
/// 获得一个随即列表，元素随即分布在0 ~ n-1，元素不能重复
pub fn random(n: u32) -> Vec<u32> {
    Vec::new()
}

fn main() {
    // this is a problem of basic algorithm.
    let random = random(1000);
    println!("{:?}", &random);
}
