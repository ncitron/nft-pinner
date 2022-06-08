use ethers::contract::Abigen;

fn main() {
    gen("ERC165");
    gen("ERC721");
}

fn gen(name: &str) {
    let abi_path = "./abi/".to_owned() + name + ".json";
    let rs_path = "./src/bindings/".to_owned() + &name.to_lowercase() + ".rs";

    Abigen::new(name, abi_path)
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file(rs_path)
        .unwrap();
}
