# The Ray Tracer Challenge: A Test-Driven Guide to Your First 3D Renderer

![Cover image](resources/images/chapter%207/chapter7.png)

This project is an ongoing Rust implementation of [The Ray Tracer Challenge](http://raytracerchallenge.com) book.

## Design choices

This project is using the *left-handed* system just like the book, however one major difference is that I used the **+Z** convention for the camera unlike the book which is using the **-Z** one.

## Tests

Tests are written in [Cucumber](https://cucumber.io) using [cucumber-rs](https://github.com/cucumber-rs/cucumber). 

Gherkin scenarios are copied from the given scenarios in the book. I only adapted the values in some steps due to my choice of diverging from the -Z convention, all the rest is *strictly* identical.

## Setup

### Visual Studio Code

You can get full support by simply installing [the official Cucumber extension](https://marketplace.visualstudio.com/items?itemName=CucumberOpen.cucumber-official)

### Zed

1. Close Zed
2. Install the Cucumber Language Server *globally* (with Node version > 18)
    ```sh
    npm install -g @cucumber/language-server
    ```
3. Clone [this fork](https://github.com/jmevel/zed-extension-cucumber)
4. Reopen Zed then `Extensions` -> `Install Dev Extension` -> Select the root path of `zed-extension-cucumber`
5. Cucumber should be fully working now, giving you Gherkin syntax highliting, `Go to definition`, `Define in ...` and other nice features

⚠️ If you're using a custom NPM prefix make sure your `PATH` contains the new `bin` folder location otherwise Zed won't be able to figure it out and will default to downloading its own broken version of the cucumber language server.

## AI

Not a single line of code was written by an LLM, everything has been proudly written by my own hands (except the Gherkin, of course).

![No AI icon](resources/images/no-ai-icon-04.png)
