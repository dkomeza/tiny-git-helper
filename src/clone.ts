import inquirer from "inquirer";
import { createSpinner } from "nanospinner";
import * as util from "node:util";
import * as child_process from "node:child_process";
const exec = util.promisify(child_process.exec);

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

async function showCloneMenu(settings: {
  username: string;
  sorting: string;
  protocol: string;
}) {
  console.clear();
  const spinner = createSpinner("Loading your repos...\n").start();
  fetch(`https://api.github.com/users/${settings.username}/repos`)
    .then((response) => response.json())
    .then((data) => {
      spinner.success();
      return listRepos(
        data,
        settings.username,
        settings.sorting,
        settings.protocol
      );
    });
}

async function listRepos(
  data: data[],
  githubUsername: string,
  sorting: string,
  protocol: string
) {
  console.clear();
  let repo_list = [];
  if (sorting === "Last updated") {
    let sortedList: Repo[] = [];
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

async function handleRepoChoice(repo_list: string[]) {
  const answers = await inquirer.prompt({
    name: "github_repo",
    type: "list",
    message: "Select repo to clone",
    choices: repo_list,
  });
  return answers.github_repo;
}

async function cloneRepo(url: string) {
  const spinner = createSpinner("Cloning your repo...").start();
  await exec(`git clone ${url}`);
  spinner.success();
}

export default showCloneMenu;
