(function() {
"use strict";

Array.prototype.toString = function() {
    return `[${this.join(", ")}]`;
};

const GAG_NAMES = [
    "pass",
    "banana_peel", "bikehorn",       "cupcake",         "squirting_flower", "flowerpot",
    "rake",        "whistle",        "fruit_pie_slice", "glass_of_water",   "sandbag",
    "marbles",     "bugle",          "cream_pie_slice", "squirtgun",        "anvil",
    "quicksand",   "aoogah",         "fruit_pie",       "seltzer_bottle",   "big_weight",
    "trap_door",   "elephant_trunk", "cream_pie",       "fire_hose",        "safe",
    "tnt",         "foghorn",        "cake",            "storm_cloud",      "grand_piano",
    "railroad",    "opera_singer",   "wedding_cake",    "geyser",           "toontanic"
];

window.onload = function() {
    fetch("wasm/gag_combo_gen_web.wasm")
        .then(r => r.arrayBuffer())
        .then(r => WebAssembly.instantiate(r))
        .then(r => {
            const gag_combo_gen = r.instance.exports;

            const test_res = gag_combo_gen.gen(11, true, false, 3, 1);
            console.log(
                `gag_combo_gen.gen(11, true, false, 3, 1): ${test_res}`
            );
            console.log(`translated: ${translate_combo(3, test_res)}`);
        });
};

function translate_combo(gag_count, gen_result) {
    const combo = [];
    for (let i = 0; i < gag_count; ++i) {
        const hash = gen_result & 0xff;
        gen_result >>= 8;

        const is_org = hash > 35;
        const name = is_org ? GAG_NAMES[hash - 35] + "_org" : GAG_NAMES[hash];

        combo.push(name);
    }

    return combo;
}

})();
