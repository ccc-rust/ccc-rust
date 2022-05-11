# rust

* https://news.ycombinator.com/item?id=19271953

In Rust release builds the integer overflow of checks are disabled. They're not zero cost and there's no magical way to do it, so it's only done in debug builds.
You can disable them in debug builds as well with wrapping integers. So this is again a practice of making the safer option the default, but won't incur runtime costs if it bothers you.

If there is a way to do it at compile time that will usually be used in rust.

You only pay for bounds checks if you use them, same with the overflow stuff, or option types. That's not in contradiction to this principle.

Rust is about zero cost abstraction. Bound checks can be disabled.

* [Does Rust's array bounds checking affect performance?](https://stackoverflow.com/questions/47542438/does-rusts-array-bounds-checking-affect-performance)
