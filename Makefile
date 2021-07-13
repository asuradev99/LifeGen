all:
	wasm-pack build --release --no-typescript --target web
	npx serve .