import inquirer from "inquirer";

class CommitMessage {
    label: string;
    heading: string;
    description: string;
    constructor() {
        this.label = "";
        this.heading = "";
        this.description = "";
    }

    async getCommitMessage(long: boolean) {
        try {
            if (long) {
                await this.getLabel();
                await this.getHeading();
                await this.getDescription();
            } else {
                await this.getHeading();
                return this.heading;
            }
        } catch (error) {
            return
        }
    }

    async getLabel() {}

    async getHeading() {}

    async getDescription() {}
}

// Path: src/utils/CommitMessage.ts

export default new CommitMessage();