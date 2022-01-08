# CLIBRI

The design of any network application should start by describing messages sent between the consumer (client) and the producer (server). As better messages would be described, then more predictable would be an application in general.

**CLIBRI** allows describing messages in a strongly typed way to prevent any possible error related to interpretation and **describe logic** of communication between consumer and producer.

Using **CLIBRI** you will get your client-server (consumer-producer) solution just in **4 steps**:

* **Step 1**. Describe protocol (messages, events, broadcasts)
* **Step 2**. Describe workflow (relations between messages, events, broadcasts)
* **Step 3**. Generate code-base with **CLIBRI**
* **Step 4**. Use an almost ready solution (just put your code into handlers and run it)

**CLIBRI** allows you to build stable network solutions really quickly and extend/scale them without huge additional efforts.

Besides, a protocol description and a workflow description together make your solution self-explainable. It would be enough to take a look into the workflow file (as usual a couple of hundreds of lines) to completely understand, what your application does and how it does.

You can combine platform and realizations: server (producer) on rust and client (consumer) on typescript. Or both on rust; or both on typescript. Considering **CLIBRI** generates a very similar API for all realizations it's always easy to work with any realization.

See API documentation: [here](http://clibri.net/)