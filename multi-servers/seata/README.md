# Seata Deployment and Configuration

## Precautions

 - Please avoid directly pulling the latest version image. The latest version is not necessarily a stable version. To avoid unnecessary problems, please go to docker image warehouse to determine the image version to be pulled.
 - Starting from Seata Server version 1.5.0, the configuration file has been changed to application.yml. So when using custom configuration, you need to copy the native configuration first.

## No Registration Center, File Storage

In this mode, there is no need for a registration center or any third-party storage center.

```yaml
version: "3.1"
services:
  seata-server:
    image: seataio/seata-server:${latest-release-version}
    hostname: seata-server
    ports:
      - "7091:7091"
      - "8091:8091"
    environment:
      - SEATA_PORT=8091
      - STORE_MODE=file
```
