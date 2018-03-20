mod consts;

extern crate gag_combo_gen;
#[macro_use]
extern crate lazy_static;
extern crate fnv;

use consts::{DEFAULT_GAGS, GAG_HASHES};
use gag_combo_gen::gag_types::Gag;
use gag_combo_gen::opt::opt_combo;


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

#[no_mangle]
pub extern fn gen(cog_level: i32,
                  is_lured:  i32,
                  is_v2:     i32,
                  gag_count: i32,
                  org_count: i32) -> i32
{
    if let Some(combo) = opt_combo(&DEFAULT_GAGS,
                                   cog_level as u8,
                                   is_lured != 0,
                                   is_v2 != 0,
                                   gag_count as u8,
                                   org_count as u8)
    {
        hash_combo(&combo)
    } else {
        0
    }
}
