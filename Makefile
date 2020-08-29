start: build
	http-server target/dist

build:
	wasm-pack build --dev --no-typescript --target web --out-name wasm --out-dir target/dist
	cp src/index.html target/dist/
