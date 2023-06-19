import * as fs from "fs";

import { fileURLToPath } from "url";

import Color from "../utils/Color.js";

class Version {
  async showVersion() {
    const jsonPath = fileURLToPath(import.meta.url).replace(
      "build/modules/version.js",
      "package.json"
    );

    const json = JSON.parse(fs.readFileSync(jsonPath, "utf-8"));

    console.log(
      Color.colorText(`\nCurrent version: ${json.version}\n`, "yellow")
    );
  }
}

export default new Version();
