# Bidirectional grpc

## Codegen

### Go
Run golang protoc compiler
```sh
protoc \
    --go_out=./golang-grpc \
    --go_opt=paths=source_relative \
    --go-grpc_out=./golang-grpc \
    --go-grpc_opt=paths=source_relative \
    proto/pingpong.proto
```
Use in code:
```go
import proto "golang-grpc/proto"
```
### Rust
Build script (build.rs) automatically creates grpc types for us on each compilation
```rs
mod proto {
    include!("../echo_unary.rs");
}
use proto::{echo_client::EchoClient, EchoRequest};
```