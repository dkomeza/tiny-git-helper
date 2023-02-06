#!/usr/bin/env node

import inquirer from "inquirer";

import Color from "./modules/color.js";
import Settings from "./modules/settings.js";
import Clone from "./modules/clone.js";
import Commit from "./modules/commit.js";
import Init from "./modules/init.js";

await Settings.loadSettings();
Color.setColor(Settings.settings.color);

await parseArgs();

async function showMenu(): Promise<void> {
  console.clear();
  const answers = await inquirer.prompt({
    name: "menu_action",
    type: "list",
    message: Color.colorText("What can I do for you?\n"),
    choices: ["Commit", "Clone", "Init", "Settings", "Help", "Exit"],
  });

  return handleMenuChoice(answers.menu_action);
}

async function handleMenuChoice(choice: string) {
  switch (choice) {
    case "Commit":
      await Commit.showCommitMenu();
      return showMenu();
    case "Clone":
      await Clone.showCloneMenu();
      return showMenu();
    case "Settings":
      await Settings.showSettings();
      return showMenu();
    case "Init":
      await Init.showInitMenu();
    // return showMenu();
    // case "Help":
    //   return showHelp(settings, showMenu);
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
        return Clone.showCloneMenu();
      case "commit":
        return Commit.showCommitMenu();
      case "c":
        return Commit.showCommitMenu();
      case "commitall":
        return Commit.commitAllFiles();
      case "ca":
        return Commit.commitAllFiles();
      case "commitfiles":
        return Commit.selectFiles();
      case "cf":
        return Commit.selectFiles();
      case "init":
        return Init.showInitMenu();
      case "settings":
        return Settings.showSettings();
      //   case "help":
      //     return showHelp(settings);
    }
  }
}
