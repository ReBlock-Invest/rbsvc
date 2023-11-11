## Endpoints

### get nonce

```
curl --location 'https://rb.blocknaut.xyz/nonce' \
--header 'Content-Type: application/json' \
--data '{
    "address": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
}'
```

```
{
    "nonce": 0,
    "address": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
}
```

### wallet login

```
curl --location 'https://rb.blocknaut.xyz/wallet/login' \
--header 'Content-Type: application/json' \
--data '{
    "address": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
    "signature": "0xef8ba019f69804fe418cdbd5e71188fa38e8c0975ac3c38c7bcdf6fb3dc6535076c8cb510e55d08a6fa27b45123a144209725a6c85a482ed9d73723a4681591a1c"
}'
```

```
{
    "access_token": "b5EcCeW7wedxN3Zd8a9H"
}
```

### issue UID

```
curl --location 'https://rb.blocknaut.xyz/issue' \
--header 'Authorization: b5EcCeW7wedxN3Zd8a9H' \
--header 'Content-Type: application/json' \
--data '{
    "recipient": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
}'
```

```
{
    "trx_id": "0xaaBa",
}
```

### user INFO

```
curl --location 'https://rb.blocknaut.xyz/userinfo' \
--header 'Authorization: b5EcCeW7wedxN3Zd8a9H' \
--data ''
```

```
{
    "invest_state": "WALLET_VERIFIED",
    "user_type": "",
    "address": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
}
```

### User Invest State

```
WALLET_VERIFIED
PENDING_KYC  // kyc inquiry completed
KYC_VERIFIED // kyc inquiry approved
```

### Type ID

```
ID_NON_US_INDIVIDUAL = 1;
ID_US_ACCR_INDIVIDUAL = 2;
ID_US_NON_ACCR_INDIVIDUAL = 3;
ID_US_ENTITY = 4;
ID_NON_US_ENTITY = 5;
```
