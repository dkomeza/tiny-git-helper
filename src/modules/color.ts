import chalk from "chalk";
class Color {
  private color = "Default";

  setColor(color: string) {
    this.color = color;
  }

  colorText(text: string, color?: string) {
    const selectedColor = color ? color : this.color;
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
}

export default new Color();
