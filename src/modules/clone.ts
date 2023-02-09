import inquirer from "inquirer";
import inquirerPrompt from "inquirer-autocomplete-prompt";
import fuzzy from "fuzzy";
import * as util from "node:util";
import * as child_process from "node:child_process";
const exec = util.promisify(child_process.exec);

import Color from "../utils/Color.js";
import Settings from "./settings.js";
import InterruptedInquirer from "../utils/InteruptedPrompt.js";
import Spinner from "../utils/Spinner.js";

interface repo {
  name: string;
  pushed_at: string;
  clone_url: string;
  ssh_url: string;
}

inquirer.registerPrompt("autocomplete", inquirerPrompt);

new InterruptedInquirer(inquirer);

class Clone {
  private static repoList: string[] = [];
  async showCloneMenu() {
    console.clear();
    const spinner = new Spinner(
      Color.colorText("Loading your repos...\n")
    ).start();
    const res = await fetch("https://api.github.com/user/repos", {
      headers: {
        Authorization: `token ${Settings.settings.key}`,
      },
    });
    const data = await res.json();
    spinner.success();
    await this.listRepos(data);
  }

  private async listRepos(data: repo[]) {
    console.clear();
    Clone.repoList = [];
    if (Settings.settings.sorting === "Last updated") {
      const sortedList = [];
      for (const repo of data) {
        const lastUpdated = new Date(repo.pushed_at as string).valueOf();
        sortedList.push({
          name: repo.name,
          lastUpdated,
        });
        sortedList.sort((a, b) => b.lastUpdated - a.lastUpdated);
      }
      for (const repo of sortedList) {
        if (repo.name !== Settings.settings.username) {
          Clone.repoList.push(repo.name);
        }
      }
    } else {
      for (const repo of data) {
        if (repo.name !== Settings.settings.username) {
          Clone.repoList.push(repo.name);
        }
      }
    }
    const repoName: string | undefined = await this.handleRepoSelection();
    if (repoName) {
      let url: string | undefined = "";
      if (Settings.settings.protocol === "HTTPS") {
        url = data.find((repo) => repo.name === repoName)?.clone_url;
      } else {
        url = data.find((repo) => repo.name === repoName)?.ssh_url;
      }
      if (url) {
        await this.cloneRepo(url);
      }
    }
  }

  private async handleRepoSelection() {
    try {
      const answers = await inquirer.prompt([
        {
          type: "autocomplete",
          name: "repo",
          message: Color.colorText("Select a repo\n"),
          source: this.searchRepos,
          pageSize: 8,
        },
      ]);
      const repo = answers.repo;
      return repo;
    } catch (error) {
      return;
    }
  }

  private searchRepos(answers: any, input: string) {
    input = input || "";
    return new Promise(function (resolve) {
      setTimeout(() => {
        const results = fuzzy
          .filter(input, Clone.repoList)
          .map((el) => el.original);
        resolve(results);
      }, 100);
    });
  }

  private async cloneRepo(url: string) {
    const spinner = new Spinner(Color.colorText("Cloning repo...\n")).start();
    await exec(`git clone ${url}`);
    spinner.success();
  }
}

export default new Clone();
