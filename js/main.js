/*
 * @licstart  The following is the entire license notice for the JavaScript
 * code in this page.
 *
 * This file is part of gag_combo_gen.
 *
 * gag_combo_gen is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Affero General Public License as published by the
 * Free Software Foundation, either version 3 of the License, or (at your
 * option) any later version.
 *
 * gag_combo_gen is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License
 * for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with gag_combo_gen.  If not, see <https://www.gnu.org/licenses/>.
 *
 * @licend  The above is the entire license notice for the JavaScript code in
 * this page.
 */

"use strict";

/* eslint-disable indent */
const GAG_NAMES = [
    "pass",
  /* trap           sound             throw              squirt              drop */
    "banana_peel", "bikehorn",       "cupcake",         "squirting_flower", "flowerpot",
    "rake",        "whistle",        "fruit_pie_slice", "glass_of_water",   "sandbag",
    "marbles",     "bugle",          "cream_pie_slice", "squirtgun",        "anvil",
    "quicksand",   "aoogah",         "fruit_pie",       "seltzer_bottle",   "big_weight",
    "trap_door",   "elephant_trunk", "cream_pie",       "fire_hose",        "safe",
    "tnt",         "foghorn",        "cake",            "storm_cloud",      "grand_piano",
    "railroad",    "opera_singer",   "wedding_cake",    "geyser",           "toontanic",
];
/* eslint-enable indent */

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
    const k_elem = document.getElementById("k");

    const trap_elem = document.getElementById("trap-select");
    const sound_elem = document.getElementById("sound-select");
    const throw_elem = document.getElementById("throw-select");
    const squirt_elem = document.getElementById("squirt-select");
    const drop_elem = document.getElementById("drop-select");

    const elems = [
        cog_level_elem, lured_elem, v2_elem,    toons_elem,  org_count_elem,
        k_elem,
        trap_elem,      sound_elem, throw_elem, squirt_elem, drop_elem,
    ];

    const gag_imgs_wrappers =
        document.getElementsByClassName("gag-imgs-wrapper");

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
        const k = +k_elem.value;
        if (k < 1 || k > 10) {
            k_elem.value = 1;

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

        const combos = gen_combos(
            k,
            cog_level,
            lured,
            v2,
            toons,
            org_count,
            gag_types_mask
        );

        for (let i = 0; i < gag_imgs_wrappers.length; ++i) {
            const gag_imgs_wrapper = gag_imgs_wrappers[i];
            if (i >= combos.length) {
                gag_imgs_wrapper.classList.add("hidden");

                continue;
            }
            gag_imgs_wrapper.classList.remove("hidden");

            const combo = combos[i];
            const gag_imgs =
                gag_imgs_wrapper.getElementsByClassName("gag-img");

            let j;
            for (j = 0; j < combo.length; ++j) {
                const [gag_img, combo_str] = [gag_imgs[j], combo[j]];
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
            for (; j < 4; ++j) {
                const gag_img = gag_imgs[j];

                gag_img.classList.add("hidden");
                gag_img.src = "";
                gag_img.alt = "\u00a0";
                gag_img.title = "";
            }
        }
    }

    function gen_combos(
        k,
        cog_level,
        lured,
        v2,
        toons,
        org_count,
        gag_types_mask,
    ) {
        gag_combo_gen.gen(
            k,
            cog_level,
            lured,
            v2,
            toons,
            org_count,
            gag_types_mask,
        );

        const combo_hashes = [];
        for (let i = 0; i < k; ++i) {
            const hash = gag_combo_gen.get(i);
            if (hash === 0) {
                break;
            }
            combo_hashes.push(hash);
        }
        if (combo_hashes.length === 0) {
            combo_hashes.push(0);
        }

        return combo_hashes.map(h => translate_combo(toons, h));
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
        txt => txt.charAt(0).toUpperCase() + txt.substr(1),
    );
}

function readable_gag_name(s) {
    if (s.substr(-4) === "_org") {
        return "Organic " +
            title_case(s.substr(0, s.length - 4).replace(/_/g, " "));
    }

    return title_case(s.replace(/_/g, " "));
}
