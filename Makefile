# Paso por defecto
all: build

# Compilar el contrato con la feature wasm-binary
build:
	cargo b -r

# Limpiar archivos generados
clean:
	cargo clean
	rm -rf out

# Testing
test:
	cargo t -p tests -r
