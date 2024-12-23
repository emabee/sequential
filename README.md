
`sequential` is a tiny library that provides a Sequence implementation that

* produces monotonously increasing integer numbers, starting from a configurable starting point and with a configurable increment
* can be fast-forwarded to skip numbers, but cannot be wound back
* stops producing values when the limit of the chosen type T is reached
* optionally (with feature serde) implements `serde::ser::Serialize` and `serde::de::Deserialize`.
* works with all unsigned integers, from u8 to u128.
