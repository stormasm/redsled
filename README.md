
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
