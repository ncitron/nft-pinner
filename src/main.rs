pub mod bindings;
use crate::bindings::erc165::ERC165;
use crate::bindings::erc721::ERC721;

use ethers::prelude::*;
use eyre::eyre;
use eyre::Result;
use hex::FromHex;
use std::fmt::Debug;
use std::sync::Arc;
use serde::Deserialize;

#[tokio::main]
async fn main() -> Result<()> {
    let address = "0x8a90CAb2b38dba80c64b7734e58Ee1dB38B8992e".parse::<Address>()?;

    let client =
        Client::new("rpc-url-goes-here")?;

    let base_uri = client.get_base_uri(address).await?;
    let image_uri = client.get_image_uri(address, 5).await?;
    
    println!("base uri: {}", base_uri);
    println!("image uri: {}", image_uri);

    client.pin_uri(&base_uri).await?;
    client.pin_uri(&image_uri).await?;

    Ok(())
}

struct Client {
    provider: Arc<Provider<Http>>,
}

#[derive(Deserialize)]
struct Metadata {
    image: String
}

#[derive(Debug)]
enum TokenType {
    ERC721,
    ERC1155,
}

impl Client {
    fn new(rpc: &str) -> Result<Client> {
        let provider = Provider::<Http>::try_from(rpc)?;
        Ok(Client {
            provider: Arc::new(provider),
        })
    }

    async fn pin_uri(&self, uri: &str) -> Result<()> {
        let req = "http://127.0.0.1:5001/api/v0/pin/add?arg=".to_string() + &uri.to_string();
        let client = reqwest::Client::new();
        let resp = client.post(req).send().await?;
        println!("{}", resp.status());
        Ok(())
    }

    async fn get_image_uri(&self, address: Address, id: usize) -> Result<String> {
        let token_type = self.get_token_type(address).await?;
        let token_uri: String = match token_type {
            TokenType::ERC721 => self.get_token_uri_erc721(address, id).await,
            TokenType::ERC1155 => Err(eyre!("ERC1155 support not implemented")),
        }?;
        let token_uri = strip_ipfs_prefix(&token_uri)?;

        let req = "http://127.0.0.1:5001/api/v0/cat?arg=".to_string() + &token_uri.to_string();
        let client = reqwest::Client::new();
        let metadata: Metadata = client.post(req)
            .send()
            .await?
            .json()
            .await?;
        
        Ok(strip_ipfs_prefix(&metadata.image)?.to_string())
    }

    async fn get_base_uri(&self, address: Address) -> Result<String> {
        let token_type = self.get_token_type(address).await?;
        let token_uri: String = match token_type {
            TokenType::ERC721 => self.get_token_uri_erc721(address, 1).await,
            TokenType::ERC1155 => Err(eyre!("ERC1155 support not implemented")),
        }?;

        let stripped = strip_ipfs_prefix(&token_uri)?;
        let base = stripped.split("/").collect::<Vec<_>>()[0];
        Ok(base.to_string())    
    }

    async fn get_token_uri_erc721(&self, address: Address, id: usize) -> Result<String> {
        let nft = ERC721::new(address, self.provider.clone());
        let token_uri = nft.token_uri(U256::from(id)).call().await?;

         Ok(token_uri)   
    }

    async fn get_token_type(&self, address: Address) -> Result<TokenType> {
        let nft = ERC165::new(address, self.provider.clone());

        let is_erc721 = nft
            .supports_interface(<[u8; 4]>::from_hex("5b5e139f")?)
            .call()
            .await?;
        let is_erc1155 = nft
            .supports_interface(<[u8; 4]>::from_hex("0e89341c")?)
            .call()
            .await?;

        if is_erc721 {
            Ok(TokenType::ERC721)
        } else if is_erc1155 {
            Ok(TokenType::ERC1155)
        } else {
            Err(eyre!("Token is not an ERC721 or ERC1155"))
        }
    }
}

fn strip_ipfs_prefix(uri: &str) -> Result<&str> {
    let stripped = uri.strip_prefix("ipfs://");
    match stripped {
        Some(uri) => Ok(uri),
        None => Err(eyre!("this nft does not use ipfs :("))
    }
}

