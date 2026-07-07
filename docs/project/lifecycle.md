## Data Flow

1. Generator randomly selects 1 framework to send a request to
2. Generator randomly selects a HTTP method to send
3. Generator performs method specific logic to send
4. The selected framework inspects the request and either accepts (and stores the information) or rejects the request
