import inquirer from "inquirer";
import * as fs from "fs";
import * as util from "node:util";
import * as child_process from "node:child_process";
const exec = util.promisify(child_process.exec);

import Color from "../utils/Color.js";
import Settings from "./settings.js";
import Spinner from "../utils/Spinner.js";

class Init {
  async showInitMenu() {
    const { name, description, privateRepo } = await inquirer.prompt([
      {
        name: "name",
        type: "input",
        message: Color.colorText("Enter repository name:"),
      },
      {
        name: "description",
        type: "input",
        message: Color.colorText("Enter repository description:"),
      },
      {
        name: "privateRepo",
        type: "confirm",
        message: Color.colorText("Is this repository private?"),
      },
    ]);
    const spinner = new Spinner(Color.colorText("Initializing...")).start();
    try {
      let folderName = name.replace(/ /g, "-");
      folderName = this.getFolderName(folderName);
      fs.mkdirSync(folderName);
      process.chdir(folderName);
      fs.writeFileSync("README.md", `# ${name}\n${description}`);
      const res = await fetch(`https://api.github.com/user/repos`, {
        method: "POST",
        headers: {
          Authorization: `token ${Settings.settings.key}`,
        },
        body: JSON.stringify({
          name: "super",
          description: "super",
          private: true,
        }),
      });
      const data = await res.json();
      let remote = "";
      if (Settings.settings.protocol === "SSH") {
        remote = data.ssh_url;
      } else {
        remote = data.clone_url;
      }
      const { stdout, stderr } = await exec(
        `git init && git add . && git commit -m "Initial commit" && git remote add origin ${remote} && git push -u origin master`
      );
      spinner.success();
      console.log(Color.colorText("Repository initialized successfully!"));
    } catch (error) {
      spinner.fail();
      console.log(error);
    }
  }

  private getFolderName(folderName: string) {
    let folder = folderName;
    let i = 1;
    while (fs.existsSync(folder)) {
      folder = `${folderName}(${i})`;
      console.log(folder);
      i++;
    }
    return folder;
  }
}

export default new Init();
