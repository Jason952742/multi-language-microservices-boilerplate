# Analytics

## Running

1. Compile the project by entering:

```bash
./sbt compile
```

2. Run the server:

```bash
sbt "runMain api.Main"
```

3. Run the client, open another console window and enter:

```bash
sbt "runMain api.infra.grpc.GreeterClient"
```
