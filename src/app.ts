#!/usr/bin/env node

import showCloneMenu from "./clone.js";
import showCommitMenu, { commitAllFiles, selectFiles } from "./commit.js";
import initRepo from "./init.js";
import showSettingsMenu, {
  loadSavedSettings,
  settingsInterface,
} from "./settings.js";
import showHelp from "./help.js";
import color from "./color.js";

import chalk from "chalk";
import inquirer from "inquirer";

const settings: settingsInterface = {
  username: "",
  key: "",
  sorting: "",
  protocol: "",
  color: "",
};

await loadSavedSettings(settings);
await parseArgs();

async function showMenu(): Promise<void> {
  console.clear();
  console.log(color("Welcome to git-helper! \n", settings.color));
  const answers = await inquirer.prompt({
    name: "menu_action",
    type: "list",
    message: color("What can I do for you?", settings.color),
    choices: ["Commit", "Clone", "Settings", "Help", "Exit"],
  });

  return handleMenuChoice(answers.menu_action);
}

async function handleMenuChoice(choice: string) {
  switch (choice) {
    case "Commit":
      return showCommitMenu(settings, showMenu);
    case "Clone":
      return showCloneMenu(settings, showMenu);
    case "Settings":
      await showSettingsMenu(settings);
      return showMenu();
    case "Help":
      return showHelp(settings, showMenu);
    default:
      return process.exit(0);
  }
}

async function parseArgs() {
  if (process.argv.slice(2).length === 0) {
    return showMenu();
  } else {
    switch (process.argv.slice(2)[0]) {
      case "clone":
        return showCloneMenu(settings);
      case "commit":
        return showCommitMenu(settings);
      case "c":
        return showCommitMenu(settings);
      case "commitall":
        return commitAllFiles(settings);
      case "ca":
        return commitAllFiles(settings);
      case "commitfiles":
        return selectFiles(settings);
      case "cf":
        return selectFiles(settings);
      case "init":
        return initRepo(settings);
      case "settings":
        return showSettingsMenu(settings);
      case "help":
        return showHelp(settings);
    }
  }
}
