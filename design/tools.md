# Tools

## User-Defined Versions

Spedire needs to be able to install and configure tools needed to build and deploy applications. For example, given a Poetry application deployed to Kubernetes; Spedire would need to resolve the following:

- Download and install Poetry
    - Poetry has a dependency on Python, so Python should be automatically configured
- Download and install Python
    - Download specific version and build
    - Building Python has a dependeny on 'make' or 'python-build'
- Download and install 'make' or 'python-build'
- Download and install 'kubectl'

Spedire should be able to resolve tool dependencies automatically or be overriden by user input

## Possible Default Behavior

If tools are not configured, Spedire can defer to the system and provide meaningful errors if certain tools are not present on the system. However, it would be encouraged to configure the entire toolchain.

## Alternate Approach

Require that all tools are specifically configured with versions, or specify the system should be used for a specific tool rather than downloading, installing, and configuring that tool.

## Handling File System and Paths

- Spedire should check for the existence of the `~/.local/spedire/bin` directory and create it if it's not there.
- All tools should be downloaded to a folder `~/.local/spedire/tmp`
- All tools should be copied to a specific location under tmp following the pattern `<tool-name>-<version>-<binary-name>`
