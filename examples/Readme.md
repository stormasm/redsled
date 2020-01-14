
There are 3 types of files created by [redis-to-file](./redis-to-file.rs).  These files get created by slightly modifying the code.

 * json files
 * txt files with json data on each line
 * txt files with the hackernews id and json on separate lines

#### Summary

* In summary I have the data coming out of the Hn API and writing
the json to redis.
* Then I read the data out of redis and write it to a file and sled...  

There is a
[hackernews-story-archive](https://github.com/stormasm/hackernews-story-archive)
of the data.

##### file-to-redis.rs

This started out
[here](https://github.com/stormasm/rust-examples/blob/master/lifetimes/examples/readfile.rs) and then I added in the part about saving it to redis.

##### redis-to-file.rs

Pulls all of the ids from a Redis Hashmap
sorts them
Goes back to Redis and gets the values of the associated keys which is json
and writes the 3 different types of files noted above.

##### top_n_items_redis.rs

Figure out one of three start ids based on a pseudo switch  
See the code comments for more details  
Generate the ids based on that starting id  
Go out to Hackernews and grab the Json for the associated id  
Only write out to Redis the Json you get if its a story.  
