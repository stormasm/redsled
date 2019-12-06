
Eventually the plan is to do sled as well...

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
