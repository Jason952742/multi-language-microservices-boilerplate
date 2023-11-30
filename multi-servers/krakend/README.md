# KrakenD Production Deployment

Simple docker-compose file to install KrakenD for production.

First, change the working directory
```bash
cd krakend
```

## Copy env file

Copy config file, then replace your host with node app running on host address with this:

```json
  {
    "url_pattern": "/realms/multi_lang/protocol/openid-connect/token",
    "method": "POST",
    "host": ["https://${your_host_ip}:8443"]
  }
```

## Installation

enter the following command to start KrakenD:

```bash
docker-compose up -d
```

## Test Auth
```bash
curl -kv --location 'https://${your_host_ip}:8080/api/v1/web-auth/login' \
--header 'Content-Type: application/x-www-form-urlencoded' \
--data-urlencode 'client_id=web-auth-client' \
--data-urlencode 'grant_type=password' \
--data-urlencode 'client_secret=your_client_secret' \
--data-urlencode 'scope=openid' \
--data-urlencode 'username=keeper' \
--data-urlencode 'password=${your_keeper_user_password}'
```

You will then see the following response:

```json
{
   "access_token":"xxxxx...xxxxx",
   "expires_in":300,
   "refresh_expires_in":1800,
   "refresh_token":"xxx...xxx",
   "token_type":"Bearer",
   "id_token":"xxx...xxx",
   "not-before-policy":0,
   "session_state":"xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
   "scope":"openid email profile"
}
```

Then authenticate using the request token in the response:

```bash
curl -kv --location 'https://${your_host_ip}:8080/api/v1/web-auth/user_info' \
--header 'Authorization: Bearer ${your_access_token}'
```

You will then see the following response:

```json
{
   "sub":"xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
   "email_verified":false,
   "name":"hello world",
   "preferred_username":"keeper",
   "given_name":"hello",
   "family_name":"world",
   "email":"keeper@world.io"
}

```
