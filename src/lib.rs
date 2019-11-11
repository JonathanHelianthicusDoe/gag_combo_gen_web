#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![deny(deprecated)]

use fxhash::{FxHashMap as Map, FxHashSet as Set};
use gag_combo_gen::{
    gag_types::{Gag, GagType, GAG_TYPES},
    gags::{hash_gag, DEFAULT_GAGS},
    opt::{k_opt_combos, opt_combo},
};
use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Args {
    k:          u8,
    cog_level:  u8,
    is_lured:   bool,
    is_v2:      bool,
    toon_count: u8,
    org_count:  u8,
    gag_types:  u8,
}

lazy_static! {
    static ref CACHE: Mutex<Map<Args, Vec<u32>>> = Mutex::new(Map::default());
    static ref BEST: Mutex<Vec<u32>> = Mutex::new(Vec::new());
}

fn hash_combo(combo: Vec<Gag>) -> (u32, Set<GagType>) {
    let mut hash = 0;
    let mut type_set = Set::default();
    for g in combo {
        hash <<= 8;
        hash |= hash_gag(&g);
        type_set.insert(g.gag_type);
    }

    (hash, type_set)
}

fn best_get(i: usize) -> Option<u32> {
    match BEST.lock() {
        Ok(lock) => lock.get(i).cloned(),
        _ => None,
    }
}

fn best_put(new_best: Vec<u32>) {
    if let Ok(mut lock) = BEST.lock() {
        *lock = new_best
    }
}

fn cache_get(key: Args) -> Option<Vec<u32>> {
    match CACHE.lock() {
        Ok(lock) => match lock.get(&key) {
            Some(c) => Some(c.clone()),
            _ => None,
        },
        _ => None,
    }
}

fn cache_put(key: Args, val: Vec<u32>) {
    if let Ok(mut lock) = CACHE.lock() {
        lock.insert(key, val);
    }
}

fn cache_put_all(
    args: Args,
    gag_types: u32,
    combo_hashes: &[u32],
    combo_types: Set<GagType>,
) {
    let combo_types_mask = combo_types.iter().fold(0, |m, &gt| {
        let n: u8 = gt.into();

        m | 1 << n
    });
    let asked_types: Set<_> = GAG_TYPES
        .iter()
        .filter(|&&gt| {
            let n: u8 = gt.into();

            gag_types & 1 << n != 0
        })
        .cloned()
        .collect();
    let unused_types: Vec<_> =
        (&asked_types - &combo_types).into_iter().collect();
    for bs in 0..1 << unused_types.len() {
        let mut gag_type_mask = combo_types_mask;
        for (i, &gt) in unused_types.iter().enumerate() {
            if bs & 1 << i != 0 {
                let n: u8 = gt.into();
                gag_type_mask |= 1 << n;
            }
        }

        let mut new_args = args.clone();
        for k in 1..=args.k {
            new_args.k = k;
            new_args.gag_types = gag_type_mask;
            cache_put(new_args.clone(), combo_hashes.to_owned());
        }
    }
}

#[no_mangle]
pub extern "C" fn get(i: u32) -> u32 {
    best_get(i as usize).unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn gen(
    k: u32,
    cog_level: u32,
    is_lured: u32,
    is_v2: u32,
    toon_count: u32,
    org_count: u32,
    gag_types: u32,
) {
    let args = Args {
        k:          k as u8,
        cog_level:  cog_level as u8,
        is_lured:   is_lured != 0,
        is_v2:      is_v2 != 0,
        toon_count: toon_count as u8,
        org_count:  org_count as u8,
        gag_types:  gag_types as u8,
    };

    if let Some(combo_hashes) = cache_get(args.clone()) {
        best_put(combo_hashes);

        return;
    }

    let gags: Vec<_> = DEFAULT_GAGS
        .iter()
        .filter(|g| {
            if g.gag_type == GagType::PassGag {
                true
            } else {
                let n: u8 = g.gag_type.into();

                args.gag_types & 1 << n != 0
            }
        })
        .cloned()
        .collect();
    let (combo_hashes, combo_types) = if k == 1 {
        if let Some(combo) = opt_combo(
            &gags,
            args.cog_level,
            args.is_lured,
            args.is_v2,
            args.toon_count,
            args.org_count,
        ) {
            let (combo_hash, combo_types) = hash_combo(combo);

            (vec![combo_hash], combo_types)
        } else {
            (Vec::new(), Set::default())
        }
    } else {
        k_opt_combos(
            args.k,
            &gags,
            args.cog_level,
            args.is_lured,
            args.is_v2,
            args.toon_count,
            args.org_count,
        )
        .into_iter()
        .fold(
            (Vec::with_capacity(k as usize), Set::default()),
            |(mut h, t), c| {
                let (c_h, c_t) = hash_combo(c);
                h.push(c_h);

                (h, &t | &c_t)
            },
        )
    };

    cache_put_all(args, gag_types, &combo_hashes, combo_types);
    best_put(combo_hashes);
}
