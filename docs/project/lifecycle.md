# Request Lifecycle

There are two components in the system:

- **Generator**: create and send request
- **Framework**: process and respond

Lifecycle:

1. Generator randomly selects **one** framework to send a request to
2. Generator randomly selects a HTTP method to send to that framework
3. Generator performs HTTP method-specific logic such as creating a payload
4. Generator sends the request to the framework
5. Framework performs request validation
   1. Pass: Perform method-specific logic like store, update, get, etc. and respond to generator
   2. Fail: Json error response
6. Generator handles response related to HTTP method and success/failure

More information on selection mechanisms, payload creation, API behaviour can be found in other pages.
