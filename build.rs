use ethers::contract::Abigen;

fn main() {
    // Only re-run the builder script if the contract changes
    println!("cargo:rerun-if-changed=./abi/*.json");
    bindgen("BentoBoxV1");
    bindgen("CauldronV2");
    bindgen("CauldronLiquidator");
    bindgen("ERC20");
}

#[allow(dead_code)]
fn bindgen(fname: &str) {
    let bindings = Abigen::new(fname, format!("./abi/{}.json", fname))
        .expect("could not instantiate Abigen")
        .generate()
        .expect("could not generate bindings");

    bindings
        .write_to_file(format!("./src/bindings/{}.rs", fname.to_lowercase()))
        .expect("could not write bindings to file");
}
