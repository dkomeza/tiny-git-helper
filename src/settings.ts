import inquirer from "inquirer";
import chalk from "chalk";
import os from "os";
import * as fs from "fs";

interface settings {
  username: string;
  sorting: string;
  protocol: string;
}

async function loadSavedSettings(settings: settings) {
  const homedir = os.homedir();
  try {
    const confFile = fs.readFileSync(
      homedir + "/.helper-config/settings.json",
      {
        encoding: "utf8",
        flag: "r",
      }
    );
    const config: settings = JSON.parse(confFile);
    if (config.username && config.protocol && config.sorting) {
      settings.username = config.username;
      settings.sorting = config.sorting;
      settings.protocol = config.protocol;
      return;
    } else {
      throw new Error("Invalid config file");
    }
  } catch (error) {
    console.log(chalk.redBright("\nNo config file found\n"));
    return getInitialSettings(settings);
  }
}

async function getInitialSettings(settings: settings) {
  settings.username = await askUsername();
  settings.sorting = await askSorting();
  settings.protocol = await askProtocol();
  return saveCurrentSettings(settings);
}

async function showSettingsMenu(settings: settings): Promise<void> {
  const answers = await inquirer.prompt({
    name: "settings_action",
    type: "list",
    message: "Settings",
    choices: [
      `Username (${settings.username})`,
      `Sorting (${settings.sorting})`,
      `Protocol (${settings.protocol})`,
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
  } else {
    return;
  }
  await saveCurrentSettings(settings);
  return showSettingsMenu(settings);
}

async function saveCurrentSettings(settings: settings) {
  const data = JSON.stringify({
    username: settings.username,
    protocol: settings.protocol,
    sorting: settings.sorting,
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

export default showSettingsMenu;
export { loadSavedSettings };
