# Keycloak Production Development

Simple docker-compose file to install Keycloak for production.

## Installation

First, change the working directory
```bash
cd keycloak
```

### Copy env file

Copy config file, then set the environment variables for this to work properly

```bash
cp /.env.example /.env
```

### Generating TLS Certificates and Keys for HTTPS

```bash
# This generates a private key file called tls.key
openssl genrsa -out tls.key 2048
# Generating a Certificate Signing Request File (CSR File)
# In this step, you will need to provide some certificate request information such as country, province, city, organisation, and so on. 
# Follow the prompts to fill in this information step by step
openssl req -new -key tls.key -out tls.csr
# Generate self-signed certificate file (tls.crt)
openssl x509 -req -days 365 -in tls.csr -signkey tls.key -out tls.crt
```

Now you will have tls.key and tls.crt files in the current directory, which represent the private key and certificate files respectively.

* Note that the generated certificates are self-signed certificates, and for production environments you may need to request a certificate from a trusted certificate authority (CA). Also, if you intend to use these certificates in a production environment, you will need to ensure that the private keys and certificates are configured for appropriate protection and security

### Docker Deploy

enter the following command to start Keycloak:

```bash
docker-compose up -d
```
This command starts Keycloak exposed on the local port 8443 and creates an initial admin user with the username ***admin*** and password ***admin***.

 go to the [Keycloak Admin Console](https://localhost:8443/admin), Log in with the username and password you created earlier.

## Configuration

### Create a realm

A realm in Keycloak is equivalent to a tenant. Each realm allows an administrator to create isolated groups of applications and users. Initially, Keycloak includes a single realm, called ***master***. Use this realm only for managing Keycloak and not for managing any applications.

Use these steps to create the first realm.

- Open the Keycloak Admin Console.
- Click the word **master** in the top-left corner, then click **Create Realm**.
- Enter multi_lang in the Realm name field.
- Click Create.

### Create a user

Initially, the realm has no users. Use these steps to create a user:

1. Click **Users** in the left-hand menu. Then click **Add User**.
2. Fill in the form with the Username ***keeper***.
3. Click **Create**.

### Initial Password
This user needs a password to log in. To set the initial password:

1. Click **Credentials** at the top of the page.
2. Fill in the **Set password** form with a password.
3. Toggle **Temporary** to **Off** so that the user does not need to update this password at the first login.

### Create Client

1. Click **Clients**
2. Click **Create client**
3. Fill in the form with the following values:
    - **Client type**: OpenID Connect
    - **Client ID**: web-auth-client
4. Click **Next**
5. Confirm that **Standard flow** is enabled
6. Set Client **authentication** is **On** and **Authorization** is **On**
7. Click **Next**
8. Click **Sava**

After saving the client, you need to switch to the advanced settings page, scroll the page to the bottom and set:
- **Browser Flow**: browser
- **Direct Grant Flow**: direct grant
