### Generator

- The generator periodically randomises its probability of which HTTP methods and operations to send
- It may send a HEAD HTTP method (all frameworks reject it)
- It may send an invalid Operation (filter, frameworks do not support it)
- It may send an invalid payload identifier (random string or valid but random UUID) instead of the ID associated with the payload
- It may generate a very long secret which causes the servers to reject it because of maximum allowed payload size
