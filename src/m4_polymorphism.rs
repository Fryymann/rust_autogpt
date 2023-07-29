use ethers::types::Address;
use std::{str::FromStr, convert};

trait EthereumAddress {
    fn convert_address(&self) -> Result<Address, &'static str>;
}


impl EthereumAddress for &str {
    fn convert_address(&self) -> Result<Address, &'static str> {
        match Address::from_str(self) {
            Ok(address) => Ok(address),
            Err(_) => Err("Invalid Ethereum Address String")
        }
    }
}

impl EthereumAddress for Address {
    fn convert_address(&self) -> Result<Address, &'static str> {
        Ok(*self) //? Dereference sef
    }
}

fn get_ethereum_data<T: EthereumAddress>(address: T) -> Address {
    let converted_address: Address = address.convert_address().unwrap();
    converted_address
}

#[cfg(test)]
    use super::*;

    #[test]
    fn test_poly() {
        let address: Address = Address::from_str("0xa27CEF8aF2B6575903b676e5644657FAe96F491F")
            .unwrap();

        let new_addr: Address = get_ethereum_data("0xa27CEF8aF2B6575903b676e5644657FAe96F491F");
        assert_eq!(address, Address::from_str("0xa27CEF8aF2B6575903b676e5644657FAe96F491F").unwrap())
    }
