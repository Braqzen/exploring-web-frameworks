# Configuration

There are 2 configuration files to change the behaviour of the system:

- [Generator](../../configs/generator.json)
- [Services](../../configs/provider.json)

Both have defaults that may be overwritten on a per provider basis.

The generator controls which providers it sends requests to and which HTTP methods it uses to send to them.

The services control artificial latency and error rates.

To overwrite a provider take a look at the dummy override in each config which specifies the provider name as the key and the values inside `{}` similar to the defaults.

The override accepts partial structures e.g. if the structure is `{ "enabled": true, "rate": 5 }` you may use `{ "enabled": true }` without the `rate`.

The generator is set to crash when

- All frameworks are disabled

> Note: Disabling all HTTP methods on a provider treats it as disabled

After changing the generator config you must rebuild the generator image and after changing the provider config you must rebuild every service that the update applies to.
