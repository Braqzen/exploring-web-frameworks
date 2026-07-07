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

> Note: status codes are inconsistent across frameworks rn
