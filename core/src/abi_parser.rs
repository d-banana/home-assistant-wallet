use ethers::prelude::Abigen;

pub struct AbiFile{
    pub path_abi: String,
    pub path_rs: String,
    pub name: String,
}
pub struct Error;

pub fn build_smart_contract(abi_file_list: Vec<AbiFile>) -> Result<(), Error> {
    for abi_file in abi_file_list{
        Abigen::new(
            &abi_file.name,
            &abi_file.path_abi).map_err(|_| Error)?
            .generate().map_err(|_| Error)?
            .write_to_file(abi_file.path_rs).map_err(|_| Error)?;
    }

    Ok(())
}