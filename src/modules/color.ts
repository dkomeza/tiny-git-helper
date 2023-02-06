import chalk from "chalk";

export default class Color {
  private static color = "Default";

  static setColor(color: string) {
    Color.color = color;
  }

  static getColor() {
    return Color.color;
  }

  static colorText(text: string, color?: string) {
    const selectedColor = color ? color : Color.color;
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
