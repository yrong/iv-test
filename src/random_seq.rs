//https://preshing.com/20121224/how-to-generate-a-sequence-of-unique-random-integers/
use std::num::Wrapping;
pub struct RandomSequenceOfUnique {
    m_index: u32,
    m_intermediate_offset: u32,
}
impl RandomSequenceOfUnique {
    pub fn new(seed_base: u32, seed_offset: u32) -> RandomSequenceOfUnique {
        RandomSequenceOfUnique {
            m_index: RandomSequenceOfUnique::permute_qpr(RandomSequenceOfUnique::permute_qpr(seed_base) + 0x682f0161),
            m_intermediate_offset: RandomSequenceOfUnique::permute_qpr(RandomSequenceOfUnique::permute_qpr(seed_offset) + 0x46790905)
        }
    }
    pub fn next(&mut self) -> u32 {
        let i = self.by_index(self.m_index);
        self.m_index = (Wrapping(self.m_index) + Wrapping(1)).0;
        i
    }
    pub fn by_index(&self, index: u32) -> u32 {
        RandomSequenceOfUnique::permute_qpr((Wrapping(RandomSequenceOfUnique::permute_qpr(index)) + Wrapping(self.m_intermediate_offset)).0 ^ 0x5bf03635)
    }
    fn permute_qpr(x: u32) -> u32 {
        let x: u64 = x.into();
        let prime = 4294967291;
        if x >= prime.into() {x as u32}
        else {
            let residue = (Wrapping(x) * Wrapping(x)).0 % prime;
            if x <= (prime / 2) {residue as u32}
            else {(prime - residue) as u32}
        }
    }
}