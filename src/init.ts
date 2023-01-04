import { settingsInterface } from "./settings.js";

import { Octokit } from "octokit";

import * as util from "node:util";
import * as child_process from "node:child_process";
import * as fs from "fs";
const exec = util.promisify(child_process.exec);

async function initRepo(settings: settingsInterface) {
  const name = process.argv.splice(3);
  if (name.length === 0) {
    console.log("Please provide a repository name");
    return;
  }
  const description = name.join(" ");
  const repoName = name.join(" ").replace(/ /g, "-").toLowerCase();
  const repoUrl =
    settings.protocol === "SSH"
      ? `git@github.com:${settings.username}/${repoName}.git`
      : `https://github.com/${settings.username}/${repoName}.git`;
  const webUrl = `https://github.com/${settings.username}/${repoName}`;
  const repoExists = await checkIfRepoExists(repoName, settings);

  if (repoExists) {
    console.log("Repository already exists");
    return;
  }
  if (fs.existsSync(repoName)) {
    console.log("Directory already exists");
    return;
  }
  fs.mkdirSync(repoName);
  process.chdir(repoName);
  const octokit = new Octokit({
    auth: settings.key,
  });
  const repo = await octokit.request("POST /user/repos", {
    name: repoName,
    description: description,
    homepage: repoUrl,
    private: false,
  });
  await exec("git init");
  fs.writeFileSync("README.md", `# ${description}`);
  await exec("git add .");
  console.log("Add");
  await exec(`git commit -m "Initial commit"`);
  console.log("Commit");
  const { stderr, stdout } = await exec(`git remote add origin ${repoUrl}`);
  console.log(stderr);
  console.log(stdout);
  console.log("Remote");
  await exec("git push -u origin master");
  console.log(`Repository created at ${webUrl}`);
}

async function checkIfRepoExists(
  repoName: string,
  settings: settingsInterface
) {
  const octokit = new Octokit({
    auth: settings.key,
  });
  const repos = await octokit.request("GET /user/repos");
  const repo = repos.data.find((repo) => repo.name === repoName);
  if (repo) {
    return true;
  } else {
    return false;
  }
}

export default initRepo;
