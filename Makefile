build:

protos: ./protos/*.proto
	@if [ ! -d "./protos" ]; then \
		echo "Error: The 'protos' directory does not exist."; \
		exit 1; \
	fi
	- protoc --plugin=./shortener-frontend/node_modules/.bin/protoc-gen-ts --ts_out ./shortener-frontend/src/ --proto_path ./ ./protos/*.proto
	# Cargo run/build automatically handles building the proto for us


clean_protos: ./shortener-frontend/src/protos/*.ts ./shortener-backend/src/protos/*.rs
	- rm ./shortener-frontend/src/protos/*.ts
	- rm ./shortener-backend/src/protos/*.rs