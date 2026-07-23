# API

This API section covers

- What the generator does
- How the web-frameworks should respond

## Generator

The two relevant sections are

- What payload does the generator send
- What HTTP methods does the generator call

### Request Payload

The full payload consists of the following json

```json
{
    secret: String,
    operation: String
}
```

The `secret` is a randomly generated string and the `operation` is a enum of 5 arbitrary variants (they have no functionality).

```rust
enum Operation {
    Transform,
    Merge,
    Sort,
    Compute,
    Filter
}
```

### HTTP Methods and Routes

The generator uses the following methods: `POST`, `GET`, `DELETE`, `PATCH`, `PUT`, `HEAD`.

#### POST

Send the full payload to the route `/` and receive a task uuid.

#### GET

Send the task id to `/:id` to get a task from the called framework.

#### DELETE

Send the task id to `/:id` to delete the task in the called framework.

#### PUT

Send a new full payload with the corresponding task id to `/:id` to overwrite the task in the called framework.

#### PATCH

Send a payload consisting of the `operation` but not the `secret` with the corresponding task id to `/:id` to update the task in the called framework.

#### HEAD

Send the task id to `/:id`.

## Web-Frameworks

### Payloads

The frameworks should only accept 4 of the 5 `operation` variants. The variant that should be rejected is `Filter`.

The task id should always be a valid `uuid` that they currently store and the `secret` must not make the payload size exceed 64kb.

### HTTP Methods

The only supported routes are `POST` to `/` and `GET`, `DELETE`, `PATCH`, `PUT` to `/:id`. Everything else should be rejected e.g. the `HEAD` the generator sends.

### Responses

Frameworks must always respond appropriately i.e. attempt to follow any RFCs for which status codes to return, when a body should be returned, and all bodies must have the same format i.e. json.

| #   | Case                                                    | Status | Body                                        |
| --- | ------------------------------------------------------- | ------ | ------------------------------------------- |
| 1   | `POST /` with a valid body                              | `201`  | `{ "id": "<uuid>" }`                        |
| 2   | `GET /:id` for a stored uuid                            | `200`  | `{ "secret": String, "operation": String }` |
| 3   | `PUT /:id` for a stored uuid with a valid body          | `200`  | `{ "secret": String, "operation": String }` |
| 4   | `PATCH /:id` for a stored uuid with a valid body        | `200`  | `{ "secret": String, "operation": String }` |
| 5   | `DELETE /:id` for a stored uuid                         | `204`  | empty                                       |
| 6   | Unsupported method (e.g. any other method on any route) | `405`  | `{ "error": "Invalid method" }`             |
| 7   | Unknown path (e.g. any other path including non-uuid)   | `404`  | `{ "error": "Invalid path" }`               |
| 8   | `/:id` is a uuid that is not stored                     | `404`  | `{ "error": "Task not found" }`             |
| 9   | Body is not valid JSON                                  | `422`  | `{ "error": "Invalid body JSON" }`          |
| 10  | Incorrect body shape for given route                    | `422`  | `{ "error": "Invalid body JSON" }`          |
| 11  | Body exceeds the 64kb limit                             | `422`  | `{ "error": "Invalid body JSON" }`          |
| 12  | Body uses `operation: Filter`                           | `422`  | `{ "error": "Invalid body JSON" }`          |
| 13  | Chaos error or unhandled failure                        | `500`  | `{ "error": "Internal server error" }`      |

> TODO: Case 11, possibly create new msg and use code 413. Case 9 can be 400 and 422 but idk if I care
> TODO: validation precedence to determine which response to return given multiple request faults
