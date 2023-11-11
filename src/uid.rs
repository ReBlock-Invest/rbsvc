use ethers::{
    core::k256::ecdsa::SigningKey,
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer, Wallet},
    types::{Address, TransactionReceipt, U256},
};

use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse};
use eyre::Result;
use redis::Commands;
use serde::{Deserialize, Serialize};
use std::env;
use std::str::FromStr;
use std::sync::Arc;

abigen!(
    IUID,
    r#"[
        function mintTo(address recipient, uint256 id) public payable
        function balanceOf(address, uint256) returns (uint256)
    ]"#,
);

fn connect() -> redis::Connection {
    let redis_url = env::var("REDIS_URL").expect("missing environment variable REDIS_URL");

    redis::Client::open(redis_url)
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis")
}

struct UID {
    contract: IUID<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
}

impl UID {
    fn new(rpc_url: &str, uid_addr: Address, wallet: LocalWallet) -> Result<Self> {
        let chain_id = env::var("CHAIN_ID").expect("missing environment variable CHAIN_ID");
        let provider = Provider::<Http>::try_from(rpc_url).expect("Provider not found");
        let wallet = wallet.with_chain_id(chain_id.parse::<u64>().unwrap());
        let signer = Arc::new(SignerMiddleware::new(provider, wallet));
        let contract = IUID::new(uid_addr, signer);

        Ok(Self { contract })
    }

    async fn mint(
        self,
        recipient: Address,
        id: U256,
    ) -> Result<std::option::Option<TransactionReceipt>> {
        let tx = self.contract.mint_to(recipient, id);
        let pending_tx = tx.send().await?;
        let mined_tx = pending_tx.await?;
        Ok(mined_tx)
    }

    async fn balance_of(&mut self, recipient: Address, id: U256) -> Result<U256> {
        let balance = self.contract.balance_of(recipient, id).call().await?;
        Ok(balance)
    }
}

pub async fn mint_uid(recipient: &str, id: u8) -> Result<TransactionReceipt> {
    let rpc_url = env::var("RPC_URL")?;
    let account = env::var("ACCOUNT_PKEY")?.parse::<LocalWallet>()?;
    let recipient_addr = Address::from_str(recipient).expect("Invalid Ethereum address format");

    let uid_addr: Address = env::var("UID_CONTRACT_ADDRESS")?.parse()?;
    let uid = UID::new(&rpc_url, uid_addr, account.clone())?;

    let trx = uid.mint(recipient_addr, U256::from(id)).await?;

    let tr = trx.unwrap();

    Ok(tr)
}

pub async fn is_eligible(recipient: &str, id: u8) -> Result<bool> {
    let rpc_url = env::var("RPC_URL")?;
    let account = env::var("ACCOUNT_PKEY")?.parse::<LocalWallet>()?;
    let recipient_addr = Address::from_str(recipient).expect("Invalid Ethereum address format");
    let id256 = U256::from(id);

    let uid_addr: Address = env::var("UID_CONTRACT_ADDRESS")?.parse()?;
    let mut uid = UID::new(&rpc_url, uid_addr, account.clone())?;

    let balance = uid.balance_of(recipient_addr, id256).await?;

    Ok(balance.is_zero())
}

#[derive(Deserialize, Debug)]
pub struct IssueReq {
    recipient: String,
}

#[derive(Serialize, Debug)]
pub struct IssueResp {
    trx_id: String,
}

pub async fn issue_uid(req: HttpRequest, data: web::Json<IssueReq>) -> HttpResponse {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(token) = auth_header.to_str() {
            let mut conn = connect();
            let key = format!("rb:session:{}", token);

            let user_address = conn.get(&key).unwrap_or_else(|_err| {
                return String::from("");
            });

            if data.recipient != user_address {
                return HttpResponse::Unauthorized()
                    .content_type(ContentType::json())
                    .body("{\"error\": \"Invalid recipient address\"}");
            }
        } else {
            return HttpResponse::Unauthorized()
                .content_type(ContentType::json())
                .body("{\"error\": \"Invalid auth header\"}");
        }
    } else {
        return HttpResponse::Unauthorized()
            .content_type(ContentType::json())
            .body("{\"error\": \"Missing auth header\"}");
    }

    let eligible: bool = is_eligible(&data.recipient, 1u8).await.unwrap();
    if !eligible {
        return HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body("{\"message\": \"Address is not eligible\"}");
    }

    let tr = mint_uid(&data.recipient, 1u8).await.expect("error");

    let resp = IssueResp {
        trx_id: tr.transaction_hash.to_string(),
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(resp)
}
