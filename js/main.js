(function() {
"use strict";

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
    WebAssembly.instantiateStreaming(fetch("wasm/gag_combo_gen_web.wasm"))
        .then(main, () => {
            fetch("wasm/gag_combo_gen_web.wasm")
                .then(r => r.arrayBuffer())
                .then(r => WebAssembly.instantiate(r))
                .then(main, e => {
                    document.getElementById("gag-imgs-wrapper").innerHTML =
                        `<div id="error-div"><h1>${e}</h1><p>Woops.</p></div>`;
                    throw e;
                });
        });
};

function main(r) {
    const gag_combo_gen = r.instance.exports;

    const cog_level_elem = document.getElementById("cog-level");
    const lured_elem = document.getElementById("lured");
    const v2_elem = document.getElementById("v2");
    const toons_elem = document.getElementById("toons");
    const org_count_elem = document.getElementById("org-count");

    const trap_elem = document.getElementById("trap-select");
    const sound_elem = document.getElementById("sound-select");
    const throw_elem = document.getElementById("throw-select");
    const squirt_elem = document.getElementById("squirt-select");
    const drop_elem = document.getElementById("drop-select");

    const elems = [
        cog_level_elem, lured_elem, v2_elem,    toons_elem,  org_count_elem,
        trap_elem,      sound_elem, throw_elem, squirt_elem, drop_elem,
    ];

    const gag_img0 = document.getElementById("gag-img-0");
    const gag_img1 = document.getElementById("gag-img-1");
    const gag_img2 = document.getElementById("gag-img-2");
    const gag_img3 = document.getElementById("gag-img-3");
    const gag_imgs = [gag_img0, gag_img1, gag_img2, gag_img3];

    function update() {
        const cog_level = +cog_level_elem.value;
        if (cog_level > 12 || cog_level < 1) {
            cog_level_elem.value = 1;
            return;
        }
        const lured = lured_elem.checked;
        const v2 = v2_elem.checked;
        const toons = +toons_elem.value;
        if (toons > 4 || toons < 1) {
            toons_elem.value = 1;
            return;
        }
        const org_count = +org_count_elem.value;
        if (org_count > 4 || org_count < 0) {
            org_count_elem.value = 0;
            return;
        }

        const use_trap = trap_elem.checked;
        const use_sound = sound_elem.checked;
        const use_throw = throw_elem.checked;
        const use_squirt = squirt_elem.checked;
        const use_drop = drop_elem.checked;
        const gag_types =
            [use_trap, use_sound, use_throw, use_squirt, use_drop];
        const gag_types_mask = gag_types.reduce((m, b, i) => m | b << i, 0);

        const combo = translate_combo(
            toons,
            gag_combo_gen.gen(
                cog_level,
                lured,
                v2,
                toons,
                org_count,
                gag_types_mask
            )
        );

        let i;
        for (i = 0; i < combo.length; ++i) {
            const [gag_img, combo_str] = [gag_imgs[i], combo[i]];
            gag_img.src = `img/${combo_str}.png`;

            const rgn = readable_gag_name(combo_str);
            gag_img.alt = rgn;
            gag_img.title = rgn;

            if (combo_str.substr(-4) === "_org") {
                gag_img.classList.add("org");
            } else {
                gag_img.classList.remove("org");
            }

            gag_img.classList.remove("hidden");
        }
        for (; i < 4; ++i) {
            const gag_img = gag_imgs[i];

            gag_img.classList.add("hidden");
            gag_img.src = "";
            gag_img.alt = "\u00a0";
            gag_img.title = "";
        }
    }

    for (const elem of elems) {
        elem.onchange = update;
    }

    update();
}

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

function title_case(s) {
    return s.replace(
        /\w\S*/g,
        txt => txt.charAt(0).toUpperCase() + txt.substr(1)
    );
}

function readable_gag_name(s) {
    if (s.substr(-4) === "_org") {
        return "Organic " +
            title_case(s.substr(0, s.length - 4).replace(/_/g, " "));
    }
    return title_case(s.replace(/_/g, " "));
}

})();
