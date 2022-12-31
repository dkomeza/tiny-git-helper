import chalk from "chalk";
import inquirer from "inquirer";
import * as util from "node:util";
import * as child_process from "node:child_process";
import color from "./color.js";
import { createSpinner } from "nanospinner";
const exec = util.promisify(child_process.exec);

interface settings {
  username: string;
  sorting: string;
  protocol: string;
  color: string;
}

let showMenu: Function | undefined;

async function showCommitMenu(settings: settings, callback?: Function) {
  showMenu = callback;
  console.clear();
  const answers = await inquirer.prompt({
    name: "commit_action",
    type: "list",
    message: color("What do you want to commit?", settings.color),
    choices: ["Commit specific files", "Commit all files", "Back"],
  });
  return handleCommitChoice(answers.commit_action, settings);
}

async function handleCommitChoice(choice: string, settings: settings) {
  switch (choice) {
    case "Commit all files":
      return commitAllFiles(settings);
    case "Commit specific files":
      return selectFiles(settings);
    default:
      if (showMenu) return showMenu();
      else return process.exit(0);
  }
}

async function commitAllFiles(settings: settings) {
  console.clear();
  if (process.argv.slice(2)[1]) {
    const spinner = createSpinner(
      color("Commiting...", settings.color)
    ).start();
    try {
      const { stdout, stderr } = await exec(
        `git add . && git commit -m "${process.argv.slice(2)[1]}" && git push`
      );
      spinner.success();
      let output = stdout.split(" ");
      let endIndex = output.indexOf("changed,");
      let outputString = output
        .slice(endIndex - 2, endIndex)
        .join(" ")
        .replace(/,/g, "");
      console.log(chalk.green(`Done! Successfully commited ${outputString}.`));
    } catch (error: any) {
      spinner.error();
      const output = error.stdout.replace(/\n/g, " ").split(" ");
      console.log(
        chalk.red(`Error: ${output.splice(3).slice(0, 6).join(" ")}.`)
      );
    }
  } else {
    const answers = await inquirer.prompt({
      name: "commit_message",
      type: "input",
      message: color("Enter commit message:", settings.color),
    });
    const spinner = createSpinner(
      color("Commiting...", settings.color)
    ).start();
    try {
      const { stdout, stderr } = await exec(
        `git add . && git commit -m "${answers.commit_message}" && git push`
      );
      spinner.success();
      let output = stdout.split(" ");
      let endIndex = output.indexOf("changed,");
      let outputString = output
        .slice(endIndex - 2, endIndex)
        .join(" ")
        .replace(/,/g, "");
      console.log(chalk.green(`Done! Successfully commited ${outputString}.`));
    } catch (error: any) {
      spinner.error();
      const output = error.stdout.replace(/\n/g, " ").split(" ");
      console.log(
        chalk.red(`Error: ${output.splice(3).slice(0, 6).join(" ")}.`)
      );
    }
  }
}

async function selectFiles(settings: settings) {
  const { stdout, stderr } = await exec("git status --short");
  const files = stdout.split("\n").filter((file) => file.length > 0);
  const choices = files.map((file) => {
    return {
      name: getFileColor(file),
      value: file,
    };
  });
  const answers = await inquirer.prompt({
    name: "files",
    type: "checkbox",
    message: color("Select files to commit:", settings.color),
    choices: choices,
  });
  return commitFiles(settings, answers.files);
}

function getFileColor(file: string) {
  let fileComparison = file.replace(" ", "");
  if (fileComparison.startsWith("M")) return chalk.yellow(file);
  else if (fileComparison.startsWith("A")) return chalk.green(file);
  else if (fileComparison.startsWith("D")) return chalk.red(file);
  else return file;
}

async function commitFiles(settings: settings, files: string[]) {
  console.clear();
  if (files.length === 0)
    return console.log(chalk.red("Error: No files selected."));
  const commitName = await askCommitName(settings);
  const spinner = createSpinner(color("Commiting...", settings.color)).start();
  for (let i = 0; i < files.length; i++) {
    files[i] = files[i].slice(3);
  }
  for (let i = 0; i < files.length; i++) {
    await exec(`git add ${files[i]}`);
  }
  try {
    const { stdout, stderr } = await exec(
      `git commit -m "${commitName}" && git push`
    );
    spinner.success();
    let output = stdout.split(" ");
    let endIndex = output.indexOf("changed,");
    let outputString = output
      .slice(endIndex - 2, endIndex)
      .join(" ")
      .replace(/,/g, "");
    console.log(chalk.green(`Done! Successfully commited ${outputString}.`));
  } catch (error: any) {
    spinner.error();
    const output = error.stdout.replace(/\n/g, " ").split(" ");
    console.log(chalk.red(`Error: ${output.splice(3).slice(0, 6).join(" ")}.`));
  }
}

async function askCommitName(settings: settings): Promise<string> {
  const answers = await inquirer.prompt({
    name: "commit_message",
    type: "input",
    message: color("Enter commit message:", settings.color),
  });
  if (answers.commit_message.length === 0) {
    console.log(chalk.red("Error: Commit message cannot be empty."));
    return askCommitName(settings);
  } else {
    return answers.commit_message;
  }
}

export default showCommitMenu;
export { commitAllFiles, selectFiles };
