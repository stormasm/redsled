
#### Summary

* In summary I have the data coming out of the Hn API and writing
the json to redis.  
* Then I read the data out of redis and write it to a file and sled...  

There is a
[hackernews-story-archive](https://github.com/stormasm/hackernews-story-archive)
of the data.

##### top_n_items_redis.rs

Figure out one of three start ids based on a pseudo switch  
See the code comments for more details  
Generate the ids based on that starting id  
Go out to Hackernews and grab the Json for the associated id  
Only write out to Redis the Json you get if its a story.  

##### redis-to-file.rs

Pulls all of the ids from a Redis Hashmap  
sorts them  
Goes back to Redis and gets the values of the associated keys which is json  
and writes these 3 different types of files.

These files get created by slightly modifying the code.

 * json files
 * txt files with json data on each line
 * txt files with the hackernews id and json on separate lines

The **text file** with json data on each line can be huge and streamed out over time and then processed.

Eventually the plan is to have a text file for each individual year.

The **json files** are good for viewing in a browser a subset of the much
bigger text file above.  Simply break off a section off the big text
file and create a much smaller json file.

##### file-to-redis.rs

This started out
[here](https://github.com/stormasm/rust-examples/blob/master/lifetimes/examples/readfile.rs) and then I added in the part about saving it to redis.
