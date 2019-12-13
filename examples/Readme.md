
Eventually the plan is to do sled as well...

Don't mess with the Redis data for now...
Focus on sled and tantivy...

#### Next steps

* Step One :

Get sled up and running... Assuming the data is in Redis I read
the data out of Redis and instead of writing it to a file I
write it to sled...

* Step Two :

Read the data out of sled and write it back to Redis.

* Goal here is to possibly use both Sled Files and Redis files
as alternative file storage places besides the individual JSON
files that have data as well.

#### Summary

* In summary I have the data coming out of the Hn API and writing
the json to redis.
* Then I read the data out of redis and write it to a file...  This is
where I can keep long term storage of the data and it will store nicely
in a github repo
* Then I have the ability to take the long term (file) storage and put
it back in redis.

##### file-to-redis.rs

This started out
[here](https://github.com/stormasm/rust-examples/blob/master/lifetimes/examples/readfile.rs) and then I added in the part about saving it to redis.

##### redis-to-file.rs

Pulls all of the ids from a Redis Hashmap
sorts them
Goes back to Redis and gets the values of the associated keys
and writes the id and the Json to a file on a separate line

##### top_n_items_redis.rs

Figure out one of three start ids based on a pseudo switch
See the code comments for more details
Generate the ids based on that starting id
Go out to Hackernews and grab the Json for the associated id
Only write out to Redis the Json you get if its a story.
