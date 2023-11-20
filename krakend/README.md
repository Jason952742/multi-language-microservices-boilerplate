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
