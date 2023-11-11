# nacos-multi-todo
Examples of how to use nacos and microservices in multiple programming languages

## Project directory

* env: Environment variable file for compose yaml
* docker: Docker compose for Nacos server and microservices

## Quick Start

```shell
docker run --name nacos-quick -e MODE=standalone -p 8848:8848 -p 9848:9848 -d nacos/nacos-server:v2.2.3
```

## Advanced Usage

* Tips: You can change the version of the Nacos image in the docker-compose file from the following configuration.
  `docker/.env`

```dotenv
NACOS_VERSION=v2.2.3
```

Run the following command：

* Clone project

  ```powershell
  git clone --depth 1 https://github.com/Jason952742/nacos-multi-todo
  cd nacos-multi-todo
  ```

* Standalone Derby

  ```powershell
  docker-compose -f docker/standalone-derby.yaml up
  ```
* Standalone Mysql

  ```powershell
  # Using mysql 5.7
  docker-compose -f docker/standalone-mysql-5.7.yaml up

  # Using mysql 8
  docker-compose -f docker/standalone-mysql-8.yaml up
  ```

* Standalone Nacos Cluster

  ```powershell
  docker-compose -f docker/cluster-hostname.yaml up 
  ```

* Service registration

  ```powershell
  curl -X POST 'http://127.0.0.1:8848/nacos/v1/ns/instance?serviceName=nacos.naming.serviceName&ip=20.18.7.10&port=8080'

  ```

* Service discovery

    ```powershell
    curl -X GET 'http://127.0.0.1:8848/nacos/v1/ns/instance/list?serviceName=nacos.naming.serviceName'
    ```

* Publish config

  ```powershell
  curl -X POST "http://127.0.0.1:8848/nacos/v1/cs/configs?dataId=nacos.cfg.dataId&group=test&content=helloWorld"
  ```

* Get config

  ```powershell
    curl -X GET "http://127.0.0.1:8848/nacos/v1/cs/configs?dataId=nacos.cfg.dataId&group=test"
  ```


* Open the Nacos console in your browser

  link：http://127.0.0.1:8848/nacos/

## Common property configuration

| name                                     | description                                                                                                                       | option                                                                                                                                                                                |
|------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| MODE                                     | cluster/standalone                                                                                                                | cluster/standalone default **cluster**                                                                                                                                                |
| NACOS_SERVERS                            | nacos cluster address                                                                                                             | eg. ip1:port1 ip2:port2 ip3:port3                                                                                                                                                     |
| PREFER_HOST_MODE                         | Whether hostname are supported                                                                                                    | hostname/ip default **ip**                                                                                                                                                            |
| NACOS_APPLICATION_PORT                   | nacos server port                                                                                                                 | default **8848**                                                                                                                                                                      |
| NACOS_SERVER_IP                          | custom nacos server ip when network was mutil-network                                                                             |                                                                                                                                                                                       |
| SPRING_DATASOURCE_PLATFORM               | standalone support mysql                                                                                                          | mysql / empty default empty                                                                                                                                                           |
| MYSQL_SERVICE_HOST                       | mysql  host                                                                                                                       |                                                                                                                                                                                       |
| MYSQL_SERVICE_PORT                       | mysql  database port                                                                                                              | default : **3306**                                                                                                                                                                    |
| MYSQL_SERVICE_DB_NAME                    | mysql  database name                                                                                                              |                                                                                                                                                                                       |
| MYSQL_SERVICE_USER                       | username of  database                                                                                                             |                                                                                                                                                                                       |
| MYSQL_SERVICE_PASSWORD                   | password of  database                                                                                                             |                                                                                                                                                                                       |
| MYSQL_DATABASE_NUM                       | It indicates the number of database                                                                                               | default :**1**                                                                                                                                                                        |
| MYSQL_SERVICE_DB_PARAM                   | Database url parameter                                                                                                            | default :**characterEncoding=utf8&connectTimeout=1000&socketTimeout=3000&autoReconnect=true&useSSL=false**                                                                            |
| JVM_XMS                                  | -Xms                                                                                                                              | default :1g                                                                                                                                                                           |
| JVM_XMX                                  | -Xmx                                                                                                                              | default :1g                                                                                                                                                                           |
| JVM_XMN                                  | -Xmn                                                                                                                              | default :512m                                                                                                                                                                         |
| JVM_MS                                   | -XX:MetaspaceSize                                                                                                                 | default :128m                                                                                                                                                                         |
| JVM_MMS                                  | -XX:MaxMetaspaceSize                                                                                                              | default :320m                                                                                                                                                                         |
| NACOS_DEBUG                              | enable remote debug                                                                                                               | y/n default :n                                                                                                                                                                        |
| TOMCAT_ACCESSLOG_ENABLED                 | server.tomcat.accesslog.enabled                                                                                                   | default :false                                                                                                                                                                        |
| NACOS_AUTH_SYSTEM_TYPE                   | The auth system to use, currently only 'nacos' is supported                                                                       | default :nacos                                                                                                                                                                        |
| NACOS_AUTH_ENABLE                        | If turn on auth system                                                                                                            | default :false                                                                                                                                                                        |
| NACOS_AUTH_TOKEN_EXPIRE_SECONDS          | The token expiration in seconds                                                                                                   | default :18000                                                                                                                                                                        |
| NACOS_AUTH_TOKEN                         |                                                                                                                                   | `Note: It is removed from Nacos 2.2.1`                                                                                                                                                |
| NACOS_AUTH_CACHE_ENABLE                  | Turn on/off caching of auth information. By turning on this switch, the update of auth information would have a 15 seconds delay. | default : false                                                                                                                                                                       |
| MEMBER_LIST                              | Set the cluster list with a configuration file or command-line argument                                                           | eg:192.168.16.101:8847?raft_port=8807,192.168.16.101?raft_port=8808,192.168.16.101:8849?raft_port=8809                                                                                |
| EMBEDDED_STORAGE                         | Use embedded storage in cluster mode without mysql                                                                                | `embedded` default : none                                                                                                                                                             |
| NACOS_AUTH_CACHE_ENABLE                  | nacos.core.auth.caching.enabled                                                                                                   | default : false                                                                                                                                                                       |
| NACOS_AUTH_USER_AGENT_AUTH_WHITE_ENABLE  | nacos.core.auth.enable.userAgentAuthWhite                                                                                         | default : false                                                                                                                                                                       |
| NACOS_AUTH_IDENTITY_KEY                  | nacos.core.auth.server.identity.key                                                                                               | `Note: It is removed from Nacos 2.2.1`                                                                                                                                                |
| NACOS_AUTH_IDENTITY_VALUE                | nacos.core.auth.server.identity.value                                                                                             | `Note: It is removed from Nacos 2.2.1`                                                                                                                                                |
| NACOS_SECURITY_IGNORE_URLS               | nacos.security.ignore.urls                                                                                                        | default : `/,/error,/**/*.css,/**/*.js,/**/*.html,/**/*.map,/**/*.svg,/**/*.png,/**/*.ico,/console-fe/public/**,/v1/auth/**,/v1/console/health/**,/actuator/**,/v1/console/server/**` |

## Advanced configuration

If you have a lot of custom configuration needs, It is highly recommended to mount `application.properties` in
production environment.

For example:

```docker
docker-compose -f docker/custom-application-config.yaml up -d
```

## Nacos + Grafana + Prometheus

Usage reference：[Nacos monitor-guide](https://nacos.io/en-us/docs/monitor-guide.html)

**Note**: When Grafana creates a new data source, the data source address must be **http://prometheus:9090**


