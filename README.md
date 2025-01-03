# Betty Blocks Runtime WasmCloud

This is a WasmCloud Application project. You need to run this on WasmCloud. Install WasmCloud on a Mac via

```bash
brew install wasmcloud/wasmcloud/wash
```

Other OS' check the [Docs](https://wasmcloud.com/docs/installation)

## Running the Runtime WasmCloud

First start WasmCloud via

```bash
wash up -d
```

The -d will start is as a daemon.

Go to the root folder of this project and deploy the application

```bash
wash app deploy local.wadm.yaml
```

The application and it's providers and components are now deployed to the WasmCloud.

## Test the application

You can test it in a browser by going to `http://localhost:8000`. It wil show that post requests are not supported.

In a terminal run a POST call to check if it works.

```bash
curl -d "POST_BODY" http://localhost:8000 -H "Authorization: Bearer"
```
