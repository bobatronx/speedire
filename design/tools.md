# Tools

## User-Defined Versions

Carrier needs to be able to install and configure tools needed to build and deploy applications. For example, given a Poetry application deployed to Kubernetes; Carrier would need to resolve the following:

- Download and install Poetry
    - Poetry has a dependency on Python, so Python should be automatically configured
- Download and install Python
    - Download specific version and build
    - Building Python has a dependeny on 'make' or 'python-build'
- Download and install 'make' or 'python-build'
- Download and install 'kubectl'

Carrier should be able to resolve tool dependencies automatically or be overriden by user input

## Possible Default Behavior

If tools are not configured, Carrier can defer to the system and provide meaningful errors if certain tools are not present on the system. However, it would be encouraged to configure the entire toolchain.

## Alternate Approach

Require that all tools are specifically configured with versions, or specify the system should be used for a specific tool rather than downloading, installing, and configuring that tool.