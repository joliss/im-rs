#![allow(unused_must_use)]

#[macro_use]
extern crate criterion;
extern crate im;
extern crate rand;

use criterion::Criterion;
use rand::{distributions as dist, Rng};
use std::iter::FromIterator;

use im::iter::unfold_mut;
use im::Vector;

fn rando<A>() -> impl Iterator<Item = A>
where
    dist::Standard: dist::Distribution<A>,
{
    unfold_mut(rand::thread_rng(), |rng| Some(rng.gen()))
}

fn vector(c: &mut Criterion) {
    let inputs = [16, 32, 64, 128, 256, 1024, 16384, 65536];

    c.bench_function_over_inputs(
        "Vector::push_front",
        move |b, count| {
            b.iter_with_setup(Vector::new, |mut vec| {
                for i in 0..*count {
                    vec.push_front(i);
                }
            })
        },
        inputs.iter().cloned(),
    );

    c.bench_function_over_inputs(
        "Vector::push_back",
        move |b, count| {
            b.iter_with_setup(Vector::new, |mut vec| {
                for i in 0..*count {
                    vec.push_back(i);
                }
            })
        },
        inputs.iter().cloned(),
    );

    c.bench_function_over_inputs(
        "Vector::pop_front",
        move |b, count| {
            b.iter_with_setup(
                || Vector::from_iter(0..*count),
                |mut vec| {
                    for _ in 0..*count {
                        vec.pop_front();
                    }
                },
            )
        },
        inputs.iter().cloned(),
    );

    c.bench_function_over_inputs(
        "Vector::pop_back",
        move |b, count| {
            b.iter_with_setup(
                || Vector::from_iter(0..*count),
                |mut vec| {
                    for _ in 0..*count {
                        vec.pop_back();
                    }
                },
            )
        },
        inputs.iter().cloned(),
    );

    c.bench_function_over_inputs(
        "Vector::index_seq",
        move |b, count| {
            b.iter_with_setup(
                || Vector::from_iter(0..*count),
                |vec| {
                    for i in 0..*count {
                        vec.get(i);
                    }
                },
            )
        },
        inputs.iter().cloned(),
    );

    c.bench_function_over_inputs(
        "Vector::index_rand",
        move |b, count| {
            b.iter_with_setup(
                || {
                    (
                        Vector::from_iter(0..*count),
                        rando().take(*count).collect::<Vec<_>>(),
                    )
                },
                |(vec, idx)| {
                    for i in idx {
                        vec.get(i);
                    }
                },
            )
        },
        inputs.iter().cloned(),
    );

    c.bench_function_over_inputs(
        "Vector::set_seq",
        move |b, count| {
            b.iter_with_setup(
                || Vector::from_iter(0..*count),
                |mut vec| {
                    for i in 0..*count {
                        vec.set(i, i);
                    }
                },
            )
        },
        inputs.iter().cloned(),
    );

    c.bench_function_over_inputs(
        "Vector::append",
        move |b, count| {
            b.iter_with_setup(
                || (Vector::from_iter(0..*count), Vector::from_iter(0..*count)),
                |(mut left, right)| left.append(right),
            )
        },
        inputs.iter().cloned(),
    );

    c.bench_function_over_inputs(
        "Vector::split",
        move |b, count| {
            b.iter_with_setup(
                || Vector::from_iter(0..*count),
                |mut vec| vec.split_off(count / 2),
            )
        },
        inputs.iter().cloned(),
    );

    c.bench_function_over_inputs(
        "Vector::eq",
        move |b, count| {
            b.iter_with_setup(
                || (Vector::from_iter(0..*count), Vector::from_iter(0..*count)),
                |(left, right)| left == right,
            )
        },
        inputs.iter().cloned(),
    );

    c.bench_function("Vector::sort/1024", move |b| {
        b.iter_with_setup(
            || rando().take(1024).collect::<Vector<u32>>(),
            |mut vec| vec.sort(),
        )
    });
}

criterion_group!(benches, vector);
criterion_main!(benches);
