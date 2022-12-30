import inquirer from "inquirer";
import * as util from "node:util";
import * as child_process from "node:child_process";
const exec = util.promisify(child_process.exec);

async function commitAllFiles() {
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
      message: "Enter commit message",
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
