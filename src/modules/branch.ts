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
        console.clear();
        try {
            const { branch_action } = await inquirer.prompt({
                name: "branch_action",
                type: "list",
                message: Color.colorText("What action do you want to perform?\n"),
                choices: [
                    {
                        name: "Switch branch",
                        value: "switch",
                    },
                    {
                        name: "Create new branch",
                        value: "create",
                    },
                    {
                        name: "Back",
                        value: "back",
                    },
                ],
            });
            return this.handleBranchChoice(branch_action);
        } catch (error: any) {
            return;
        }
    }

    private async handleBranchChoice(choice: string) {
        switch (choice) {
            case "switch":
                return this.switchBranch();
            case "create":
                return this.createBranch();
            default:
                return;
        }
    }

    async switchBranch() {
        console.clear();
        const spinner = new Spinner(
            Color.colorText("Loading branches...\n")
        ).start();

        const branches = await exec(`git branch -a`);

        spinner.success();
        await this.listBranches(branches.stdout.split("\n"));
    }

    async createBranch() {
        console.log("create branch");
    }

    private async listBranches(branches: string[]) {
        console.clear();
        Branch.branchesList = branches;

        const selectedBranch: string | undefined = await this.handleBranchSelection();

        if (!selectedBranch) {
            return
        };

        let currentBranch = await exec(`git branch --show-current`);

        let stashes = await exec(`git stash list`);

        let stashToLoad: string | undefined = undefined
        for (let i = 0; i < stashes.stdout.split("\n").length; i++) {
            if (stashes.stdout.split("\n")[i].includes(selectedBranch.slice(2))) {
                stashToLoad = stashes.stdout.split("\n")[i].split(":")[0];
            }
        }

        await exec(`git stash save ${currentBranch.stdout} -u`);
        await exec(`git checkout ${selectedBranch.slice(2)}`);

        if (stashToLoad) {
            await exec(`git stash apply ${stashToLoad}`);
            await exec(`git stash drop ${stashToLoad}`);
        }
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
}

export default new Branch();
