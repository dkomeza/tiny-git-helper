#!/usr/bin/env node

import inquirer from "inquirer";

import Settings from "./modules/settings.js";
import Clone from "./modules/clone.js";
import Commit from "./modules/commit.js";
import Init from "./modules/init.js";
import Branch from "./modules/branch.js";

import Color from "./utils/Color.js";

await Settings.loadSettings();
Color.setColor(Settings.settings.color);

await parseArgs();

/**
 * Asks the user what they want to do
 * @returns Function that handles the user's choice
 */
async function showMenu(): Promise<void> {
  try {
    const answers = await inquirer.prompt({
      name: "menu_action",
      type: "list",
      message: Color.colorText("What can I do for you?\n"),
      choices: ["Commit", "Branch", "Clone", "Init", "Settings", "Help", "Exit"],
    });

    return handleMenuChoice(answers.menu_action);
  } catch (error: unknown) {
    console.log(); // create empty line (visual improvement)
  }
}

/**
 *  Handle the user menu choice
 * @param {string} choice User menu choice
 * @returns Either returns to the menu or closes the program
 */
async function handleMenuChoice(choice: string) {
  switch (choice) {
    case "Commit":
      await Commit.showCommitMenu();
      return showMenu();
    case "Branch":
      await Branch.showBranchMenu();
      return showMenu();
    case "Clone":
      await Clone.showCloneMenu();
      return showMenu();
    case "Settings":
      await Settings.showSettings();
      return showMenu();
    case "Init":
      await Init.showInitMenu();
      return showMenu();
    // case "Help":
    //   return showHelp(settings, showMenu);
    default:
      return process.exit(0);
  }
}

/**
 * Function to parse cli arguments
 * @returns If there are no arguments shows main menu, otherwise shows appropriate submenu
 */
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
      default:
        return showMenu();
    }
  }
}
