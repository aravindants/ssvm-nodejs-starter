# Getting started
This is a simple command line utility for conversion between arabic and roman numerals.

This is made as a part of : [Second State Raspberry Pi Kit-Wave 2](https://www.secondstate.io/articles/raspberry-pi-for-free-wave-two)

* The Rust functions are in the `src` directory. You can put high performance workload into Rust functions.
* The JavaScript functions are in the `node` directory and they can access the Rust functions.
* Use the `node node/app` command for help.
* Using `node node/app --value=XX --to=arabic` will print `20`.
* Using `node node/app --value=20 --to=roman` will print `XX`.


## Use Docker to build and run

```
$ docker pull secondstate/ssvm-nodejs-starter:v1
$ docker run -p 3000:3000 --rm -it -v $(pwd):/app secondstate/ssvm-nodejs-starter:v1
(docker) # cd /app
(docker) # ssvmup build
(docker) # node node/app.js
```


## Use VSCode Codespace

<p>
    <a href="https://online.visualstudio.com/environments/new?name=Rust%20and%20WebAssembly%20in%20Node.js&repo=second-state/ssvm-nodejs-starter">
        <img src="https://img.shields.io/endpoint?style=social&url=https%3A%2F%2Faka.ms%2Fvso-badge">
    </a>
</p>

![SSVM](https://github.com/second-state/blog/blob/master/static/images/SSVM-edited-without-music.gif?raw=true)

This project template works with the VS Codespaces online IDE! Code, build, and run directly from inside the browser. No software download or install needed! Check out the [high-res screencast](https://youtu.be/j85cbNsciOs).

> VS Codespaces runs entirely in your browser and costs around $1 per work day. It is cheaper than a cup of coffee in the office. Alternatively, use locally installed VSCode and Docker, and [launch the IDE with your remote git repository](https://code.visualstudio.com/remote-tutorials/containers/getting-started).

1 First, open the [VS Codespaces](https://online.visualstudio.com/) web site and login with your Azure account. You can get a [free Azure account](https://azure.microsoft.com/en-us/free/).

2 Next, create a new Codespace. Put your forked repository into the Git Repository field.

![Create a new Codespace](docs/img/vscode_create.png)

3 Then open the `src/lib.rs`, `node/app.js` and `Cargo.toml` files and see how the Node.js express app calls the Rust function to say hello.

![Code in Codespace](docs/img/vscode_code.png)

4 Click on the Run button on the left panel, and then the Launch Program at the top to build and run the application.

![Build and run](docs/img/vscode_run.png)

The Terminal window at the bottom shows the build progress. It builds the Rust program, and then launches the Node.js app.

![Build](docs/img/vscode_build.png)


## Resources

* [The Second State VM (SSVM)](https://github.com/second-state/ssvm) is a high performance [WebAssembly virtual machine](https://www.secondstate.io/ssvm/) designed for server-side applications.
* [The SSVM NPM addon](https://github.com/second-state/ssvm-napi) provides access to the SSVM, and programs in it, through a Node.js host application.
* [The SSVM ready tool, ssvmup](https://github.com/second-state/ssvmup) is a [toolchain](https://www.secondstate.io/articles/ssvmup/) for compiling Rust programs into WebAssembly, and then make them accessible from JavaScripts via the SSVM.


