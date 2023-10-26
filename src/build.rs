use ethers_contract::MultiAbigen;

fn main() {
    let abigen = MultiAbigen::from_json_files("./abi").unwrap();
    let bindings = abigen.build().unwrap();
    bindings.write_to_module("./src/bindings", false).unwrap();
}
