import chalk from "chalk";

function color(text: string, selectedColor: string) {
  switch (selectedColor) {
    case getColorName("Red"):
      return chalk.red(text);
    case getColorName("Green"):
      return chalk.green(text);
    case getColorName("Yellow"):
      return chalk.yellow(text);
    case getColorName("Blue"):
      return chalk.blue(text);
    case getColorName("Magenta"):
      return chalk.magenta(text);
    case getColorName("Cyan"):
      return chalk.cyan(text);
    case getColorName("White"):
      return chalk.white(text);
    case getColorName("Gray"):
      return chalk.gray(text);
    default:
      return text;
  }
}

function settingsColor(text: string, selectedColor: string) {
  switch (selectedColor) {
    case "Red":
      return chalk.red(text);
    case "Green":
      return chalk.green(text);
    case "Yellow":
      return chalk.yellow(text);
    case "Blue":
      return chalk.blue(text);
    case "Magenta":
      return chalk.magenta(text);
    case "Cyan":
      return chalk.cyan(text);
    case "White":
      return chalk.white(text);
    case "Gray":
      return chalk.gray(text);
    default:
      return text;
  }
}

function getColorName(color: string) {
  switch (color.toLowerCase()) {
    case "red":
      return chalk.red("Red");
    case "green":
      return chalk.green("Green");
    case "yellow":
      return chalk.yellow("Yellow");
    case "blue":
      return chalk.blue("Blue");
    case "magenta":
      return chalk.magenta("Magenta");
    case "cyan":
      return chalk.cyan("Cyan");
    case "white":
      return chalk.white("White");
    case "gray":
      return chalk.gray("Gray");
    default:
      return "Default";
  }
}

export default color;
export { settingsColor };
