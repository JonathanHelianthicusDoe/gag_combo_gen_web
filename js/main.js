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

const NO_LURE = 0x00;
const LURING  = 0x01;
const LURED   = 0x02;

const BLUE_MAGNET      = 0x00;
const HYPNO            = 0x01;
const ORG_HYPNO        = 0x02;
const PRESENTATION     = 0x03;
const ORG_PRESENTATION = 0x04;

window.onload = function() {
    WebAssembly.instantiateStreaming(fetch("wasm/gag_combo_gen_web.wasm"))
        .then(main, () => {
            fetch("wasm/gag_combo_gen_web.wasm")
                .then(r => r.arrayBuffer())
                .then(r => WebAssembly.instantiate(r))
                .then(main, e => {
                    document.getElementById("combo-display-wrapper-0")
                        .innerHTML = `<div id="error-div">
                                      <p><code>${e}</code></p>
                                      <p>Woops.</p></div>`;

                    throw e;
                });
        });
};

function main(r) {
    const gag_combo_gen = r.instance.exports;

    const cog_level_elem = document.getElementById("cog-level");
    const no_lure_elem = document.getElementById("no-lure");
    const luring_elem = document.getElementById("luring");
    const lured_elem = document.getElementById("lured");
    const lure_gag_elem = document.getElementById("lure-gag");
    const v2_elem = document.getElementById("v2");
    const toons_elem = document.getElementById("toons");
    const org_count_elem = document.getElementById("org-count");
    const k_elem = document.getElementById("k");

    const trap_elem = document.getElementById("trap-select");
    const sound_elem = document.getElementById("sound-select");
    const throw_elem = document.getElementById("throw-select");
    const squirt_elem = document.getElementById("squirt-select");
    const drop_elem = document.getElementById("drop-select");

    const combo_display_wrappers =
        document.getElementsByClassName("combo-display-wrapper");

    function update() {
        const cog_level = +cog_level_elem.value;
        if (cog_level > 12 || cog_level < 1) {
            cog_level_elem.value = 1;

            return;
        }
        const luring = (() => {
            if (no_lure_elem.checked) {
                return NO_LURE;
            } else if (luring_elem.checked) {
                return LURING;
            } else if (lured_elem.checked) {
                return LURED;
            } else {
                no_lure_elem.checked = true;

                return NO_LURE;
            }
        })();
        const lure_gag = (() => {
            switch (lure_gag_elem.value) {
            case "blue-magnet":      return BLUE_MAGNET;
            case "hypno":            return HYPNO;
            case "org-hypno":        return ORG_HYPNO;
            case "presentation":     return PRESENTATION;
            case "org-presentation": return ORG_PRESENTATION;
            default:
                lure_gag_elem.value = "blue-magnet";

                return BLUE_MAGNET;
            }
        })();
        const lure = luring | (lure_gag << 8);
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
            lure,
            v2,
            toons,
            org_count,
            gag_types_mask,
        );

        for (let i = 0; i < combo_display_wrappers.length; ++i) {
            const combo_display_wrapper = combo_display_wrappers[i];
            if (i >= combos.length) {
                combo_display_wrapper.classList.add("hidden");

                continue;
            }
            combo_display_wrapper.classList.remove("hidden");

            const [combo, accuracy] = combos[i];
            const gag_imgs =
                combo_display_wrapper.getElementsByClassName("gag-img");

            let j = 0;
            for (; j < combo.length; ++j) {
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
                gag_img.alt = "";
                gag_img.title = "";
            }

            const accuracy_span =
                combo_display_wrapper.getElementsByClassName("accuracy")[0];
            accuracy_span.innerText = `${(accuracy * 100).toFixed(1)}%`;
            accuracy_span.style.color = accuracy_color(accuracy);
        }
    }

    function gen_combos(
        k,
        cog_level,
        lure,
        v2,
        toons,
        org_count,
        gag_types_mask,
    ) {
        gag_combo_gen.gen(
            k,
            cog_level,
            lure,
            v2,
            toons,
            org_count,
            gag_types_mask,
        );

        const stored_combos = [];
        for (let i = 0; i < k; ++i) {
            const hash = gag_combo_gen.get_combo(i);
            if (hash === 0) {
                break;
            }
            const accuracy = gag_combo_gen.get_accuracy(i);
            stored_combos.push([hash, accuracy]);
        }
        if (stored_combos.length === 0) {
            stored_combos.push([0, 1]);
        }

        const non_lure_toons =
            (lure & 0x000000ff) === LURING ? toons - 1 : toons;

        return stored_combos.map(
            ([h, a]) => [translate_combo(non_lure_toons, h), a]);
    }

    const elems = [
        cog_level_elem, no_lure_elem, luring_elem, lured_elem,
        lure_gag_elem,  v2_elem,      toons_elem,  org_count_elem, k_elem,

        trap_elem,      sound_elem,   throw_elem,  squirt_elem,    drop_elem,
    ];
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

function accuracy_color(accuracy) {
    if (accuracy > 0.99) {
        return "#6cf6ee";
    }
    if (accuracy <= 0.5) {
        return "#f65353";
    }

    let [r, g, b] = [0x6c, 0xf6, 0xee];
    const diff = 1.0 - accuracy;

    const b_decr =          Math.min(diff,        0.05);
    const r_incr = Math.max(Math.min(diff - 0.05, 0.2 ), 0);
    const g_decr = Math.max(Math.min(diff - 0.25, 0.2 ), 0);

    // Spooky magic numbers!
    b -= b_decr * 2600;
    r += r_incr *  700;
    g -= g_decr *  700;

    return "#"
        + Math.round(r).toString(16).padStart(2, "0")
        + Math.round(g).toString(16).padStart(2, "0")
        + Math.round(b).toString(16).padStart(2, "0");
}
