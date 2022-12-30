#!/usr/bin/env node

import chalk from "chalk";
import inquirer from "inquirer";
import chalkAnimation from "chalk-animation";
import figlet from "figlet";
import os, { type } from "os";

import * as fs from "fs";
import { createSpinner } from "nanospinner";

import * as util from "node:util";
import * as child_process from "node:child_process";
const exec = util.promisify(child_process.exec);

let loadedConfig = false;

let githubUsername;
let sorting = "Name";
let protocol = "HTTPS";

async function checkSavedSettings() {
  let homedir = os.homedir();
  try {
    let confFile = fs.readFileSync(homedir + "/.helper-config/settings.json", {
      encoding: "utf8",
      flag: "r",
    });
    let config = JSON.parse(confFile);
    if (config.username && config.protocol && config.sorting) {
      loadedConfig = true;
      githubUsername = config.username;
      sorting = config.sorting;
      protocol = config.protocol;
    } else {
      throw err;
    }
  } catch (error) {
    console.log(chalk.redBright("\nNo config file found\n"));
  }
}

async function saveCurrentSettings(update) {
  let data = JSON.stringify({
    username: githubUsername,
    protocol: protocol,
    sorting: sorting,
  });
  console.log(data);
  let homedir = os.homedir();
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

async function renderMenu() {
  console.log("Welcome to git-helper! \n");
  const answers = await inquirer.prompt({
    name: "menu_action",
    type: "list",
    message: "What can I do for you?",
    choices: ["Clone repo", "Edit settings", "Exit"],
  });

  return handleMenuChoice(answers.menu_action);
}

async function handleMenuChoice(choice) {
  if (choice === "Clone repo") {
    return handleCloneRepo();
  } else if (choice === "Edit settings") {
    return renderSettingsMenu();
  } else if (choice === "Exit") {
    process.exit(0);
  }
}

async function renderSettingsMenu() {
  const answers = await inquirer.prompt({
    name: "settings_action",
    type: "list",
    message: "Settings",
    choices: [
      `Username (${githubUsername})`,
      `Sorting (${sorting})`,
      `Protocol (${protocol})`,
      "Back",
    ],
  });
  let answer = answers.settings_action;
  if (answer === `Username (${githubUsername})`) {
    githubUsername = await askUsername();
  } else if (answer === `Sorting (${sorting})`) {
    sorting = await askSorting();
  } else if (answer === `Protocol (${protocol})`) {
    protocol = await askProtocol();
  } else {
    return renderMenu();
  }
  await saveCurrentSettings();
  return renderSettingsMenu();
}

async function handleCloneRepo() {
  const spinner = createSpinner("Loading your repos...").start();
  fetch(`https://api.github.com/users/${githubUsername}/repos`)
    .then((response) => response.json())
    .then((data) => {
      spinner.success();
      return listRepos(data);
    });
}

async function listRepos(data) {
  let repo_list = [];
  if (sorting === "Last updated") {
    let sortedList = [];
    for (let i = 0; i < data.length; i++) {
      sortedList.push({
        name: data[i].name,
        lastUpdated: new Date(data[i].pushed_at).valueOf(),
      });
    }
    sortedList.sort((a, b) => b.lastUpdated - a.lastUpdated);
    for (let i = 0; i < sortedList.length; i++) {
      if (sortedList[i].name !== githubUsername) {
        repo_list.push(sortedList[i].name);
      }
    }
  } else {
    for (let i = 0; i < data.length; i++) {
      if (data[i].name !== githubUsername) {
        repo_list.push(data[i].name);
      }
    }
  }

  let choice = await handleRepoChoice(repo_list);
  for (let i = 0; i < data.length; i++) {
    if (data[i].name === choice) {
      if (protocol === "HTTPS") {
        cloneRepo(data[i].clone_url);
      } else if (protocol === "SSH") {
        cloneRepo(data[i].ssh_url);
      }
    }
  }
}

async function handleRepoChoice(repo_list) {
  const answers = await inquirer.prompt({
    name: "github_repo",
    type: "list",
    message:"Select repo to clone",
    choices: repo_list,
  });
  return answers.github_repo;
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

async function cloneRepo(url) {
  const spinner = createSpinner("Cloning your repo...").start();
  await exec(`git clone ${url}`);
  spinner.success();
}

await checkSavedSettings();

if (!loadedConfig) {
  githubUsername = await askUsername();
  sorting = await askSorting();
  protocol = await askProtocol();
  await saveCurrentSettings();
}

await renderMenu();
