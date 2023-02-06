import inquirer from "inquirer";
import observe from "inquirer/lib/utils/events.js";

class InterruptedInquirer {
  constructor(Inquirer: typeof inquirer) {
    for (const key of Object.keys(Inquirer.prompt.prompts)) {
      Inquirer.prompt.prompts[key] = this.addInterruptedPrompt(
        Inquirer.prompt.prompts[key]
      );
    }
  }

  private addInterruptedPrompt(prompt: inquirer.prompts.PromptConstructor) {
    class InterruptedPrompt extends prompt {
      run(): Promise<any> {
        return new Promise((resolve, reject) => {
          // @ts-ignore
          const events = observe(this.rl);
          events.keypress.pipe().forEach((e) => {
            if (e.key.name === "escape") reject();
          });
          super.run().then(resolve, reject);
        });
      }
    }
    return InterruptedPrompt;
  }
}

export default InterruptedInquirer;
