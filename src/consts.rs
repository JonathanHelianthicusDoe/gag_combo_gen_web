use gag_combo_gen::gags::PASS;
use gag_combo_gen::gag_types::Gag;
use gag_combo_gen::gag_types::GagType::{
    DropGag, SoundGag, SquirtGag, ThrowGag, TrapGag,
};
use fnv::FnvHashMap as Map;


lazy_static! {
    pub static ref DEFAULT_GAGS: Vec<Gag> = vec![
        PASS,

        Gag { name: "banana_peel",      gag_type: TrapGag,   is_org: false, base_dmg: 12,  cost: 10      },
        Gag { name: "bikehorn",         gag_type: SoundGag,  is_org: false, base_dmg: 4,   cost: 12      },
        Gag { name: "flowerpot",        gag_type: DropGag,   is_org: false, base_dmg: 10,  cost: 14      },
        Gag { name: "squirting_flower", gag_type: SquirtGag, is_org: false, base_dmg: 4,   cost: 15      },
        Gag { name: "cupcake",          gag_type: ThrowGag,  is_org: false, base_dmg: 6,   cost: 16      },

        Gag { name: "banana_peel",      gag_type: TrapGag,   is_org: true,  base_dmg: 13,  cost: 15      },
        Gag { name: "bikehorn",         gag_type: SoundGag,  is_org: true,  base_dmg: 4,   cost: 17      },
        Gag { name: "flowerpot",        gag_type: DropGag,   is_org: true,  base_dmg: 11,  cost: 19      },
        Gag { name: "squirting_flower", gag_type: SquirtGag, is_org: true,  base_dmg: 4,   cost: 20      },
        Gag { name: "cupcake",          gag_type: ThrowGag,  is_org: true,  base_dmg: 6,   cost: 21      },

        Gag { name: "rake",             gag_type: TrapGag,   is_org: false, base_dmg: 20,  cost: 100     },
        Gag { name: "whistle",          gag_type: SoundGag,  is_org: false, base_dmg: 7,   cost: 110     },
        Gag { name: "sandbag",          gag_type: DropGag,   is_org: false, base_dmg: 18,  cost: 120     },
        Gag { name: "glass_of_water",   gag_type: SquirtGag, is_org: false, base_dmg: 8,   cost: 140     },
        Gag { name: "fruit_pie_slice",  gag_type: ThrowGag,  is_org: false, base_dmg: 10,  cost: 160     },

        Gag { name: "rake",             gag_type: TrapGag,   is_org: true,  base_dmg: 22,  cost: 150     },
        Gag { name: "whistle",          gag_type: SoundGag,  is_org: true,  base_dmg: 7,   cost: 160     },
        Gag { name: "sandbag",          gag_type: DropGag,   is_org: true,  base_dmg: 19,  cost: 170     },
        Gag { name: "glass_of_water",   gag_type: SquirtGag, is_org: true,  base_dmg: 8,   cost: 190     },
        Gag { name: "fruit_pie_slice",  gag_type: ThrowGag,  is_org: true,  base_dmg: 11,  cost: 210     },

        Gag { name: "marbles",          gag_type: TrapGag,   is_org: false, base_dmg: 35,  cost: 1000    },
        Gag { name: "bugle",            gag_type: SoundGag,  is_org: false, base_dmg: 11,  cost: 1100    },
        Gag { name: "anvil",            gag_type: DropGag,   is_org: false, base_dmg: 30,  cost: 1200    },
        Gag { name: "squirtgun",        gag_type: SquirtGag, is_org: false, base_dmg: 12,  cost: 1400    },
        Gag { name: "cream_pie_slice",  gag_type: ThrowGag,  is_org: false, base_dmg: 17,  cost: 1600    },

        Gag { name: "marbles",          gag_type: TrapGag,   is_org: true,  base_dmg: 38,  cost: 1500    },
        Gag { name: "bugle",            gag_type: SoundGag,  is_org: true,  base_dmg: 12,  cost: 1600    },
        Gag { name: "anvil",            gag_type: DropGag,   is_org: true,  base_dmg: 33,  cost: 1700    },
        Gag { name: "squirtgun",        gag_type: SquirtGag, is_org: true,  base_dmg: 13,  cost: 1900    },
        Gag { name: "cream_pie_slice",  gag_type: ThrowGag,  is_org: true,  base_dmg: 18,  cost: 2100    },

        Gag { name: "aoogah",           gag_type: SoundGag,  is_org: false, base_dmg: 16,  cost: 4000    },
        Gag { name: "quicksand",        gag_type: TrapGag,   is_org: false, base_dmg: 50,  cost: 4500    },
        Gag { name: "big_weight",       gag_type: DropGag,   is_org: false, base_dmg: 45,  cost: 6000    },
        Gag { name: "seltzer_bottle",   gag_type: SquirtGag, is_org: false, base_dmg: 21,  cost: 7000    },
        Gag { name: "fruit_pie",        gag_type: ThrowGag,  is_org: false, base_dmg: 27,  cost: 8000    },

        Gag { name: "aoogah",           gag_type: SoundGag,  is_org: true,  base_dmg: 17,  cost: 6000    },
        Gag { name: "quicksand",        gag_type: TrapGag,   is_org: true,  base_dmg: 55,  cost: 6500    },
        Gag { name: "big_weight",       gag_type: DropGag,   is_org: true,  base_dmg: 49,  cost: 8000    },
        Gag { name: "seltzer_bottle",   gag_type: SquirtGag, is_org: true,  base_dmg: 23,  cost: 9000    },
        Gag { name: "fruit_pie",        gag_type: ThrowGag,  is_org: true,  base_dmg: 29,  cost: 10000   },

        Gag { name: "elephant_trunk",   gag_type: SoundGag,  is_org: false, base_dmg: 21,  cost: 12000   },
        Gag { name: "trap_door",        gag_type: TrapGag,   is_org: false, base_dmg: 70,  cost: 18000   },
        Gag { name: "safe",             gag_type: DropGag,   is_org: false, base_dmg: 60,  cost: 32000   },
        Gag { name: "fire_hose",        gag_type: SquirtGag, is_org: false, base_dmg: 30,  cost: 48000   },
        Gag { name: "cream_pie",        gag_type: ThrowGag,  is_org: false, base_dmg: 40,  cost: 50000   },

        Gag { name: "elephant_trunk",   gag_type: SoundGag,  is_org: true,  base_dmg: 23,  cost: 16000   },
        Gag { name: "trap_door",        gag_type: TrapGag,   is_org: true,  base_dmg: 77,  cost: 24000   },
        Gag { name: "safe",             gag_type: DropGag,   is_org: true,  base_dmg: 66,  cost: 36000   },
        Gag { name: "fire_hose",        gag_type: SquirtGag, is_org: true,  base_dmg: 33,  cost: 56000   },
        Gag { name: "cream_pie",        gag_type: ThrowGag,  is_org: true,  base_dmg: 44,  cost: 60000   },

        Gag { name: "storm_cloud",      gag_type: SquirtGag, is_org: false, base_dmg: 80,  cost: 400000  },
        Gag { name: "cake",             gag_type: ThrowGag,  is_org: false, base_dmg: 100, cost: 600000  },
        Gag { name: "grand_piano",      gag_type: DropGag,   is_org: false, base_dmg: 170, cost: 740000  },
        Gag { name: "tnt",              gag_type: TrapGag,   is_org: false, base_dmg: 180, cost: 766000  },
        Gag { name: "foghorn",          gag_type: SoundGag,  is_org: false, base_dmg: 50,  cost: 820000  },

        Gag { name: "storm_cloud",      gag_type: SquirtGag, is_org: true,  base_dmg: 88,  cost: 470000  },
        Gag { name: "cake",             gag_type: ThrowGag,  is_org: true,  base_dmg: 110, cost: 700000  },
        Gag { name: "grand_piano",      gag_type: DropGag,   is_org: true,  base_dmg: 187, cost: 780000  },
        Gag { name: "tnt",              gag_type: TrapGag,   is_org: true,  base_dmg: 198, cost: 790000  },
        Gag { name: "foghorn",          gag_type: SoundGag,  is_org: true,  base_dmg: 55,  cost: 920000  },

        Gag { name: "geyser",           gag_type: SquirtGag, is_org: false, base_dmg: 105, cost: 4000000 },
        Gag { name: "opera_singer",     gag_type: SoundGag,  is_org: false, base_dmg: 90,  cost: 6000000 },
        Gag { name: "wedding_cake",     gag_type: ThrowGag,  is_org: false, base_dmg: 120, cost: 6010000 },
        Gag { name: "toontanic",        gag_type: DropGag,   is_org: false, base_dmg: 180, cost: 7000000 },
        Gag { name: "railroad",         gag_type: TrapGag,   is_org: false, base_dmg: 195, cost: 7010000 },

        Gag { name: "geyser",           gag_type: SquirtGag, is_org: true,  base_dmg: 115, cost: 4500000 },
        Gag { name: "opera_singer",     gag_type: SoundGag,  is_org: true,  base_dmg: 99,  cost: 6500000 },
        Gag { name: "wedding_cake",     gag_type: ThrowGag,  is_org: true,  base_dmg: 132, cost: 6510000 },
        Gag { name: "toontanic",        gag_type: DropGag,   is_org: true,  base_dmg: 198, cost: 7010000 },
        Gag { name: "railroad",         gag_type: TrapGag,   is_org: true,  base_dmg: 214, cost: 7777777 },
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
