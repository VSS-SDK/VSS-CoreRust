extern crate protoc_rust;

fn main() {

    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/protos",
        input: &["protos/command.proto", "protos/control.proto", "protos/debug.proto", "protos/state.proto"],
        includes: &["protos"]
    }).expect("protoc");

}