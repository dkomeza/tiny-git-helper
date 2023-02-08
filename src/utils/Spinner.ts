import tty from "tty";
import Color from "./Color.js";

const isTTY =
  tty.isatty(1) && process.env.TERM !== "dumb" && !("CI" in process.env);
const isCI =
  process.env.CI ||
  process.env.WT_SESSION ||
  process.env.ConEmuTask === "{cmd::Cmder}" ||
  process.env.TERM_PROGRAM === "vscode" ||
  process.env.TERM === "xterm-256color" ||
  process.env.TERM === "alacritty";
const supportUnicode =
  process.platform !== "win32" ? process.env.TERM !== "linux" : isCI;

class Spinner {
  private message: string;
  private symbols = {
    frames: isTTY
      ? supportUnicode
        ? ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]
        : ["-", "\\", "|", "/"]
      : ["-"],
    tick: supportUnicode ? "✔" : "√",
    cross: supportUnicode ? "✖" : "×",
    warn: supportUnicode ? "⚠" : "!!",
  };
  private currentFrame = 0;
  private timer: NodeJS.Timeout | null = null;
  private lines = 0;
  constructor(message: string) {
    this.message = message;
  }

  start() {
    this.update().loop();
    return this;
  }

  success() {
    let symbol = this.symbols.tick;
    return this.stop(symbol, "green");
  }

  fail() {
    let symbol = this.symbols.cross;
    return this.stop(symbol, "red");
  }

  private update() {
    if (this.symbols.frames.length - 1 < this.currentFrame) {
      this.currentFrame = 0;
    }
    return this;
  }

  private loop() {
    isTTY && (this.timer = setTimeout(() => this.loop(), 50));
    return this.spin();
  }

  private spin() {
    this.update();
    this.render();
    this.currentFrame++;
    return this;
  }

  private render() {
    const mark = this.symbols.frames[this.currentFrame];
    let str = `${mark} ${this.message}`;
    isTTY ? this.write(`\x1b[?25l`) : (str += "\n");
    this.write(str, true);
    isTTY && (this.lines = this.getLines(str, process.stderr.columns));
  }

  private write(str: string, clear = false) {
    if (clear && isTTY) {
      this.clear();
    }
    process.stderr.write(Color.colorText(str));
    return this;
  }

  private getLines(str: string, columns: number) {
    return str
      .replace(/\u001b[^m]*?m/g, "")
      .split("\n")
      .reduce(
        (col, line) => (col += Math.max(1, Math.ceil(line.length / columns))),
        0
      );
  }

  private clear() {
    this.write("\x1b[1G");
    for (let i = 0; i < this.lines; i++) {
      i > 0 && this.write("\x1b[1A");
      this.write("\x1b[2K\x1b[1G");
    }
    this.lines = 0;

    return this;
  }

  private stop(symbol: string, color: string) {
    if (this.timer) {
      clearTimeout(this.timer);
    }

    this.write(`${Color.colorText(symbol, color)} ${this.message}\n`, true);

    return isTTY ? this.write(`\x1b[?25h`) : this;
  }
}

export default Spinner;
