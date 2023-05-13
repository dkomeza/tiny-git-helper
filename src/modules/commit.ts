import inquirer from "inquirer";
import * as util from "node:util";
import * as child_process from "node:child_process";
const exec = util.promisify(child_process.exec);

import settings from "./settings.js";

import Color from "../utils/Color.js";
import Spinner from "../utils/Spinner.js";
import CommitMessage from "../utils/CommitMessage.js";

class Commit {
  async showCommitMenu() {
    console.clear();
    try {
      const { commit_action } = await inquirer.prompt({
        name: "commit_action",
        type: "list",
        message: Color.colorText("What do you want to commit?\n"),
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
      return this.handleCommitChoice(commit_action);
    } catch (error: any) {
      return;
    }
  }

  private async handleCommitChoice(choice: string) {
    switch (choice) {
      case "all":
        return this.commitAllFiles();
      case "specific":
        return this.selectFiles();
      default:
        return;
    }
  }

  async commitAllFiles() {
    console.clear();
    let title: string;
    let description = "";
    if (process.argv.slice(3).length !== 0) {
      title = process.argv.slice(3).join(" ");
    } else {
      const commitMessage = await CommitMessage.getCommitMessage(
        settings.settings.fancyCommit ? true : false
      );
      title = commitMessage.title;
      description = commitMessage.description;
    }
    if (title.length === 0) return;
    const spinner = new Spinner(Color.colorText("Commiting...")).start();
    try {
      const commitCommand = description
        ? `git commit -m "${title}" -m "${description}"`
        : `git commit -m "${title}"`;
      const { stdout } = await exec(
        `git add . && ${commitCommand} && git push`
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
      console.log(error);
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
    try {
      const { selected_files } = await inquirer.prompt({
        name: "selected_files",
        type: "checkbox",
        message: Color.colorText("Select files to commit:"),
        choices: choices,
      });
      return this.commitFiles(selected_files);
    } catch (error: any) {
      return;
    }
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
    const { title, description } = await CommitMessage.getCommitMessage(
      settings.settings.fancyCommit ? true : false
    );
    if (title.length === 0) return;
    const spinner = new Spinner(Color.colorText("Commiting...")).start();
    for (let i = 0; i < files.length; i++) {
      files[i] = files[i].slice(3);
    }
    for (let i = 0; i < files.length; i++) {
      await exec(`git add ${files[i]}`);
    }
    try {
      const commitCommand = description
        ? `git commit -m "${title}" -m "${description}"`
        : `git commit -m "${title}"`;
      await exec(`${commitCommand} && git push`);
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
    try {
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
    } catch (error: any) {
      return "";
    }
  }
}

export default new Commit();
