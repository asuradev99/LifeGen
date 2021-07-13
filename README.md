# LifeGen #

LifeGen is an online WebAssembly and WebGL2-based 2d cellular automata simulator. 

## Status ##

Currently this project is a huge work-in-progress. So far I have merely set up the WebGl2 boilerplate. Below are the tasks I plan on completing: 

* Finish and optimize WebGl2 renderer for CA
* Implement UI with drag and touch features
* Add other CA such as Brian's Brain, Ants, etc. and enable support for other rulespaces
* possibly implement and tinker with particle-based automata

## Running ##

You will eventually be able to visit my personal website to use this tool online, but for now, you can build it yourself by installing wasm-pack:

```
# curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```


Then, if you have `make` and `npm` installed you can simply run the Makefile in the home directory to host this project in your browser. Alternatively, if you want to host it locally some other way, you can just build it with `wasm-pack`: 

```
# wasm-pack build --release --no-typescript --target web
```



