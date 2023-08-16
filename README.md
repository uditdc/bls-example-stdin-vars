# Blockless Stdin over HTTP

This is a sample blockless app demonstrating STDIN over Http.

### Locally

Run the following command:

`bls functions invoke --stdin your-stdin-input`

Result:
`stdinString: your-stdin-input`

### Via PATH

Run the following command:

```
curl -X GET \
  https://example-stdin-vars-8ceb538e.rc2.bls.dev/your-stdin-from-path
```

Result:
`stdinString: /your-stdin-from-path`

### Via GET

Run the following command:

```
curl -X GET \
  https://example-stdin-vars-8ceb538e.rc2.bls.dev?stdin=your-stdin-input-from-get
```

Result:
`stdinString: your-stdin-input-from-get`

### Via POST

Run the following command:

```
curl -X POST \
  -H "Content-Type: text/plain" \
  -d 'your-stdin-input-from-post' \
  https://example-stdin-vars-8ceb538e.rc2.bls.dev
```

Result:
`stdinString: your-stdin-input-from-post`

### Via POST using application/json

Run the following command:

```
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{"stdin": "your-stdin-input-from-post-json"}' \
  https://example-stdin-vars-8ceb538e.rc2.bls.dev
```

Result:
`stdinString: your-stdin-input-from-post-json`

## Connecting via rc2.bls.dev

Using the Blockless CLI, login via:

`bls login -u rc2.bls.dev`