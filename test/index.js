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

const express = require("express");
const path = require("path");

const app = express();

app.set("port", process.env.PORT || 3000);

app.get("/", (_req, res) => {
    res.sendFile(path.join(__dirname, "..", "index.html"));
});

app.get("/wasm/gag_combo_gen_web.wasm", (_req, res) => {
    res.set("Content-Type", "application/wasm");
    res.sendFile(path.join(__dirname, "../wasm", "gag_combo_gen_web.wasm"));
});

app.use(express.static(path.join(__dirname, "..")));

app.listen(app.get("port"), () => {
    console.log(`Node running at localhost:${app.get("port")}`);
});
