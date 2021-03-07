fn main() {
    tonic_build::compile_protos("../protos/roombaservice/roombaservice.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
