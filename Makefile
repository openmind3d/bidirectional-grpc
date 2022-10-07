generate_go_code:
	protoc \
		--go_out=./golang-grpc \
		--go_opt=paths=source_relative \
		--go-grpc_out=./golang-grpc \
		--go-grpc_opt=paths=source_relative \
		proto/pingpong.proto