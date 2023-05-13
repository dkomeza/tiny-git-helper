import fuzzy from "fuzzy";
import inquirer from "inquirer";
import inquirerPrompt from "inquirer-autocomplete-prompt";

import Color from "./Color.js";
import InterruptedInquirer from "./InteruptedPrompt.js";

inquirer.registerPrompt("autocomplete", inquirerPrompt);

new InterruptedInquirer(inquirer);

const labels = [
  { name: "Initial commit", emoji: ":tada:" },
  { name: "Version tag", emoji: ":bookmark:" },
  { name: "New feature", emoji: ":sparkles:" },
  { name: "Bug fix", emoji: ":bug:" },
  { name: "Metadata", emoji: ":card_index:" },
  { name: "Documentation", emoji: ":books:" },
  { name: "Documenting source code", emoji: ":bulb:" },
  { name: "Performance", emoji: ":racehorse:" },
  { name: "Cosmetic", emoji: ":lipstick:" },
  { name: "Tests", emoji: ":rotating_light:" },
  { name: "Adding a test", emoji: ":white_check_mark:" },
  { name: "Make a test pass", emoji: ":heavy_check_mark:" },
  { name: "General update", emoji: ":zap:" },
  { name: "Improve format/structure", emoji: ":art:" },
  { name: "Refactor code", emoji: ":hammer:" },
  { name: "Removing code/files", emoji: ":fire:" },
  { name: "Continuous Integration", emoji: ":green_heart:" },
  { name: "Security", emoji: ":lock:" },
  { name: "Upgrading dependencies", emoji: ":arrow_up:" },
  { name: "Downgrading dependencies", emoji: ":arrow_down:" },
  { name: "Lint", emoji: ":shirt:" },
  { name: "Translation", emoji: ":alien:" },
  { name: "Text", emoji: ":pencil:" },
  { name: "Critical hotfix", emoji: ":ambulance:" },
  { name: "Deploying stuff", emoji: ":rocket:" },
  { name: "Fixingon MacOS", emoji: ":apple:" },
  { name: "Fixingon Linux", emoji: ":penguin:" },
  { name: "Fixingon Windows", emoji: ":checkered_flag:" },
  { name: "Work in progress", emoji: ":construction:" },
  { name: "Adding CI build system", emoji: ":construction_worker:" },
  { name: "Analytics or tracking code", emoji: ":chart_with_upwards_trend:" },
  { name: "Removing a dependency", emoji: ":heavy_minus_sign:" },
  { name: "Adding a dependency", emoji: ":heavy_plus_sign:" },
  { name: "Docker", emoji: ":whale:" },
  { name: "Configuration files", emoji: ":wrench:" },
  { name: "Package.json in JS", emoji: ":package:" },
  { name: "Merging branches", emoji: ":twisted_rightwards_arrows:" },
  { name: "Bad code/need improv.", emoji: ":hankey:" },
  { name: "Reverting changes", emoji: ":rewind:" },
  { name: "Breaking changes", emoji: ":boom:" },
  { name: "Code review changes", emoji: ":ok_hand:" },
  { name: "Accessibility", emoji: ":wheelchair:" },
  { name: "Move/rename repository", emoji: ":truck:" },
];

class CommitMessage {
  async getCommitMessage(
    fancyCommit: boolean
  ): Promise<{ title: string; description: string }> {
    try {
      if (fancyCommit) {
        const { name, emoji } = await this.getLabel();
        const heading = await this.getHeading();
        const description = await this.getDescription();
        return { title: `${emoji} ${heading}`, description };
      } else {
        const heading = await this.getHeading();
        return { title: heading, description: "" };
      }
    } catch (error) {
      return { title: "", description: "" };
    }
  }

  private async getLabel(): Promise<{ name: string; emoji: string }> {
    const answer = await inquirer.prompt([
      {
        type: "autocomplete",
        name: "label",
        message: Color.colorText("Select label\n"),
        source: this.searchLabel,
        pageSize: 8,
      },
    ]);
    const { label } = answer;
    const { name, emoji } = labels.filter((value) => value.name === label)[0];
    return { name, emoji };
  }

  private async getHeading(): Promise<string> {
    const answer = await inquirer.prompt([
      {
        type: "input",
        name: "heading",
        message: Color.colorText("Enter commit heading"),
        validate: this.validateHeading,
      },
    ]);
    const { heading } = answer;
    return heading || "";
  }

  private async getDescription(): Promise<string> {
    const answer = await inquirer.prompt([
      {
        type: "input",
        name: "description",
        message: Color.colorText("Enter commit description"),
      },
    ]);
    const { description } = answer;
    return description || "";
  }

  private searchLabel(_answers: string, input: string) {
    input = input || "";
    return new Promise(function (resolve) {
      setTimeout(() => {
        const results = fuzzy
          .filter(
            input,
            labels.map((el) => el.name)
          )
          .map((el) => el.original);
        resolve(results);
      }, 100);
    });
  }

  private validateHeading(input: any) {
    if (input == "") {
      return "Heading cannot be empty";
    }
    return true;
  }
}

export default new CommitMessage();

// Path: src/utils/CommitMessage.ts
