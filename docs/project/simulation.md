# Randomisation

To make the simulation interesting the generator and frameworks have built-in randomness.

**Generator**

At a random interval the generator will randomise the % distribution of HTTP methods for each enabled provider.\
This means that for X seconds one provider may receive `POST` requests while another provider may have an equal split between all of its enabled methods.

Similarly, the `Operation`s are randomised however this applies to all enabled providers instead of unique distributions per provider.

Provider selection is also random for each request.

**Frameworks**

For each received request a framework has a chance to sleep to simulate increased workload/latency.\
Moreover, it may return an internal server error to simulate random failures.
