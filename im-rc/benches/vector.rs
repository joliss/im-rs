// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(test)]

extern crate im_rc;
extern crate rand;
extern crate test;

use im_rc::iter::unfold_mut;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::iter::FromIterator;
use test::Bencher;

use im_rc::vector::Vector;

fn rando<A>() -> impl Iterator<Item = A>
where
    Standard: Distribution<A>,
{
    let rng = rand::thread_rng();
    unfold_mut(rng, |rng| Some(rng.gen()))
}

// fn vector_push_front(b: &mut Bencher, count: usize) {
//     b.iter(|| {
//         let mut l = Vector::new();
//         for i in 0..count {
//             l = l.push_front(i)
//         }
//     })
// }

// #[bench]
// fn vector_push_front_10(b: &mut Bencher) {
//     vector_push_front(b, 10)
// }

// #[bench]
// fn vector_push_front_100(b: &mut Bencher) {
//     vector_push_front(b, 100)
// }

// #[bench]
// fn vector_push_front_1000(b: &mut Bencher) {
//     vector_push_front(b, 1000)
// }

// fn vector_push_back(b: &mut Bencher, count: usize) {
//     b.iter(|| {
//         let mut l = Vector::new();
//         for i in 0..count {
//             l = l.push_back(i)
//         }
//     })
// }

// #[bench]
// fn vector_push_back_10(b: &mut Bencher) {
//     vector_push_back(b, 10)
// }

// #[bench]
// fn vector_push_back_100(b: &mut Bencher) {
//     vector_push_back(b, 100)
// }

// #[bench]
// fn vector_push_back_1000(b: &mut Bencher) {
//     vector_push_back(b, 1000)
// }

// fn vector_pop_front(b: &mut Bencher, count: usize) {
//     let l = Vector::from_iter(0..(count + 1));
//     b.iter(|| {
//         let mut p = l.clone();
//         for _ in 0..count {
//             p = p.pop_front().unwrap().1
//         }
//     })
// }

// #[bench]
// fn vector_pop_front_10(b: &mut Bencher) {
//     vector_pop_front(b, 10)
// }

// #[bench]
// fn vector_pop_front_100(b: &mut Bencher) {
//     vector_pop_front(b, 100)
// }

// #[bench]
// fn vector_pop_front_1000(b: &mut Bencher) {
//     vector_pop_front(b, 1000)
// }

// fn vector_pop_back(b: &mut Bencher, count: usize) {
//     let l = Vector::from_iter(0..(count + 1));
//     b.iter(|| {
//         let mut p = l.clone();
//         for _ in 0..count {
//             p = p.pop_back().unwrap().1
//         }
//     })
// }

// #[bench]
// fn vector_pop_back_10(b: &mut Bencher) {
//     vector_pop_back(b, 10)
// }

// #[bench]
// fn vector_pop_back_100(b: &mut Bencher) {
//     vector_pop_back(b, 100)
// }

// #[bench]
// fn vector_pop_back_1000(b: &mut Bencher) {
//     vector_pop_back(b, 1000)
// }

// fn vector_append(b: &mut Bencher, count: usize) {
//     let size = Vec::from_iter((0..count).into_iter().map(|i| Vector::from_iter(0..i)));
//     b.iter(|| {
//         for item in &size {
//             item.append(item.clone());
//         }
//     })
// }

// #[bench]
// fn vector_append_10(b: &mut Bencher) {
//     vector_append(b, 10)
// }

// #[bench]
// fn vector_append_100(b: &mut Bencher) {
//     vector_append(b, 100)
// }

// #[bench]
// fn vector_append_1000(b: &mut Bencher) {
//     vector_append(b, 1000)
// }

fn vector_push_front_mut(b: &mut Bencher, count: usize) {
    b.iter(|| {
        let mut l = Vector::new();
        for i in 0..count {
            l.push_front(i);
        }
    })
}

#[bench]
fn vector_push_front_mut_10(b: &mut Bencher) {
    vector_push_front_mut(b, 10)
}

#[bench]
fn vector_push_front_mut_100(b: &mut Bencher) {
    vector_push_front_mut(b, 100)
}

#[bench]
fn vector_push_front_mut_1000(b: &mut Bencher) {
    vector_push_front_mut(b, 1000)
}

#[bench]
fn vector_push_front_mut_100000(b: &mut Bencher) {
    vector_push_front_mut(b, 100_000)
}

fn vector_push_back_mut(b: &mut Bencher, count: usize) {
    b.iter(|| {
        let mut l = Vector::new();
        for i in 0..count {
            l.push_back(i);
        }
    })
}

#[bench]
fn vector_push_back_mut_10(b: &mut Bencher) {
    vector_push_back_mut(b, 10)
}

#[bench]
fn vector_push_back_mut_100(b: &mut Bencher) {
    vector_push_back_mut(b, 100)
}

#[bench]
fn vector_push_back_mut_1000(b: &mut Bencher) {
    vector_push_back_mut(b, 1000)
}

#[bench]
fn vector_push_back_mut_100000(b: &mut Bencher) {
    vector_push_back_mut(b, 100_000)
}

fn vector_pop_front_mut(b: &mut Bencher, count: usize) {
    let l = Vector::from_iter(0..count);
    b.iter(|| {
        let mut p = l.clone();
        for _ in 0..count {
            p.pop_front();
        }
    })
}

#[bench]
fn vector_pop_front_mut_10(b: &mut Bencher) {
    vector_pop_front_mut(b, 10)
}

#[bench]
fn vector_pop_front_mut_100(b: &mut Bencher) {
    vector_pop_front_mut(b, 100)
}

#[bench]
fn vector_pop_front_mut_1000(b: &mut Bencher) {
    vector_pop_front_mut(b, 1000)
}

#[bench]
fn vector_pop_front_mut_100000(b: &mut Bencher) {
    vector_pop_front_mut(b, 100_000)
}

fn vector_pop_back_mut(b: &mut Bencher, count: usize) {
    let l = Vector::from_iter(0..count);
    b.iter(|| {
        let mut p = l.clone();
        for _ in 0..count {
            p.pop_back();
        }
    })
}

#[bench]
fn vector_pop_back_mut_10(b: &mut Bencher) {
    vector_pop_back_mut(b, 10)
}

#[bench]
fn vector_pop_back_mut_100(b: &mut Bencher) {
    vector_pop_back_mut(b, 100)
}

#[bench]
fn vector_pop_back_mut_1000(b: &mut Bencher) {
    vector_pop_back_mut(b, 1000)
}

#[bench]
fn vector_pop_back_mut_100000(b: &mut Bencher) {
    vector_pop_back_mut(b, 100_000)
}

fn vector_split(b: &mut Bencher, count: usize) {
    let vec = Vector::from_iter(0..count);
    b.iter(|| vec.clone().split_off(count / 2))
}

#[bench]
fn vector_split_10(b: &mut Bencher) {
    vector_split(b, 10)
}

#[bench]
fn vector_split_100(b: &mut Bencher) {
    vector_split(b, 100)
}

#[bench]
fn vector_split_1000(b: &mut Bencher) {
    vector_split(b, 1000)
}

#[bench]
fn vector_split_100000(b: &mut Bencher) {
    vector_split(b, 100_000)
}

fn vector_append(b: &mut Bencher, count: usize) {
    let vec1 = Vector::from_iter(0..count / 2);
    let vec2 = Vector::from_iter(count / 2..count);
    b.iter(|| {
        let mut vec = vec1.clone();
        vec.append(vec2.clone());
    })
}

#[bench]
fn vector_append_10(b: &mut Bencher) {
    vector_append(b, 10)
}

#[bench]
fn vector_append_100(b: &mut Bencher) {
    vector_append(b, 100)
}

#[bench]
fn vector_append_1000(b: &mut Bencher) {
    vector_append(b, 1000)
}

#[bench]
fn vector_append_100000(b: &mut Bencher) {
    vector_append(b, 100_000)
}

fn vector_iter(b: &mut Bencher, count: usize) {
    let vec: Vector<i32> = rando().take(count).collect();
    b.iter(|| {
        let it = vec.iter();
        for _ in it {}
    })
}

#[bench]
fn vector_iter_10(b: &mut Bencher) {
    vector_iter(b, 10)
}

#[bench]
fn vector_iter_100(b: &mut Bencher) {
    vector_iter(b, 100)
}

#[bench]
fn vector_iter_1000(b: &mut Bencher) {
    vector_iter(b, 1000)
}

#[bench]
fn vector_iter_100000(b: &mut Bencher) {
    vector_iter(b, 100_000)
}

fn vector_get_seq(b: &mut Bencher, count: usize) {
    let vec: Vector<i32> = rando().take(count).collect();
    b.iter(|| {
        for i in 0..count {
            let _ = vec.get(i);
        }
    })
}

#[bench]
fn vector_get_seq_10(b: &mut Bencher) {
    vector_get_seq(b, 10)
}

#[bench]
fn vector_get_seq_100(b: &mut Bencher) {
    vector_get_seq(b, 100)
}

#[bench]
fn vector_get_seq_1000(b: &mut Bencher) {
    vector_get_seq(b, 1000)
}

#[bench]
fn vector_get_seq_100000(b: &mut Bencher) {
    vector_get_seq(b, 100_000)
}

fn vector_get_seq_focus(b: &mut Bencher, count: usize) {
    let vec: Vector<i32> = rando().take(count).collect();
    let mut focus = vec.focus();
    b.iter(|| {
        for i in 0..count {
            let _ = focus.get(i);
        }
    })
}

#[bench]
fn vector_get_seq_focus_10(b: &mut Bencher) {
    vector_get_seq_focus(b, 10)
}

#[bench]
fn vector_get_seq_focus_100(b: &mut Bencher) {
    vector_get_seq_focus(b, 100)
}

#[bench]
fn vector_get_seq_focus_1000(b: &mut Bencher) {
    vector_get_seq_focus(b, 1000)
}

#[bench]
fn vector_get_seq_focus_100000(b: &mut Bencher) {
    vector_get_seq_focus(b, 100_000)
}

fn vector_get_seq_focus_mut(b: &mut Bencher, count: usize) {
    let mut vec: Vector<i32> = rando().take(count).collect();
    let mut focus = vec.focus_mut();
    b.iter(|| {
        for i in 0..count {
            let _ = focus.get(i);
        }
    })
}

#[bench]
fn vector_get_seq_focus_mut_10(b: &mut Bencher) {
    vector_get_seq_focus_mut(b, 10)
}

#[bench]
fn vector_get_seq_focus_mut_100(b: &mut Bencher) {
    vector_get_seq_focus_mut(b, 100)
}

#[bench]
fn vector_get_seq_focus_mut_1000(b: &mut Bencher) {
    vector_get_seq_focus_mut(b, 1000)
}

#[bench]
fn vector_get_seq_focus_mut_100000(b: &mut Bencher) {
    vector_get_seq_focus_mut(b, 100_000)
}

// fn vector_extend(b: &mut Bencher, count: usize) {
//     let vec = Vec::from_iter(0..count);
//     b.iter(|| {
//         let mut l: Vector<usize> = Vector::new();
//         l.extend(vec.iter());
//     })
// }

// #[bench]
// fn vector_extend_10(b: &mut Bencher) {
//     vector_extend(b, 10)
// }

// #[bench]
// fn vector_extend_100(b: &mut Bencher) {
//     vector_extend(b, 100)
// }

// #[bench]
// fn vector_extend_1000(b: &mut Bencher) {
//     vector_extend(b, 1000)
// }
