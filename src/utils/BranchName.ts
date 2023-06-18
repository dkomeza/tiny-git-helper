import fuzzy from "fuzzy";
import inquirer from "inquirer";
import inquirerPrompt from "inquirer-autocomplete-prompt";

import Color from "./Color.js";
import InterruptedInquirer from "./InteruptedPrompt.js";

inquirer.registerPrompt("autocomplete", inquirerPrompt);

new InterruptedInquirer(inquirer);

const types = [
    { name: "feature" },
    { name: "development" },
    { name: "bugfix" },
    { name: "hotfix" },
    { name: "release" },
    { name: "support" },
    { name: "custom" }
];

class BranchName {
    async getBranchName(): Promise<string> {
        try {
            const branchType = await this.getBranchType();
            const name = await this.getName();

            return `${branchType}-${name}`;
        } catch (error) {
            return "";
        }
    }

    private async getBranchType(): Promise<string> {
        const answer = await inquirer.prompt([
            {
                type: "autocomplete",
                name: "type",
                message: Color.colorText("Select branch type\n"),
                source: this.searchTypes,
                pageSize: 8,
            },
        ]);
        const { type } = answer;
        return type;
    }

    private async getName(): Promise<string> {
        const answer = await inquirer.prompt([
            {
                type: "input",
                name: "name",
                message: Color.colorText("Enter branch name"),
                validate: this.validateName,
            },
        ]);
        const { name } = answer;
        name.replace(" ", "-");
        return name || "";
    }

    private searchTypes(_answers: string, input: string) {
        input = input || "";
        return new Promise(function (resolve) {
            setTimeout(() => {
                const results = fuzzy
                    .filter(
                        input,
                        types.map((el) => el.name)
                    )
                    .map((el) => el.original);
                resolve(results);
            }, 100);
        });
    }

    private validateName(input: any) {
        if (input == "") {
            return "Name cannot be empty";
        }
        return true;
    }
}

export default new BranchName();

// Path: src/utils/CommitMessage.ts
