## Configuration

You may edit the generator's [config](../generator/config.json) to specify which framework to send requests to and which HTTP methods to use.

The generator is made to crash when

- All frameworks are disabled
- A framework is enabled but all of its methods are disabled

After changing the config you must rebuild the generator image.
