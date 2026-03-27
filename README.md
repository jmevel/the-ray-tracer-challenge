# The Ray Tracer Challenge: A Test-Driven Guide to Your First 3D Renderer

## Gherking / Cucumber

### IDE

#### Visual Studio Code

You can get full support by simply installing [this extension](https://marketplace.visualstudio.com/items?itemName=CucumberOpen.cucumber-official)

#### Zed

You can install [this extension](https://zed.dev/extensions/cucumber) but this is only a client. In order to get it to work you must also install the [Cucumber LSP](https://github.com/cucumber/language-server)

As of today, the Zed Cucumber extension requires the Cucumber LSP to be installed *globally* with Node 18 (or lower)

1. Install [the Zed cucumber extension](https://zed.dev/extensions/cucumber) then close Zed 
2. Install [nvm](https://github.com/nvm-sh/nvm)
3. Install the latest LTS of Node 18: 
    ```sh
    nvm install 18.20.8
    ```
4. Make sure Node 18 is now the default version
    ```sh
    node -v
    nvm ls
    ```
5. Install the Cucumber LSP globally
    ```sh
    npm install -g @cucumber/language-server
    ```
6. Reopen Zed and wait the LSP is loaded and started. The extension should now work and you should now be able to navigate from steps in your `.feature` files to your step definitions for example
