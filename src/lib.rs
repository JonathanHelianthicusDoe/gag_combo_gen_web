#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod consts;

use consts::DEFAULT_GAGS;
use fxhash::{FxHashMap as Map, FxHashSet as Set};
use gag_combo_gen::{
    gag_types::{Gag, GagType, GAG_TYPES},
    gags::hash_gag,
    opt::{k_opt_combos, opt_combo},
};
use lazy_static::lazy_static;
use std::sync::Mutex;

type Args = (u8, u8, bool, bool, u8, u8, u8);

lazy_static! {
    static ref CACHE: Mutex<Map<Args, Vec<i32>>> = Mutex::new(Map::default());
    static ref BEST: Mutex<Vec<i32>> = Mutex::new(Vec::new());
}

fn hash_combo(combo: Vec<Gag>) -> (i32, Set<GagType>) {
    let mut hash = 0;
    let mut type_set = Set::default();
    for g in combo {
        hash <<= 8;
        hash |= hash_gag(&g);
        type_set.insert(g.gag_type);
    }

    (hash as i32, type_set)
}

fn best_get(i: usize) -> Option<i32> {
    match BEST.lock() {
        Ok(lock) => lock.get(i).cloned(),
        _ => None,
    }
}

fn best_put(new_best: Vec<i32>) {
    if let Ok(mut lock) = BEST.lock() {
        *lock = new_best
    }
}

fn cache_get(key: Args) -> Option<Vec<i32>> {
    match CACHE.lock() {
        Ok(lock) => match lock.get(&key) {
            Some(c) => Some(c.clone()),
            _ => None,
        },
        _ => None,
    }
}

fn cache_put(key: Args, val: Vec<i32>) {
    if let Ok(mut lock) = CACHE.lock() {
        lock.insert(key, val);
    }
}

fn cache_put_all(
    args: Args,
    gag_types: i32,
    combo_hashes: &[i32],
    combo_types: Set<GagType>,
) {
    let combo_types_mask = combo_types.iter().fold(0, |m, &gt| {
        let n: u8 = gt.into();

        m | 1 << n
    });
    let asked_types = GAG_TYPES
        .iter()
        .filter(|&&gt| {
            let n: u8 = gt.into();

            gag_types & 1 << n != 0
        })
        .cloned()
        .collect::<Set<GagType>>();
    let unused_types = (&asked_types - &combo_types)
        .into_iter()
        .collect::<Vec<GagType>>();
    for bs in 0..1 << unused_types.len() {
        let mut gag_type_mask = combo_types_mask;
        for (i, &gt) in unused_types.iter().enumerate() {
            if bs & 1 << i != 0 {
                let n: u8 = gt.into();
                gag_type_mask |= 1 << n;
            }
        }

        let mut new_args = args;
        for k in 1..=args.0 {
            new_args.0 = k;
            new_args.5 = gag_type_mask;
            cache_put(new_args, combo_hashes.to_owned());
        }
    }
}

#[no_mangle]
pub extern "C" fn get(i: i32) -> i32 {
    best_get(i as usize).unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn gen(
    k: i32,
    cog_level: i32,
    is_lured: i32,
    is_v2: i32,
    gag_count: i32,
    org_count: i32,
    gag_types: i32,
) {
    let args = (
        k as u8,
        cog_level as u8,
        is_lured != 0,
        is_v2 != 0,
        gag_count as u8,
        org_count as u8,
        gag_types as u8,
    );

    if let Some(combo_hashes) = cache_get(args) {
        best_put(combo_hashes);

        return;
    }

    let gags = DEFAULT_GAGS
        .iter()
        .filter(|g| {
            if g.gag_type == GagType::PassGag {
                true
            } else {
                let n: u8 = g.gag_type.into();

                args.6 & 1 << n != 0
            }
        })
        .cloned()
        .collect::<Vec<Gag>>();
    let (combo_hashes, combo_types) = if k == 1 {
        if let Some(combo) =
            opt_combo(&gags, args.1, args.2, args.3, args.4, args.5)
        {
            let (combo_hash, combo_types) = hash_combo(combo);

            (vec![combo_hash], combo_types)
        } else {
            (Vec::new(), Set::default())
        }
    } else {
        k_opt_combos(args.0, &gags, args.1, args.2, args.3, args.4, args.5)
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
