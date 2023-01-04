import inquirer from "inquirer";
import chalk from "chalk";
import os from "os";
import * as fs from "fs";
import color, { settingsColor } from "./color.js";

interface settingsInterface {
  username: string;
  key: string;
  sorting: string;
  protocol: string;
  color: string;
}

async function loadSavedSettings(settings: settingsInterface) {
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
    if (
      config.username &&
      config.key &&
      config.protocol &&
      config.sorting &&
      config.color
    ) {
      settings.username = config.username;
      settings.key = config.key;
      settings.sorting = config.sorting;
      settings.protocol = config.protocol;
      settings.color = config.color;
      return;
    } else {
      throw new Error("Invalid config file");
    }
  } catch (error) {
    console.log(chalk.redBright("\nNo config file found\n"));
    return getInitialSettings(settings);
  }
}

async function getInitialSettings(settings: settingsInterface) {
  settings.username = await askUsername();
  settings.sorting = await askSorting();
  settings.protocol = await askProtocol();
  settings.color = await askColor(settings);
  return saveCurrentSettings(settings);
}

async function showSettingsMenu(settings: settingsInterface): Promise<void> {
  console.clear();
  const answers = await inquirer.prompt({
    name: "settings_action",
    type: "list",
    message: color("Settings \n", settings.color),
    choices: [
      `Username (${settings.username})`,
      `Sorting (${settings.sorting})`,
      `Protocol (${settings.protocol})`,
      `Color (${settings.color})`,
      "Back",
    ],
  });
  let answer = answers.settings_action;
  if (answer === `Username (${settings.username})`) {
    settings.username = await askUsername();
  } else if (answer === `Sorting (${settings.sorting})`) {
    settings.sorting = await askSorting();
  } else if (answer === `Protocol (${settings.protocol})`) {
    settings.protocol = await askProtocol();
  } else if (answer === `Color (${settings.color})`) {
    settings.color = await askColor(settings);
  } else {
    return;
  }
  await saveCurrentSettings(settings);
  return showSettingsMenu(settings);
}

async function saveCurrentSettings(settings: settingsInterface) {
  const data = JSON.stringify({
    username: settings.username,
    protocol: settings.protocol,
    sorting: settings.sorting,
    color: settings.color,
  });
  const homedir = os.homedir();
  try {
    if (!fs.existsSync(homedir + "/.helper-config")) {
      fs.mkdirSync(homedir + "/.helper-config");
    }
    fs.writeFileSync(homedir + "/.helper-config/settings.json", data);
  } catch (error) {
    console.log(error);
    console.log(chalk.redBright("\nCritical error\n"));
    process.exit(1);
  }
}

async function askUsername() {
  const answer = await inquirer.prompt({
    name: "github_username",
    type: "input",
    message: "What is you Github username?",
  });
  if (answer.github_username) {
    return answer.github_username;
  } else {
    await askUsername();
  }
}

async function askSorting() {
  const answers = await inquirer.prompt({
    name: "sorting",
    type: "list",
    message: "Repo sorting method.",
    choices: ["Name", "Last updated"],
  });
  return answers.sorting;
}

async function askProtocol() {
  const answers = await inquirer.prompt({
    name: "protocol",
    type: "list",
    message: "Which protocol to use?",
    choices: ["HTTPS", "SSH"],
  });
  return answers.protocol;
}

async function askColor(settings: settingsInterface) {
  const answers = await inquirer.prompt({
    name: "color",
    type: "list",
    message: color("Which color to use?", settings.color),
    choices: [
      settingsColor("Default", "Default"),
      settingsColor("Red", "Red"),
      settingsColor("Green", "Green"),
      settingsColor("Yellow", "Yellow"),
      settingsColor("Blue", "Blue"),
      settingsColor("Magenta", "Magenta"),
      settingsColor("Cyan", "Cyan"),
      settingsColor("White", "White"),
      settingsColor("Gray", "Gray"),
    ],
  });
  console.log(answers.color);
  return answers.color;
}

export default showSettingsMenu;
export { loadSavedSettings, settingsInterface };
