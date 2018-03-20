mod consts;

extern crate gag_combo_gen;
#[macro_use]
extern crate lazy_static;
extern crate fnv;

use consts::{DEFAULT_GAGS, GAG_HASHES};
use fnv::FnvHashMap as Map;
use gag_combo_gen::gag_types::Gag;
use gag_combo_gen::opt::opt_combo;
use std::sync::Mutex;


lazy_static! {
    static ref CACHE: Mutex<Map<(u8, bool, bool, u8, u8), i32>> =
        Mutex::new(Map::default());
}


fn hash_gag(gag: &Gag) -> u32 {
    GAG_HASHES[gag.name] + if gag.is_org { 35 } else { 0 }
}

fn hash_combo(combo: &Vec<Gag>) -> i32 {
    let mut hash = 0u32;
    for g in combo {
        hash <<= 8;
        hash |= hash_gag(g);
    }
    hash as i32
}

fn cache_get(key: &(u8, bool, bool, u8, u8)) -> Option<i32> {
    match CACHE.lock() {
        Ok(lock) => match lock.get(key) {
            Some(&c) => Some(c),
            _        => None,
        },
        _ => None,
    }
}

fn cache_put(key: (u8, bool, bool, u8, u8), val: i32) {
    match CACHE.lock() {
        Ok(mut lock) => { lock.insert(key, val); },
        _            => (),
    }
}

#[no_mangle]
pub extern fn gen(cog_level: i32,
                  is_lured:  i32,
                  is_v2:     i32,
                  gag_count: i32,
                  org_count: i32) -> i32
{
    let args = (cog_level as u8,
                is_lured != 0,
                is_v2 != 0,
                gag_count as u8,
                org_count as u8);

    if let Some(combo_hash) = cache_get(&args) {
        combo_hash
    } else if let Some(combo) = opt_combo(&DEFAULT_GAGS,
                                          args.0,
                                          args.1,
                                          args.2,
                                          args.3,
                                          args.4)
    {
        let combo_hash = hash_combo(&combo);
        cache_put(args, combo_hash);
        combo_hash
    } else {
        cache_put(args, 0);
        0
    }
}
