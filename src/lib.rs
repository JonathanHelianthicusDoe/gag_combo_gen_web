mod consts;

extern crate gag_combo_gen;
#[macro_use]
extern crate lazy_static;
extern crate fnv;

use consts::{DEFAULT_GAGS, GAG_HASHES};
use fnv::{FnvHashMap as Map, FnvHashSet as Set};
use gag_combo_gen::gag_types::{Gag, GAG_TYPES, GagType};
use gag_combo_gen::opt::opt_combo;
use std::sync::Mutex;


lazy_static! {
    static ref CACHE: Mutex<Map<(u8, bool, bool, u8, u8, u8), i32>> =
        Mutex::new(Map::default());
}


fn hash_gag(gag: &Gag) -> u32 {
    GAG_HASHES[gag.name] + if gag.is_org { 35 } else { 0 }
}

fn hash_combo(combo: Vec<Gag>) -> (i32, Set<GagType>) {
    let mut hash = 0u32;
    let mut type_set = Set::default();
    for g in combo {
        hash <<= 8;
        hash |= hash_gag(&g);
        type_set.insert(g.gag_type);
    }

    (hash as i32, type_set)
}

fn cache_get(key: &(u8, bool, bool, u8, u8, u8)) -> Option<i32> {
    match CACHE.lock() {
        Ok(lock) => match lock.get(key) {
            Some(&c) => Some(c),
            _        => None,
        },
        _ => None,
    }
}

fn cache_put(key: (u8, bool, bool, u8, u8, u8), val: i32) {
    match CACHE.lock() {
        Ok(mut lock) => { lock.insert(key, val); },
        _            => (),
    }
}

fn cache_put_all(args:        (u8, bool, bool, u8, u8, u8),
                 gag_types:   i32,
                 combo_hash:  i32,
                 combo_types: Set<GagType>)
{
    let combo_types_mask = combo_types.iter()
                                      .fold(0, |m, gt| m | 1 << gt.as_u8());
    let asked_types = GAG_TYPES.into_iter()
                               .filter(|gt| gag_types & 1 << gt.as_u8() != 0)
                               .cloned()
                               .collect::<Set<GagType>>();
    let unused_types =
        (&asked_types - &combo_types).into_iter().collect::<Vec<GagType>>();
    for bs in 0..1 << unused_types.len() {
        let mut gag_type_mask = combo_types_mask;
        for (i, gt) in unused_types.iter().enumerate() {
            if bs & 1 << i != 0 {
                gag_type_mask |= 1 << gt.as_u8();
            }
        }

        let mut new_args = args;
        new_args.5 = gag_type_mask;
        cache_put(new_args, combo_hash);
    }
}

#[no_mangle]
pub extern fn gen(cog_level: i32,
                  is_lured:  i32,
                  is_v2:     i32,
                  gag_count: i32,
                  org_count: i32,
                  gag_types: i32) -> i32
{
    let args = (cog_level as u8,
                is_lured != 0,
                is_v2 != 0,
                gag_count as u8,
                org_count as u8,
                gag_types as u8);

    if let Some(combo_hash) = cache_get(&args) {
        return combo_hash;
    }

    let gags = DEFAULT_GAGS.iter()
                           .filter(|g|
        if g.gag_type == GagType::PassGag {
            true
        } else {
            gag_types & (1 << g.gag_type.as_u8()) != 0
        }).cloned()
          .collect::<Vec<Gag>>();
    if let Some(combo) = opt_combo(&gags,
                                   args.0,
                                   args.1,
                                   args.2,
                                   args.3,
                                   args.4)
    {
        let (combo_hash, combo_types) = hash_combo(combo);
        cache_put_all(args, gag_types, combo_hash, combo_types);

        combo_hash
    } else {
        cache_put_all(args, gag_types, 0, Set::default());

        0
    }
}
