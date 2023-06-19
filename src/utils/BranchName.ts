import fuzzy from "fuzzy";
import inquirer from "inquirer";
import inquirerPrompt from "inquirer-autocomplete-prompt";

import Color from "./Color.js";
import InterruptedInquirer from "./InteruptedPrompt.js";

inquirer.registerPrompt("autocomplete", inquirerPrompt);

new InterruptedInquirer(inquirer);

const types = [
    { name: "feat" },
    { name: "bugfix" },
    { name: "test" },
    { name: "refactor" },
    { name: "hotfix" },
    { name: "release" },
    { name: "docs" },
    { name: "support" },
    { name: "custom" }
];

class BranchName {
    async getBranchName(): Promise<string> {
        try {
            const branchType = await this.getBranchType();
            const name = await this.getName();

            return `${branchType}/${name}`;
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

        if (type === "custom") {
            const answer = await inquirer.prompt({
                name: "customBranchName",
                type: "input",
                message: "Enter custom branch type",
                validate: this.validateType,
            });

            return answer.customBranchName;
        } else {
            return type;
        }
    }

    private async getName(): Promise<string> {
        const { name }: { name: string } = await inquirer.prompt([
            {
                type: "input",
                name: "name",
                message: Color.colorText("Enter branch name"),
                validate: this.validateName,
            },
        ]);
        return name.trim().replace(/\ +/g, "-") || "";
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

    private validateName(input: string) {
        if (input == "") {
            return "Name cannot be empty";
        }
        return true;
    }

    private validateType(input: string) {
        if (input == "") {
            return "Type cannot be empty";
        }
        return true;
    }
}

export default new BranchName();

// Path: src/utils/CommitMessage.ts
