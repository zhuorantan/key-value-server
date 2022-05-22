# key-value-server

A simple key-value store http server written in Rust.

This is a toy project for experimental use. It is NOT designed for production.

# Usage

## Start the server

### Options

||Environment|Default|Use|
|-|-|-|-|
|`--host`|`KV_SERVER_HOST`|`localhost`|Listening host|
|`--port`|`KV_SERVER_PORT`|`8124`|Listening port|
|`--file-path`|`KV_SERVER_FILE_PATH`||File path for persisting data|

If `--file-path` is not provided, the data would only be stored in memory.

### Docker

```sh
docker build --tag kv-server https://github.com/zhuorantan/key-value-server
docker run -v ./data.json:/app/data.json -p 8124:8124 kv-server
```

By default, `--host` is `0.0.0.0` and `--file-path` is `/app/data.json` in this image.

## Client usage

### Set value

To set a value, use http `POST` or `PUT` request:

```
POST 'bar' http://localhost:1234/foo

result: {"foo": "bar"}
```

Value can also be a JSON string:

```
POST '{"port": 8124}' http://localhost:1234/foo

result: {"foo": {"port": 8124}}
```

Multiple levels are supported. Objects would be automatically created:

```
POST 'foo' http://localhost:1234/a/b/c

result: {"a": {"b": {"c": "foo"}}}
```

If one of the immediate values is not an object, the request is invalid:

```
{"a": {"b": "foo"}}

POST 'bar' http://localhost:1234/a/b/c

400 Bad Request
```

### Get value

To get a value, use `GET` request:

```
GET http://localhost:1234/foo

200 OK "bar"
```

For value that is not exist:

```
GET http://localhost:1234/bar

404 Not Found
```

### Remove value

To remove a value, use `DELETE` request:

```
DELETE http://localhost:1234/foo

200 OK
```

# License

[MIT license](LICENSE)
