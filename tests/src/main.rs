use creusot_contracts::{logic::Int, *};

#[ensures(forall<i: Int> 0 <= i && i < (^v)@.len() ==> (^v)[i] == 0u32)]
#[ensures(v@.len() == (^v)@.len())]
pub fn all_zero(v: &mut Vec<u32>) {
    let old_v = ghost! { v };
    #[invariant(v@.len() == old_v@.len())]
    #[invariant(forall<j: Int> 0 <= j && j < produced.len() ==> v[j] == 0u32)]
    for i in 0..v.len() {
        v[i] = 0;
    }
}

pub fn main () {
    let mut v = Vec::new();
    v.push(1); v.push(2);
    all_zero(&mut v);
    assert_eq!(v[0], 0);
    assert_eq!(v[1], 0);
}