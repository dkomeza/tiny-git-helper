import chalk from "chalk";
class Color {
  private color = "Default";

  /**
   * Set the default color to use
   * @param {string} color Color to set
   */
  setColor(color: string) {
    this.color = color;
  }

  /**
   * Colorize text based on the default color or the color passed in the parameter
   * @param text Text to color
   * @param color Optional color to use
   * @returns Colored text
   */
  colorText(text: string, color?: string) {
    const selectedColor = color ? color : this.color;
    switch (selectedColor.toLowerCase()) {
      case "red":
        return chalk.red(text);
      case "green":
        return chalk.green(text);
      case "yellow":
        return chalk.yellow(text);
      case "blue":
        return chalk.blue(text);
      case "magenta":
        return chalk.magenta(text);
      case "cyan":
        return chalk.cyan(text);
      case "white":
        return chalk.white(text);
      case "gray":
        return chalk.gray(text);
      default:
        return text;
    }
  }
}

export default new Color();
