import inquirer from "inquirer";
import observe from "inquirer/lib/utils/events.js";

class InterruptedInquirer {
  /**
   * Create a new interruptable inquirer prompt for each prompt type
   * @param Inquirer Inquirer instance
   */
  constructor(Inquirer: typeof inquirer) {
    for (const key of Object.keys(Inquirer.prompt.prompts)) {
      Inquirer.prompt.prompts[key] = this.addInterruptedPrompt(
        Inquirer.prompt.prompts[key]
      );
    }
  }

  /**
   * Add the interruptable prompt to the inquirer
   * @param prompt Prompt to add interrupt to
   * @returns The new prompt with interrupt
   */
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
