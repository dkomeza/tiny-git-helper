import inquirer from "inquirer";
import inquirerPrompt from "inquirer-autocomplete-prompt";
import fuzzy from "fuzzy";
import { createSpinner } from "nanospinner";
import * as util from "node:util";
import * as child_process from "node:child_process";
import color from "./color.js";
import { settingsInterface } from "./settings.js";
const exec = util.promisify(child_process.exec);

inquirer.registerPrompt("autocomplete", inquirerPrompt);

let repoList: string[] = [];
let showMenu: Function | undefined;

interface data {
  name: string;
  pushed_at: string;
  clone_url: string;
  ssh_url: string;
}

interface Repo {
  name: string;
  lastUpdated: number;
}

function seatchRepos(answers: any, input: string) {
  input = input || "";
  return new Promise(function (resolve) {
    setTimeout(() => {
      const results = fuzzy.filter(input, repoList).map((el) => el.original);
      resolve(results);
    }, 100);
  });
}

async function showCloneMenu(settings: settingsInterface, callback?: Function) {
  showMenu = callback;
  console.clear();
  const spinner = createSpinner(
    color("Loading your repos...\n", settings.color)
  ).start();
  fetch(`https://api.github.com/users/${settings.username}/repos`)
    .then((response) => response.json())
    .then((data) => {
      spinner.success();
      return listRepos(data, settings);
    });
}

async function listRepos(
  data: data[],
  settings: settingsInterface
): Promise<void> {
  console.clear();
  let repo_list = [];
  if (settings.sorting === "Last updated") {
    let sortedList: Repo[] = [];
    for (let i = 0; i < data.length; i++) {
      sortedList.push({
        name: data[i].name,
        lastUpdated: new Date(data[i].pushed_at).valueOf(),
      });
    }
    sortedList.sort((a, b) => b.lastUpdated - a.lastUpdated);
    for (let i = 0; i < sortedList.length; i++) {
      if (sortedList[i].name !== settings.username) {
        repo_list.push(sortedList[i].name);
      }
    }
  } else {
    for (let i = 0; i < data.length; i++) {
      if (data[i].name !== settings.username) {
        repo_list.push(data[i].name);
      }
    }
  }
  repoList = repo_list;
  repoList.push(" ");
  repoList.push("Exit");
  repoList.push(" ");
  let choice = await handleRepoChoice(settings.color);
  for (let i = 0; i < data.length; i++) {
    if (data[i].name === choice) {
      if (settings.protocol === "HTTPS") {
        await cloneRepo(data[i].clone_url);
      } else if (settings.protocol === "SSH") {
        await cloneRepo(data[i].ssh_url);
      }
    } else if (choice === "Exit") {
      if (showMenu) return showMenu();
      else process.exit(0);
    } else {
      console.clear();
      console.log(color("Please select a repo\n", settings.color));
      return listRepos(data, settings);
    }
  }
}

async function handleRepoChoice(userColor: string) {
  const answers = await inquirer.prompt([
    {
      type: "autocomplete",
      name: "github_repo",
      message: color("Github repo \n", userColor),
      source: seatchRepos,
      pageSize: 8,
    },
  ]);
  return answers.github_repo;
}

async function cloneRepo(url: string) {
  const spinner = createSpinner("Cloning your repo...").start();
  await exec(`git clone ${url}`);
  spinner.success();
}

export default showCloneMenu;
