# key-value-server

A simple key-value store http server written in Rust.

This is a toy project for experimental use. It is NOT designed for production.

# Usage

## Start the server

Directly run the executable to start the server at `localhost:8124`.

Use `--host` and `--port` options to customize listening address:

```sh
./key-value-server --host 0.0.0.0 --port 12345
```

Data would be stored only in memory by default. To persist the data, provide a file path using the option `--file-path`:

```sh
./key-value-server --file-path ./data.json
```

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
