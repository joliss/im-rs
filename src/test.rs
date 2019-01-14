// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use rand::Rng;
use std::hash::{BuildHasher, Hasher, SipHasher13};
use std::marker::PhantomData;
use typenum::{Unsigned, U64};

pub fn is_sorted<A, I>(l: I) -> bool
where
    I: IntoIterator<Item = A>,
    A: Ord,
{
    let mut it = l.into_iter().peekable();
    loop {
        match (it.next(), it.peek()) {
            (_, None) => return true,
            (Some(ref a), Some(b)) if a > b => return false,
            _ => (),
        }
    }
}

pub struct LolHasher<N: Unsigned = U64> {
    state: u64,
    shift: usize,
    size: PhantomData<N>,
}

impl<N: Unsigned> LolHasher<N> {
    fn feed_me(&mut self, byte: u8) {
        self.state ^= u64::from(byte) << self.shift;
        self.shift += 8;
        if self.shift >= 64 {
            self.shift = 0;
        }
    }
}

impl<N: Unsigned> Hasher for LolHasher<N> {
    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.feed_me(*byte)
        }
    }

    fn finish(&self) -> u64 {
        if N::USIZE == 64 {
            self.state
        } else {
            self.state & ((1 << N::USIZE) - 1)
        }
    }
}

impl<N: Unsigned> Default for LolHasher<N> {
    fn default() -> Self {
        LolBuildHasher::new().build_hasher()
    }
}

pub struct LolBuildHasher<N: Unsigned> {
    seed: u64,
    hasher_size: PhantomData<N>,
}

impl<N: Unsigned> LolBuildHasher<N> {
    pub fn new() -> Self {
        LolBuildHasher {
            seed: 0,
            hasher_size: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub fn with_seed(seed: u64) -> Self {
        LolBuildHasher {
            seed,
            hasher_size: PhantomData,
        }
    }

    pub fn with_random_seed() -> Self {
        LolBuildHasher {
            seed: rand::thread_rng().gen(),
            hasher_size: PhantomData,
        }
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }
}

impl<N: Unsigned> BuildHasher for LolBuildHasher<N> {
    type Hasher = LolHasher<N>;
    fn build_hasher(&self) -> Self::Hasher {
        LolHasher {
            state: self.seed,
            shift: 0,
            size: PhantomData,
        }
    }
}

pub struct SeededSipHasher {
    seed: (u64, u64),
}

impl SeededSipHasher {
    pub fn new() -> Self {
        SeededSipHasher {
            seed: rand::thread_rng().gen(),
        }
    }

    pub fn with_seed(seed: (u64, u64)) -> Self {
        SeededSipHasher { seed }
    }

    pub fn seed(&self) -> (u64, u64) {
        self.seed
    }
}

impl BuildHasher for SeededSipHasher {
    type Hasher = SipHasher13;
    fn build_hasher(&self) -> Self::Hasher {
        SipHasher13::new_with_keys(self.seed.0, self.seed.1)
    }
}
