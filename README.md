# Image-server

simple static image upload and hosting server, written in Rust Actix.

## Features

png, jpg, jpeg, gif, webp served as static files.

## Usage

```bash
$ cargo build --release
$ ./target/release/image-server
```

## API

### Upload

```bash
$ curl -F "file=@/path/to/image.jpg" -H "Authorization: your_token" -X POST http://localhost:8080/upload
```

### Get

```bash
$ curl http://localhost:8080/i/filename.jpg
```

### Delete

```bash
$ curl -H "Authorization: your_token" http://localhost:8080/delete/filename.jpg
```

## Environment Variables

| Name           | Description              | Default |
| -------------- | ------------------------ | ------- |
| `PASSWORD`     | Token for authorization  | `""`    |
| `REDIRECT_URI` | Redirect URI for `GET /` | `""`    |
