# The Ray Tracer Challenge: A Test-Driven Guide to Your First 3D Renderer

## Gherking / Cucumber

### IDE

#### Visual Studio Code

You can get full support by simply installing [this extension](https://marketplace.visualstudio.com/items?itemName=CucumberOpen.cucumber-official)

#### Zed

1. Close Zed
2. Install the Cucumber Language Server *globally* (with Node version > 18)
    ```sh
    npm install -g @cucumber/language-server
    ```
3. Clone [this fork](https://github.com/alistairstead/zed-extension-cucumber)
4. Reopen Zed then `Extensions` -> `Install Dev Extension` -> Select the root path of `zed-extension-cucumber`
5. Cucumber should be fully working now, giving you Gherkin syntax highliting, `Go to definition`, `Define in ...` and other nice features

⚠️ If you're using a custom NPM prefix to change the default global folder location, Zed won't be able to figure it out and will default to downloading its own broken version of the cucumber language server. In that case check the `.zed/settings.json` file, uncomment the related settings and set your own desired path.
