import os from "os";
import * as fs from "fs";

import Color from "../utils/Color.js";
import inquirer from "inquirer";

interface settingsInterface {
  username: string;
  key: string;
  sorting: string;
  protocol: string;
  color: string;
  fancyCommit: boolean | string;
}

class Settings {
  settings: settingsInterface = {
    username: "",
    key: "",
    sorting: "",
    protocol: "",
    color: "",
    fancyCommit: "",
  };

  async loadSettings() {
    const homedir = os.homedir();
    try {
      const confFile = fs.readFileSync(
        homedir + "/.helper-config/settings.json",
        {
          encoding: "utf8",
          flag: "r",
        }
      );
      const config: settingsInterface = JSON.parse(confFile);
      for (const key in config) {
        if (config.hasOwnProperty(key)) {
          const keyName = key as keyof settingsInterface;
          (this.settings[keyName] as string | boolean) = config[keyName];
        }
      }
      this.fillSettings();
    } catch (error) {
      console.log(Color.colorText("\nNo config file found\n", "Red"));
      return this.askSettings();
    }
  }

  async showSettings(): Promise<void> {
    console.clear();
    try {
      const answers = await inquirer.prompt({
        name: "settings",
        type: "list",
        message: Color.colorText("Settings\n"),
        choices: [
          {
            name: `Change username (${this.settings.username})`,
            value: "username",
          },
          {
            name: "Change token",
            value: "token",
          },
          {
            name: `Change sorting (${this.settings.sorting})`,
            value: "sorting",
          },
          {
            name: `Change protocol (${this.settings.protocol})`,
            value: "protocol",
          },
          {
            name: `Change color (${Color.colorText(
              this.settings.color,
              this.settings.color
            )})`,
            value: "color",
          },
          {
            name: `Change fancy commit (${this.settings.fancyCommit})`,
            value: "fancyCommit",
          },
          {
            name: "Back",
            value: "back",
          },
        ],
      });
      switch (answers.settings) {
        case "username":
          this.settings.username = await this.askUsername();
          break;
        case "token":
          this.settings.key = await this.askToken();
          break;
        case "sorting":
          this.settings.sorting = await this.askSorting();
          break;
        case "protocol":
          this.settings.protocol = await this.askProtocol();
          break;
        case "color":
          this.settings.color = await this.askColor();
          break;
        case "fancyCommit":
          this.settings.fancyCommit = await this.askFancyCommit();
          break;
        case "back":
          return;
      }
      await this.saveSettings();
      return this.showSettings();
    } catch (error) {
      return;
    }
  }

  private async askSettings() {
    this.settings.username = await this.askUsername();
    this.settings.key = await this.askToken();
    this.settings.sorting = await this.askSorting();
    this.settings.protocol = await this.askProtocol();
    this.settings.color = await this.askColor();
    return this.saveSettings();
  }

  private async fillSettings() {
    for (const key in this.settings) {
      const keyName = key as keyof settingsInterface;
      if (this.settings[keyName] === "") {
        switch (keyName) {
          case "username":
            this.settings.username = await this.askUsername();
            break;
          case "key":
            this.settings.key = await this.askToken();
            break;
          case "sorting":
            this.settings.sorting = await this.askSorting();
            break;
          case "protocol":
            this.settings.protocol = await this.askProtocol();
            break;
          case "color":
            this.settings.color = await this.askColor();
            break;
          case "fancyCommit":
            this.settings.fancyCommit = await this.askFancyCommit();
            break;
        }
      }
    }
    for (const key in this.settings) {
      const keyName = key as keyof settingsInterface;
      if (this.settings[keyName] === "") {
        throw new Error("Invalid config file");
      }
    }
    return this.saveSettings();
  }

  private async askUsername() {
    const answer = await inquirer.prompt({
      name: "github_username",
      type: "input",
      message: "What is you Github username?",
    });
    if (answer.github_username) {
      return answer.github_username;
    } else {
      await this.askUsername();
    }
  }

  private async askToken() {
    const answer = await inquirer.prompt({
      name: "github_token",
      type: "input",
      message: "What is you Github token?",
    });
    if (answer.github_token) {
      return answer.github_token;
    } else {
      await this.askToken();
    }
  }

  private async askSorting() {
    const answers = await inquirer.prompt({
      name: "sorting",
      type: "list",
      message: "Repo sorting method.",
      choices: ["Name", "Last updated"],
    });
    return answers.sorting;
  }

  private async askProtocol() {
    const answers = await inquirer.prompt({
      name: "protocol",
      type: "list",
      message: "Which protocol to use?",
      choices: ["HTTPS", "SSH"],
    });
    return answers.protocol;
  }

  private async askColor() {
    const answers = await inquirer.prompt({
      name: "color",
      type: "list",
      message: Color.colorText("Which color to use?", this.settings.color),
      choices: [
        {
          name: Color.colorText("Red", "Red"),
          value: "Red",
        },
        {
          name: Color.colorText("Green", "Green"),
          value: "Green",
        },
        {
          name: Color.colorText("Yellow", "Yellow"),
          value: "Yellow",
        },
        {
          name: Color.colorText("Blue", "Blue"),
          value: "Blue",
        },
        {
          name: Color.colorText("Magenta", "Magenta"),
          value: "Magenta",
        },
        {
          name: Color.colorText("Cyan", "Cyan"),
          value: "Cyan",
        },
        {
          name: Color.colorText("White", "White"),
          value: "White",
        },
        {
          name: Color.colorText("Gray", "Gray"),
          value: "Gray",
        },
        {
          name: Color.colorText("Default", "Default"),
          value: "Default",
        },
      ],
    });
    Color.setColor(answers.color);
    return answers.color;
  }

  private async askFancyCommit() {
    const answers = await inquirer.prompt({
      name: "fancyCommit",
      type: "confirm",
      message: "Use fancy commit messages?",
    });
    return answers.fancyCommit;
  }

  private async saveSettings() {
    const data = JSON.stringify(this.settings);
    const homedir = os.homedir();
    try {
      if (!fs.existsSync(homedir + "/.helper-config")) {
        fs.mkdirSync(homedir + "/.helper-config");
      }
      fs.writeFileSync(homedir + "/.helper-config/settings.json", data);
      return;
    } catch (error) {
      console.log(error);
      console.log(Color.colorText("\nError saving settings\n", "Red"));
      process.exit(1);
    }
  }
}

export default new Settings();
