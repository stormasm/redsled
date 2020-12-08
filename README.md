
[rediset-to-file](https://github.com/stormasm/redsled/blob/master/examples/rediset-to-file.rs) is used by the repo [hackernews-favorites](https://github.com/stormasm/hackernews-favorites)

[sled-to-file.rs](https://github.com/stormasm/redsled/blob/master/examples/sled-to-file.rs) is a great starting example of lifetimes.

Check out this
[Readme](./examples/Readme.md)
for more details.

This assumes that the data already exists in Redis
and redsled is all about moving the data out of Redis
and / or using Redis for further processing.

Data can be written to files, sled and in the future
other persistent data stores.  Including...

* Tantivy

For now data lands in Redis via
[hn-api-examples](https://github.com/stormasm/hn-api-examples)
and then redsled will take over and do more processing.

Much of the tantivy processing will happen in tansled.

##### References:

[Sort a vector of Integers](https://rust-lang-nursery.github.io/rust-cookbook/algorithms/sorting.html)
