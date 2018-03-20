const path    = require("path");
const express = require("express");


const app = express();

app.set("port", process.env.PORT || 3000);

app.get("/", (req, res) => {
    res.sendFile(path.join(__dirname, "..", "index.html"));
});

app.get("/wasm/gag_combo_gen_web.wasm", (req, res) => {
    res.set("Content-Type", "application/wasm");
    res.sendFile(path.join(__dirname, "../wasm", "gag_combo_gen_web.wasm"));
});

app.use(express.static(path.join(__dirname, "..")));

app.listen(app.get("port"), () => {
    console.log("Node running at localhost:" + app.get("port"));
});
