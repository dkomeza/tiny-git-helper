import inquirer from "inquirer";
import { createSpinner } from "nanospinner";
import * as fs from "fs";
import { Octokit } from "octokit";
import * as util from "node:util";
import * as child_process from "node:child_process";
const exec = util.promisify(child_process.exec);

import Color from "./color.js";
import Settings from "./settings.js";

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
    const spinner = createSpinner(Color.colorText("Initializing...")).start();
    try {
      let folderName = name.replace(/ /g, "-");
      folderName = this.getFolderName(folderName);
      fs.mkdirSync(folderName);
      process.chdir(folderName);
      fs.writeFileSync("README.md", `# ${name}\n${description}`);
      const octokit = new Octokit({
        auth: Settings.settings.key,
      });
      const { data } = await octokit.request("POST /user/repos", {
        name,
        description,
        private: privateRepo,
      });
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
      spinner.error();
      console.log(error);
    }
    console.log(name, description, privateRepo);
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
