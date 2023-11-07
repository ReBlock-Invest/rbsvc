## Endpoints

### get nonce

```
curl --location 'http://178.79.160.20:8000/nonce' \
--header 'Content-Type: application/json' \
--data '{
    "address": "0xAA"
}'
```

### wallet login

```
curl --location 'http://178.79.160.20:8000/wallet/login' \
--header 'Content-Type: application/json' \
--data '{
    "address": "0xAA",
    "signature": "AAbb1100"
}'
```

### issue UID

```
curl --location 'http://178.79.160.20:8000/issue' \
--header 'Content-Type: application/json' \
--data '{
    "type_id": "1",
    "recipient": "0xBB11"
}'
```

### Type ID

```
ID_NON_US_INDIVIDUAL = 1;
ID_US_ACCR_INDIVIDUAL = 2;
ID_US_NON_ACCR_INDIVIDUAL = 3;
ID_US_ENTITY = 4;
ID_NON_US_ENTITY = 5;
```
