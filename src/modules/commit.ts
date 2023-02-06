import inquirer from "inquirer";
import * as util from "node:util";
import * as child_process from "node:child_process";
const exec = util.promisify(child_process.exec);

import Color from "./color.js";
import Settings from "./settings.js";
import Spinner from "../utils/Spinner.js";

class Commit {
  async showCommitMenu() {
    console.clear();
    const answers = await inquirer.prompt({
      name: "commit_action",
      type: "list",
      message: Color.colorText("What do you want to commit?"),
      choices: [
        {
          name: "Commit specific files",
          value: "specific",
        },
        {
          name: "Commit all files",
          value: "all",
        },
        {
          name: "Back",
          value: "back",
        },
      ],
    });
    return this.handleCommitChoice(answers.commit_action);
  }

  private async handleCommitChoice(choice: string) {
    switch (choice) {
      case "all":
        return this.commitAllFiles();
      case "specific":
        return this.selectFiles();
      default:
        return Settings.showSettings();
    }
  }

  async commitAllFiles() {
    console.clear();
    let message: string;
    if (process.argv.slice(3).length !== 0) {
      message = process.argv.slice(3).join(" ");
    } else {
      message = await this.askCommitMessage();
    }
    const spinner = new Spinner(Color.colorText("Commiting...")).start();
    try {
      const { stdout, stderr } = await exec(
        `git add . && git commit -m "${message}" && git push`
      );
      spinner.success();
      let output = stdout.split(" ");
      let endIndex = output.indexOf("changed,");
      let outputString = output
        .slice(endIndex - 2, endIndex)
        .join(" ")
        .replace(/,/g, "");
      console.log(
        Color.colorText(`Done! Successfully commited ${outputString}.`, "green")
      );
    } catch (error: any) {
      spinner.fail();
      const output = error.stdout.replace(/\n/g, " ").split(" ");
      console.log(
        Color.colorText(
          `Error: ${output.splice(3).slice(0, 6).join(" ")}.`,
          "red"
        )
      );
    }
  }

  async selectFiles() {
    console.clear();
    const { stdout, stderr } = await exec("git status --short");
    const files = stdout.split("\n").filter((file) => file.length > 0);
    const choices = files.map((file) => {
      return {
        name: Color.colorText(file, this.getFileColor(file)),
        value: file,
      };
    });
    const answers = await inquirer.prompt({
      name: "files",
      type: "checkbox",
      message: Color.colorText("Select files to commit:"),
      choices: choices,
    });
    return this.commitFiles(answers.files);
  }

  private getFileColor(file: string) {
    let fileComparison = file.replace(" ", "");
    if (fileComparison.startsWith("M")) return "yellow";
    else if (fileComparison.startsWith("A")) return "green";
    else if (fileComparison.startsWith("D")) return "red";
    else if (fileComparison.startsWith("??")) return "white";
    else return "default";
  }

  private async commitFiles(files: string[]) {
    console.clear();
    if (files.length === 0) {
      return console.log(Color.colorText("Error: No files selected.", "red"));
    }
    const message = await this.askCommitMessage();
    const spinner = new Spinner(Color.colorText("Commiting...")).start();
    for (let i = 0; i < files.length; i++) {
      files[i] = files[i].slice(3);
    }
    for (let i = 0; i < files.length; i++) {
      await exec(`git add ${files[i]}`);
    }
    try {
      const { stdout, stderr } = await exec(
        `git commit -m "${message}" && git push`
      );
      spinner.success();
      console.log(
        Color.colorText(
          `Done! Successfully commited ${files.length} files.`,
          "green"
        )
      );
    } catch (error: any) {
      spinner.fail();
      const output = error.stdout.replace(/\n/g, " ").split(" ");
      console.log(
        Color.colorText(
          `Error: ${output.splice(3).slice(0, 6).join(" ")}.`,
          "red"
        )
      );
    }
  }

  private async askCommitMessage(): Promise<string> {
    const { message } = await inquirer.prompt({
      name: "message",
      type: "input",
      message: Color.colorText("Enter commit message:"),
    });
    if (message.length === 0) {
      console.log(
        Color.colorText("Error: Commit message cannot be empty.", "red")
      );
      return this.askCommitMessage();
    } else {
      return message;
    }
  }
}

export default new Commit();
