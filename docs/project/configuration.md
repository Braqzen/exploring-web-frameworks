# Configuration

You may edit the generator's [config](../configs/generator.json) to specify which framework to send requests to and which HTTP methods to use.

The generator is made to crash when

- All frameworks are disabled

> Note: Disabling all HTTP methods on a provider treats it as disabled

After changing the config you must rebuild the generator image.
