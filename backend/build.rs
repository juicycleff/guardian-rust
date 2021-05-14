extern crate prost_build;

fn main() {
    // build proto
    prost_build::compile_protos(&["src/service_proto.proto"], &["src/"]).unwrap();
}
