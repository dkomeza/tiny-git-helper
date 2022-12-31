import inquirer from "inquirer";
import inquirerPrompt from "inquirer-autocomplete-prompt";
import fuzzy from "fuzzy";
import readline from "readline";
import color from "./color.js";

interface settings {
  username: string;
  sorting: string;
  protocol: string;
  color: string;
}

inquirer.registerPrompt("autocomplete", inquirerPrompt);

const optionList = [
  "clone      Clone a repo",
  "settings   Edit settings",
  "help       Show help",
  "Exit",
];

function parseArgs() {
  return process.argv.slice(3)[0];
}

function searchOptions(answers: any, input: string) {
  input = input || "";
  return new Promise(function (resolve) {
    setTimeout(() => {
      const results = fuzzy.filter(input, optionList).map((el) => el.original);
      resolve(results);
    }, 100);
  });
}

async function showHelp(settings: settings, callback?: any) {
  console.clear();
  if (process.argv.slice(3).length === 0) {
    console.log(color(`Usage: helper [options]`, settings.color));
    const answers = await inquirer.prompt([
      {
        name: "help_action",
        type: "autocomplete",
        message: color("What can I help you with? \n", settings.color),
        source: searchOptions,
        pageSize: 8,
      },
    ]);
    return showHelpCommand(answers.help_action, settings, callback);
  } else {
    return showHelpCommand(parseArgs(), settings);
  }
}

async function showHelpCommand(
  command: string,
  settings: settings,
  callback?: any
): Promise<void> {
  console.clear();
  switch (command.split(" ")[0]) {
    case "clone":
      console.log(color("Usage: helper clone", settings.color));
      console.log("Clone a repo");
      await waitForKeyPress();
      if (callback) return showHelp(settings, callback);
      else return process.exit(0);
    case "settings":
      console.log(color("Usage: helper settings", settings.color));
      console.log("Edit settings");
      await waitForKeyPress();
      if (callback) return showHelp(settings, callback);
      else return process.exit(0);
    case "Exit":
      if (callback) return callback();
      else process.exit(0);
  }
}

async function waitForKeyPress() {
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  return new Promise((resolve) =>
    rl.question("\npress enter to exit", (ans) => {
      rl.close();
      resolve(ans);
    })
  );
}

export default showHelp;
