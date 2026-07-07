# Frameworks

The following behaviour is expected from frameworks:

- Must conform to the API doc
- Limit request size (likely 64kb)
- All responses must be consistent and in `json` unless a HTTP method disallows it
- Fallback mechanisms if a user calls a non-existent route, uses an incorrect method, sends nonsense to an existing route which would trigger a fallback route (so we always respond with appropriate `json` errors)
