# serde_builder
An add-on to serde that aims to provide an alternative to derive using a builder pattern instead

At the time this is a proof of concept. To see how the usage looks, run `simple_case` example and look at its code. It doesn't use all the functionality, I hastily put it together to test if this even works and provide and example of usage.

Theoretically, deserialization may fail due to wrong field names being provided to deserializer. If this happens, enable `leaking` feature, which supplies correct field names at expense of leaking memory through `Box::leak`
