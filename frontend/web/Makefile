serve:
	npm run serve
build: 
	npm run build
run: build serve 
dev: 
	@$(MAKE) run -j2
proto:
	mkdir src/generated & \
	protoc -I=../../backend/grpc/proto \
	--js_out=import_style=commonjs:./src/generated \
	--grpc-web_out=import_style=typescript,mode=grpcwebtext:./src/generated \
	../../backend/grpc/proto/greeter.proto
proto_update: 
	protoc -I=../../backend/grpc/proto \
	--js_out=import_style=commonjs:./src/generated \
	--grpc-web_out=import_style=typescript,mode=grpcwebtext:./src/generated \
	../../backend/grpc/proto/greeter.proto