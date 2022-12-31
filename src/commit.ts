import inquirer from "inquirer";
import * as util from "node:util";
import * as child_process from "node:child_process";
import color from "./color.js";
const exec = util.promisify(child_process.exec);

interface settings {
  username: string;
  sorting: string;
  protocol: string;
  color: string;
}

async function commitAllFiles(settings: settings) {
  if (process.argv.slice(2)[1]) {
    try {
      const { stdout, stderr } = await exec(
        `git add . && git commit -m "${process.argv.slice(2)[1]}" && git push`
      );
    } catch (error: any) {
      console.log(error.stderr);
    }
  } else {
    const answers = await inquirer.prompt({
      name: "commit_message",
      type: "input",
      message: color("Enter commit message:", settings.color),
    });
    try {
      await exec(
        `git add . && git commit -m "${answers.commit_message}" && git push`
      );
    } catch (error: any) {
      console.log(error.stderr);
    }
  }
}

export { commitAllFiles };
