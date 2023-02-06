#!/usr/bin/env node

import inquirer from "inquirer";

import Color from "./modules/color.js";
import Settings from "./modules/settings.js";

await Settings.loadSettings();
Color.setColor(Settings.settings.color);

await parseArgs();

async function showMenu(): Promise<void> {
  console.clear();
  const answers = await inquirer.prompt({
    name: "menu_action",
    type: "list",
    message: Color.colorText("What can I do for you?\n"),
    choices: ["Commit", "Clone", "Settings", "Help", "Exit"],
  });

  return handleMenuChoice(answers.menu_action);
}

async function handleMenuChoice(choice: string) {
  switch (choice) {
    // case "Commit":
    //   return showCommitMenu(settings, showMenu);
    // case "Clone":
    //   return showCloneMenu(settings, showMenu);
    case "Settings":
      await Settings.showSettings();
      return showMenu();
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
    switch (
      process.argv.slice(2)[0]
      //   case "clone":
      //     return showCloneMenu(settings);
      //   case "commit":
      //     return showCommitMenu(settings);
      //   case "c":
      //     return showCommitMenu(settings);
      //   case "commitall":
      //     return commitAllFiles(settings);
      //   case "ca":
      //     return commitAllFiles(settings);
      //   case "commitfiles":
      //     return selectFiles(settings);
      //   case "cf":
      //     return selectFiles(settings);
      //   case "init":
      //     return initRepo(settings);
      //   case "settings":
      //     return showSettingsMenu(settings);
      //   case "help":
      //     return showHelp(settings);
    ) {
    }
  }
}
