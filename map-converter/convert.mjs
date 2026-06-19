import fs from "node:fs";
import { spawnSync } from "node:child_process";
import otbm from "./otbm2json.js";

let input = process.argv[2];
if (!input) {
    input = "map.otbm";
    if (!fs.existsSync(input)) {
        console.log("Usage: node convert.mjs map.otbm");
        process.exit(1);
    }
}

const data = otbm.read(input);
const tiles = data.data.nodes[0].features[0].tiles;

function formatTile(tile) {
    const parts = [
        `"x": ${tile.x}`,
        `"y": ${tile.y}`
    ];
    
    if (tile.tileid != null) {
        parts.push(`"tileid": ${tile.tileid}`);
    }
    
    if (Array.isArray(tile.items) && tile.items.length) {
        const items = tile.items.map(item => `{ "id": ${item.id} }`).join(", ");
        parts.push(`"items": [${items}]`);
    }
    
    return `{ ${parts.join(", ")} }`;
}

const json = `{
\t"tiles": [
\t\t${tiles.map(formatTile).join(",\n\t\t")}
\t]
}`;

const result = spawnSync(
    "grep",
    ["-v", '"type"'],
    {
        input: json,
        encoding: "utf8"
    }
);

if (result.status === 0) {
    fs.writeFileSync("map.json", result.stdout, "utf8");
    console.log("Converted OTBM to JSON.");
}