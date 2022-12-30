import inquirer from "inquirer";
import inquirerPrompt from "inquirer-autocomplete-prompt";
import fuzzy from "fuzzy";
import readline from "readline";

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

async function showHelp(callback?: any) {
  callback = callback || "";
  console.clear();
  if (process.argv.slice(3).length === 0) {
    console.log(`Usage: helper [options]`);
    const answers = await inquirer.prompt([
      {
        name: "help_action",
        type: "autocomplete",
        message: "What can I help you with?",
        source: searchOptions,
        pageSize: 8,
      },
    ]);
    return showHelpCommand(answers.help_action, callback);
  } else {
    return showHelpCommand(parseArgs(), callback);
  }
}

async function showHelpCommand(command: string, callback?: any): Promise<void> {
  callback = callback || "";
  switch (command.split(" ")[0]) {
    case "clone":
      console.log("Usage: helper clone");
      console.log("Clone a repo");
      await waitForKeyPress();
      return showHelp(callback);
    case "settings":
      console.log("Usage: helper settings");
      console.log("Edit settings");
      await waitForKeyPress();
      return showHelp(callback);
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
