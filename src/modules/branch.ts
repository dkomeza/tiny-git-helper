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
import BranchName from "../utils/BranchName.js";

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
                        name: "Delete branch",
                        value: "delete",
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
            case "delete":
                return this.deleteBranch();
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
        console.clear();

        const branchName = await BranchName.getBranchName();

        if (!branchName) {
            return;
        }

        const changes = await exec(`git status --short`)
        let loadChanges = false;

        if (changes.stdout.split("\n").filter((line: string) => line.length > 0).length > 0) {
            const answers = await inquirer.prompt({
                name: "stashChanges",
                type: "confirm",
                message: "You have uncommited changes. Do you want to stash them?",
            });

            if (answers.stashChanges) {
                loadChanges = true;
            }
        }

        const spinner = new Spinner(
            Color.colorText("Creating branch...\n")
        ).start();

        try {
            const currentBranch = await exec(`git branch --show-current`);
            await exec(`git stash save -u ${currentBranch.stdout}`);

            await exec(`git switch -c ${branchName}`);

            await exec(`git config --add --bool push.autoSetupRemote true`);

            if (loadChanges) {
                await exec(`git stash apply`);
            }

            spinner.success();
            console.log(
                Color.colorText(`Done! Successfully created branch: ${branchName}.`, "green")
            );
            return;
        } catch (error) {
            spinner.fail();
            console.log(Color.colorText("Something went wrong!\n", "red"));
            console.log(error);
            return;
        }
    }

    async deleteBranch() {
        console.clear();

        const spinner = new Spinner(
            Color.colorText("Loading branches...\n")
        ).start();
        const branches = await exec(`git branch -a`);
        Branch.branchesList = branches.stdout.split("\n");
        spinner.success();

        const result: string | undefined = await this.handleBranchSelection();

        if (!result) {
            return
        };

        const selectedBranch = result?.slice(2);

        const answers = await inquirer.prompt({
            name: "deleteBranch",
            type: "confirm",
            message: `You sure you want to delete '${selectedBranch}' branch?`,
        });

        if (answers.deleteBranch) {
            const spinner = new Spinner(
                Color.colorText("Deleting branch...\n")
            ).start();
            try {
                if (selectedBranch.startsWith("remotes")) {
                    const remote = selectedBranch.split("/")[1];
                    const start = [selectedBranch.split("/")[0], selectedBranch.split("/")[1]].join("/") + "/"
                    const branch = selectedBranch.slice(start.length);

                    await exec(`git push ${remote} --delete ${branch}`);
                } else {
                    await exec(`git branch -d ${selectedBranch}`);
                }

                spinner.success();
                console.log(
                    Color.colorText(`Done! Successfully deleted branch: ${selectedBranch}.`, "green")
                );
            } catch (error) {
                spinner.fail();
                console.log(Color.colorText("Something went wrong!\n", "red"));
                console.log(error);
                return;
            }
        }
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
