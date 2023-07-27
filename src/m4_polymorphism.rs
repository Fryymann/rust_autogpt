use ethers::types::Address;
use std::str::FromStr;

trait EthereumAddress {
    fn convert_address(&self) -> Result<Address, &'static str> {

    }
}


impl EthereumAddress for &str {
    fn convert_address(&self) -> Result<Address, &'static str> {
        match Address::from_str(self) {
            Ok(address) => Ok(address),
            Err(_) => Err("Invalid Ethereum Address String")
        }
    }
}

fn get_ethereum_data<T>(address: T) {

}

#[cfg(test)]
    use super::*;

    #[test]
    fn test_polymorphism() {
        dbg!("TEST");
    }
