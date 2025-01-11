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

You can test it in a browser by going to `http://localhost:8000/graphql`. It wil show that post requests are not supported.

Add the temp cloud artefact to the KV storen by calling `http://localhost:8000/artefact-webhook`.

In a terminal run a POST call to check if it works.

```bash
curl --location 'localhost:8000/graphql' \
--header 'Authorization: eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJhcHBfdXVpZCI6IjY5M2IyMmU5ODNmYjQ2YWZhNGViMzUzZDgyZWNlNGJiIiwiYXVkIjoiSm9rZW4iLCJhdXRoX3Byb2ZpbGUiOiI1YmY5ZWJhMzQ2MzY0OTVkODBlZDVhNzkwY2EzOTA3NyIsImNhc190b2tlbiI6ImQ2NTI1ODU5NjRlY2ZkNTliZDczOGJiMzNmNWE0MjFjZTg1YzQ5M2UiLCJleHAiOjE3MzYwODA1NDIsImlhdCI6MTczNjA3MzM0MiwiaXNzIjoiSm9rZW4iLCJqdGkiOiIzMGJzYjBkb2lidHUycHJ0NTAwMDcwZDMiLCJsb2NhbGUiOm51bGwsIm5iZiI6MTczNjA3MzM0Miwicm9sZXMiOlsxXSwidXNlcl9pZCI6MX0.qnlhrIcCbVSf5szKBJSnjsVLz_b8Cem-Bfwe6u-_921UYS9qRGJGpsZ9Sr7aAGz1NwC78eXT0GTuAz4fL28k_A' \
--data 'fakebody
'
```

In this JWT there is an `app_uuid` which will be checked with the cloud artefact.

If no JWT is supplied in the header you will get an error.
