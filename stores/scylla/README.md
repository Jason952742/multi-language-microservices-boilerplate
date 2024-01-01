# Scylla

## change default user and password

in docker container, Connect to ScyllaDB's cqlsh terminal using the cqlsh command:

```shell
cqlsh -u cassandra -p cassandra
```

Create a new role and assign the specified KEYSPACES permission to that role:

```shell
CREATE ROLE your_username WITH PASSWORD = 'your_password' AND LOGIN = true;
GRANT ALL PERMISSIONS ON KEYSPACES  myks TO your_username;
```

change cassandra user password:

```shell
ALTER ROLE cassandra WITH PASSWORD = 'mypass';
EXIT;
```