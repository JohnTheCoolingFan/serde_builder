# serde_builder
An add-on to serde that aims to provide an alternative to derive using a builder pattern instead

At the time this is a proof of concept. To see how the usage looks, run `simple_case` example and looks at its code. It doesn't use all the functionality, but I hastily put it together to test if this even works.

Right now it only supports deserialization of 3-field structs (due to only manual implementation, macros will be later), and validator functionality is ignored.
