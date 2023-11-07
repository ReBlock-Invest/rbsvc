use ethers::{
    core::k256::ecdsa::SigningKey,
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer, Wallet},
    types::Address,
};

use actix_web::{http::header::ContentType, web, HttpResponse};
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;

abigen!(
    IUID,
    r#"[
        function mintTo(address recipient, uint256 id, uint256 expiresAt, bytes calldata signature)
    ]"#,
);

struct UID {
    contract: IUID<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
}

impl UID {
    fn new(rpc_url: &str, uid_addr: Address, wallet: LocalWallet) -> Result<Self> {
        let provider = Provider::<Http>::try_from(rpc_url).expect("Provider not found");
        let wallet = wallet.with_chain_id(80001u64);
        let signer = Arc::new(SignerMiddleware::new(provider, wallet));
        let contract = IUID::new(uid_addr, signer);

        Ok(Self { contract })
    }

    async fn mint(self) -> Result<()> {
        /*  let tx = self.contract.mintTo();
                let pending_tx = tx.send().await?;
                let _mined_tx = pending_tx.await?;
        */
        Ok(())
    }
}

pub async fn mint_uid() -> Result<()> {
    let rpc_url = env::var("RPC_URL")?;
    let account = env::var("ACCOUNT_PKEY")?.parse::<LocalWallet>()?;

    let uid_addr: Address = env::var("UID_CONTRACT_ADDRESS")?.parse()?;
    let uid = UID::new(&rpc_url, uid_addr, account);

    // let signature = account.sign_message("hello world").await?;
    //uid.mint()?.await;

    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct IssueReq {
    type_id: String,
    recipient: String,
}

#[derive(Serialize, Debug)]
pub struct IssueResp {
    trx_id: String,
}

pub async fn issue_uid(form: web::Json<IssueReq>) -> HttpResponse {
    println!("{:?}", form);

    let resp = IssueResp {
        trx_id: String::from("0xAABB"),
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(resp)
}
