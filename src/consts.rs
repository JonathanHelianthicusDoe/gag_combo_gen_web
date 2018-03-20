use gag_combo_gen::gags::PASS;
use gag_combo_gen::gag_types::Gag;
use gag_combo_gen::gag_types::GagType::{
    DropGag, SoundGag, SquirtGag, ThrowGag, TrapGag,
};
use fnv::FnvHashMap as Map;


lazy_static! {
    pub static ref DEFAULT_GAGS: Vec<Gag> = vec![
        PASS,
        Gag { name: "flowerpot",        gag_type: DropGag,   is_org: false, base_dmg: 10  },
        Gag { name: "banana_peel",      gag_type: TrapGag,   is_org: false, base_dmg: 12  },
        Gag { name: "bikehorn",         gag_type: SoundGag,  is_org: false, base_dmg: 4   },
        Gag { name: "squirting_flower", gag_type: SquirtGag, is_org: false, base_dmg: 4   },
        Gag { name: "cupcake",          gag_type: ThrowGag,  is_org: false, base_dmg: 6   },
        Gag { name: "flowerpot",        gag_type: DropGag,   is_org: true,  base_dmg: 11  },
        Gag { name: "banana_peel",      gag_type: TrapGag,   is_org: true,  base_dmg: 13  },
        Gag { name: "bikehorn",         gag_type: SoundGag,  is_org: true,  base_dmg: 4   },
        Gag { name: "squirting_flower", gag_type: SquirtGag, is_org: true,  base_dmg: 4   },
        Gag { name: "cupcake",          gag_type: ThrowGag,  is_org: true,  base_dmg: 6   },
        Gag { name: "sandbag",          gag_type: DropGag,   is_org: false, base_dmg: 18  },
        Gag { name: "rake",             gag_type: TrapGag,   is_org: false, base_dmg: 20  },
        Gag { name: "whistle",          gag_type: SoundGag,  is_org: false, base_dmg: 7   },
        Gag { name: "glass_of_water",   gag_type: SquirtGag, is_org: false, base_dmg: 8   },
        Gag { name: "fruit_pie_slice",  gag_type: ThrowGag,  is_org: false, base_dmg: 10  },
        Gag { name: "sandbag",          gag_type: DropGag,   is_org: true,  base_dmg: 19  },
        Gag { name: "rake",             gag_type: TrapGag,   is_org: true,  base_dmg: 22  },
        Gag { name: "whistle",          gag_type: SoundGag,  is_org: true,  base_dmg: 7   },
        Gag { name: "glass_of_water",   gag_type: SquirtGag, is_org: true,  base_dmg: 8   },
        Gag { name: "fruit_pie_slice",  gag_type: ThrowGag,  is_org: true,  base_dmg: 11  },
        Gag { name: "anvil",            gag_type: DropGag,   is_org: false, base_dmg: 30  },
        Gag { name: "marbles",          gag_type: TrapGag,   is_org: false, base_dmg: 35  },
        Gag { name: "bugle",            gag_type: SoundGag,  is_org: false, base_dmg: 11  },
        Gag { name: "squirtgun",        gag_type: SquirtGag, is_org: false, base_dmg: 12  },
        Gag { name: "cream_pie_slice",  gag_type: ThrowGag,  is_org: false, base_dmg: 17  },
        Gag { name: "anvil",            gag_type: DropGag,   is_org: true,  base_dmg: 33  },
        Gag { name: "marbles",          gag_type: TrapGag,   is_org: true,  base_dmg: 38  },
        Gag { name: "bugle",            gag_type: SoundGag,  is_org: true,  base_dmg: 12  },
        Gag { name: "squirtgun",        gag_type: SquirtGag, is_org: true,  base_dmg: 13  },
        Gag { name: "cream_pie_slice",  gag_type: ThrowGag,  is_org: true,  base_dmg: 18  },
        Gag { name: "aoogah",           gag_type: SoundGag,  is_org: false, base_dmg: 16  },
        Gag { name: "big_weight",       gag_type: DropGag,   is_org: false, base_dmg: 45  },
        Gag { name: "quicksand",        gag_type: TrapGag,   is_org: false, base_dmg: 50  },
        Gag { name: "seltzer_bottle",   gag_type: SquirtGag, is_org: false, base_dmg: 21  },
        Gag { name: "fruit_pie",        gag_type: ThrowGag,  is_org: false, base_dmg: 27  },
        Gag { name: "aoogah",           gag_type: SoundGag,  is_org: true,  base_dmg: 17  },
        Gag { name: "big_weight",       gag_type: DropGag,   is_org: true,  base_dmg: 49  },
        Gag { name: "quicksand",        gag_type: TrapGag,   is_org: true,  base_dmg: 55  },
        Gag { name: "seltzer_bottle",   gag_type: SquirtGag, is_org: true,  base_dmg: 23  },
        Gag { name: "fruit_pie",        gag_type: ThrowGag,  is_org: true,  base_dmg: 29  },
        Gag { name: "elephant_trunk",   gag_type: SoundGag,  is_org: false, base_dmg: 21  },
        Gag { name: "safe",             gag_type: DropGag,   is_org: false, base_dmg: 60  },
        Gag { name: "trap_door",        gag_type: TrapGag,   is_org: false, base_dmg: 70  },
        Gag { name: "fire_hose",        gag_type: SquirtGag, is_org: false, base_dmg: 30  },
        Gag { name: "cream_pie",        gag_type: ThrowGag,  is_org: false, base_dmg: 40  },
        Gag { name: "elephant_trunk",   gag_type: SoundGag,  is_org: true,  base_dmg: 23  },
        Gag { name: "safe",             gag_type: DropGag,   is_org: true,  base_dmg: 66  },
        Gag { name: "trap_door",        gag_type: TrapGag,   is_org: true,  base_dmg: 77  },
        Gag { name: "fire_hose",        gag_type: SquirtGag, is_org: true,  base_dmg: 33  },
        Gag { name: "cream_pie",        gag_type: ThrowGag,  is_org: true,  base_dmg: 44  },
        Gag { name: "storm_cloud",      gag_type: SquirtGag, is_org: false, base_dmg: 80  },
        Gag { name: "cake",             gag_type: ThrowGag,  is_org: false, base_dmg: 100 },
        Gag { name: "grand_piano",      gag_type: DropGag,   is_org: false, base_dmg: 170 },
        Gag { name: "tnt",              gag_type: TrapGag,   is_org: false, base_dmg: 180 },
        Gag { name: "foghorn",          gag_type: SoundGag,  is_org: false, base_dmg: 50  },
        Gag { name: "storm_cloud",      gag_type: SquirtGag, is_org: true,  base_dmg: 88  },
        Gag { name: "cake",             gag_type: ThrowGag,  is_org: true,  base_dmg: 110 },
        Gag { name: "grand_piano",      gag_type: DropGag,   is_org: true,  base_dmg: 187 },
        Gag { name: "tnt",              gag_type: TrapGag,   is_org: true,  base_dmg: 198 },
        Gag { name: "foghorn",          gag_type: SoundGag,  is_org: true,  base_dmg: 55  },
        Gag { name: "geyser",           gag_type: SquirtGag, is_org: false, base_dmg: 105 },
        Gag { name: "opera_singer",     gag_type: SoundGag,  is_org: false, base_dmg: 90  },
        Gag { name: "wedding_cake",     gag_type: ThrowGag,  is_org: false, base_dmg: 120 },
        Gag { name: "toontanic",        gag_type: DropGag,   is_org: false, base_dmg: 180 },
        Gag { name: "railroad",         gag_type: TrapGag,   is_org: false, base_dmg: 195 },
        Gag { name: "geyser",           gag_type: SquirtGag, is_org: true,  base_dmg: 115 },
        Gag { name: "opera_singer",     gag_type: SoundGag,  is_org: true,  base_dmg: 99  },
        Gag { name: "wedding_cake",     gag_type: ThrowGag,  is_org: true,  base_dmg: 132 },
        Gag { name: "toontanic",        gag_type: DropGag,   is_org: true,  base_dmg: 198 },
        Gag { name: "railroad",         gag_type: TrapGag,   is_org: true,  base_dmg: 214 },
    ];

    pub static ref GAG_HASHES: Map<&'static str, u32> = {
        let mut m = Map::default();

        m.insert("pass",              0);

        m.insert("banana_peel",       1);
        m.insert("rake",              6);
        m.insert("marbles",          11);
        m.insert("quicksand",        16);
        m.insert("trap_door",        21);
        m.insert("tnt",              26);
        m.insert("railroad",         31);

        m.insert("bikehorn",          2);
        m.insert("whistle",           7);
        m.insert("bugle",            12);
        m.insert("aoogah",           17);
        m.insert("elephant_trunk",   22);
        m.insert("foghorn",          27);
        m.insert("opera_singer",     32);

        m.insert("cupcake",           3);
        m.insert("fruit_pie_slice",   8);
        m.insert("cream_pie_slice",  13);
        m.insert("fruit_pie",        18);
        m.insert("cream_pie",        23);
        m.insert("cake",             28);
        m.insert("wedding_cake",     33);

        m.insert("squirting_flower",  4);
        m.insert("glass_of_water",    9);
        m.insert("squirtgun",        14);
        m.insert("seltzer_bottle",   19);
        m.insert("fire_hose",        24);
        m.insert("storm_cloud",      29);
        m.insert("geyser",           34);

        m.insert("flowerpot",         5);
        m.insert("sandbag",          10);
        m.insert("anvil",            15);
        m.insert("big_weight",       20);
        m.insert("safe",             25);
        m.insert("grand_piano",      30);
        m.insert("toontanic",        35);

        m
    };
}
