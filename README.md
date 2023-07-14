# serde_builder
An add-on to serde that aims to provide an alternative to derive using a builder pattern instead

At the time this is a proof of concept. To see how the usage looks, run `simple_de` and `simple_ser` examples and look at their code. Additionally, you can check the documentation on docs.rs to see the api fully

Theoretically, deserialization may fail due to wrong field names being provided to deserializer. If this happens, enable `leaking` feature, which supplies correct field names at expense of leaking memory through `Box::leak`

### TODO list:
- Enum de/serialization
- Transformations along with checks
- Defaults dependent on other values
- Become on-par in customizability with serde derive
