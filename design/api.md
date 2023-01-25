# Spedire API

## CLI vs Library

The cli should setup a new Spedire project with a very basic build pipeline including self-install of Spedire, Github actions, Spedire skeleton pipeline.

## Public API

The public API should expose tool configuration only. Download and install should happen as part of the private API. In addition there should be a way to execute build and deploy steps for each tool. Whatever is exposed for build/deploy is up to each plugin. They can be very opinionated or flexible or have versions for both.

## Example for kubectl

User creates a new kubectl metadata object and can call configure() and execute() on that object.

Possible example:

```
let kubectl_metadata = KubectlMetadata::new();
kubectl_metadata.configure()
kubectl_metadata.execute("apply -f test.yaml");
```