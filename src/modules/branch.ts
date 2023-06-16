import inquirer from "inquirer";
import inquirerPrompt from "inquirer-autocomplete-prompt";
import fuzzy from "fuzzy";
import * as util from "node:util";
import * as child_process from "node:child_process";
const exec = util.promisify(child_process.exec);

import Color from "../utils/Color.js";
import Settings from "./settings.js";
import InterruptedInquirer from "../utils/InteruptedPrompt.js";
import Spinner from "../utils/Spinner.js";

inquirer.registerPrompt("autocomplete", inquirerPrompt);

new InterruptedInquirer(inquirer);

class Branch {
    private static branchesList: string[] = [];

    async showBranchMenu() {
        // console.clear();
        const spinner = new Spinner(
            Color.colorText("Loading branches...\n")
        ).start();

        const branches = await exec(`git branch -a`);

        spinner.success();
        await this.listBranches(branches.stdout.split("\n"));
    }

    private async listBranches(branches: string[]) {
        console.clear();
        Branch.branchesList = branches;
        console.log(Branch.branchesList);

        const selectedBranch: string | undefined = await this.handleBranchSelection();

        console.log(selectedBranch);

        await exec(`git checkout ${selectedBranch}`);

        // if (repoName) {
        //     let url: string | undefined = "";
        //     if (Settings.settings.protocol === "HTTPS") {
        //         url = data.find((repo) => repo.name === repoName)?.clone_url;
        //     } else {
        //         url = data.find((repo) => repo.name === repoName)?.ssh_url;
        //     }
        //     if (url) {
        //         await this.cloneRepo(url);
        //     }
        // }
    }

    private async handleBranchSelection() {
        try {
            const answers = await inquirer.prompt([
                {
                    type: "autocomplete",
                    name: "branch",
                    message: Color.colorText("Select branch\n"),
                    source: this.searchBranches,
                    pageSize: 8,
                },
            ]);
            const branch = answers.branch;
            return branch;
        } catch (error) {
            return;
        }
    }

    private searchBranches(answers: any, input: string) {
        input = input || "";
        return new Promise(function (resolve) {
            setTimeout(() => {
                const results = fuzzy
                    .filter(input, Branch.branchesList)
                    .map((el) => el.original);
                resolve(results);
            }, 100);
        });
    }

    private async cloneRepo(url: string) {
        const spinner = new Spinner(Color.colorText("Cloning repo...\n")).start();
        await exec(`git clone ${url}`);
        spinner.success();
    }
}

export default new Branch();
