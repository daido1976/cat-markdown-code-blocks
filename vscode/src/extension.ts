// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import * as vscode from "vscode";

// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {
  // Use the console to output diagnostic information (console.log) and errors (console.error)
  // This line of code will only be executed once when your extension is activated
  console.log('Congratulations, your extension "cat-markdown" is now active!');

  // The command has been defined in the package.json file
  // Now provide the implementation of the command with registerCommand
  // The commandId parameter must match the command field in package.json
  let disposable = vscode.commands.registerCommand(
    "cat-markdown.helloWorld",
    () => {
      // The code you place here will be executed every time your command is executed
      // Display a message box to the user
      vscode.window.showInformationMessage("Hello World from cat-markdown!");
      import("../../wasm/pkg/wasm.js").then((wasm) => {
        const data = [
          // eslint-disable-next-line @typescript-eslint/naming-convention
          { file_name: "file1.txt", content: "Hello, world" },
          // eslint-disable-next-line @typescript-eslint/naming-convention
          { file_name: "file2.txt", content: "Goodbye, world" },
        ];
        vscode.window.showInformationMessage(
          `format: ${wasm.format_wasm(data)}`
        );
      });
    }
  );

  context.subscriptions.push(disposable);
}

// This method is called when your extension is deactivated
export function deactivate() {}
