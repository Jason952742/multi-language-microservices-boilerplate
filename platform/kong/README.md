# kong setup

## Test Service And Route

### Create an example Service and a Route

```bash
curl -i -X POST \
  --url http://localhost:8001/services/ \
  --data 'name=example-service' \
  --data 'url=http://httpbin.org/anything'
```

Add a Route to the Service:

```bash
curl -i -X POST \
--url http://localhost:8001/services/example-service/routes \
--data 'paths[]=/auth-sample' \
--data 'name=example_route'
```

The url will now echo whatever is being requested. **Http://localhost:8000/auth-sample**

### Set up consumers and keys

Create a new consumer:

For the purposes of this tutorial, create a new consumer with a username: sample-consumer

```bash
curl -i -X POST http://localhost:8001/consumers/ \
 --data username=sample-key-consumer
```

Assign the consumer a key:

Once provisioned, call the Admin API to assign a key for the new consumer. For this tutorial, set the key value to: top-secret-key

```bash
curl -i -X POST http://localhost:8001/consumers/sample-key-consumer/key-auth \
 --data key=top-secret-key
```

### Global key authentication

Installing the plugin globally means every proxy request to Kong Gateway is protected by key authentication.

```bash
curl -X POST http://localhost:8001/plugins/ \
   --data "name=key-auth"  \
   --data "config.key_names=apikey"
```

## JWT Auth And Keycloak

### Create a Consumer

```bash
curl -X POST http://localhost:8001/consumers/ \
 --data username=member-jwt-consumer
```

### Create a JWT credential

First, get the public key from Keycloak. This is the signature of the token issued by Keycloak, so it is the public key
in the realm setup.

After obtaining the public key, you can provision a new RS256 JWT credential by issuing the following HTTP request:

```bash
curl -X POST http://localhost:8001/consumers/member-jwt-consumer/jwt \
--header 'Content-Type: application/x-www-form-urlencoded' \
--data-urlencode 'key=https://localhost:9443/realms/multi_lang' \
--data-urlencode 'algorithm=RS256' \
--data-urlencode 'rsa_public_key=-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAv8ShUrUpAVIa9FoZNFsjonJCj66kG493vQlmqxTcjNLsqCtJShBWuRnY+/s3AYqZCAm0CuT0Uw5NJbkOUUjGZdXZfwp6+FYsurv+CkAIKp2NDUlhPSJgfl5Jh2dT/hdEmpjhSM/q/VsLsVlHDnkje8tgZ9PstoVWOkX2TDftuFb5tbFCmj6o2t1/2NPvYftSNmv7sdBCxMr3I78TupAfJyhTKdDliEbVS8uSs0nrvzfehTyKaNG3LLXFyWKXTO11CVZ7YxxykPsqovQ2BHNaihcK70J+TWDX9q9OR4qKyFrt5YMgf1LSYCIKWCvXPbBkBLDsR3VlMMpYUVPJAxsD7QIDAQAB
-----END PUBLIC KEY-----'
```

### List JWT credentials

You can list a Consumer’s JWT credentials by issuing the following HTTP request:

```bash
curl -X GET http://localhost:8001/consumers/member-jwt-consumer/jwt
```

### Create a JWT Plugin for Route

```bash
curl -X POST http://localhost:8001/routes/example_route/plugins \
   --data "name=jwt"  \
   --data "config.uri_param_names=access_token"
```

Kong can also perform verification on registered claims, as defined in RFC 7519. To perform verification on a claim, add it to the config.claims_to_verify property:

You can patch an existing JWT plugin:

```bash
# This adds verification for both nbf and exp claims:
curl -X PATCH http://localhost:8001/plugins/704f0395-fc25-459e-8639-588c59d8042b \
 --data "config.claims_to_verify=exp"
```

### Test Auth

```bash
curl http://localhost:8000/auth-sample \
 -H 'Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJZSmRtYUR2VlRKeHRjV1JDdmtNaWtjOG9FTGdBVk5jeiIsImV4cCI6MTQ0MjQzMDA1NCwibmJmIjoxNDQyNDI2NDU0LCJpYXQiOjE0NDI0MjY0NTR9.WuLdHyvZGj2UAsnBl6YF9A4NqGQpaDftHjX18ooK8YY'
```
