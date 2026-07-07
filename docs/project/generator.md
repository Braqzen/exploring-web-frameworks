# Generator

Broadly speaking the generator consists of 3 components

1. An infinite loop putting together all the pieces to send requests
2. Selection mechanisms for providers and HTTP methods
3. The randomiser changing internal state to make the simulation interesting

In the background the randomiser changes distributions affecting selection mechanisms however to prevent a pointless state we fall back to `POST`ing if a selected provider has no current tasks.

The general workflow is:

1. Load config and initialise systems
2. Start the loop
   1. Select a provider
   2. Select a HTTP method to send to them
   3. Call the function that handles that logic
   4. Handle telemetry, send request and deal with response
   5. Restart loop

The generator maintains an in-memory state of its tasks.
