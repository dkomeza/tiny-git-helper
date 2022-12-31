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

async function commitAllFiles(settings: settings) {
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

export { commitAllFiles };
