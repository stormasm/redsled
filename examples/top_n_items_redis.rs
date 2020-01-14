use hn_api::types::Item;
use hn_api::HnClient;
use r2d2_redis::{r2d2, RedisConnectionManager};
// use redis::Commands;
use redis::{Commands, RedisResult};

#[allow(dead_code)]
fn get_redis_key(key: String) -> RedisResult<Option<u32>> {
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let pool = pool.clone();
    let mut con = pool.get().unwrap();
    con.get(key)
}

fn write_json_to_redis(key: String, value: String) -> RedisResult<()> {
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let pool = pool.clone();
    let mut con = pool.get().unwrap();

    let _: () = con.set("hn-story-start", &key)?;

    let _x0 = redis::cmd("HSET")
        .arg("hn-story-20")
        .arg(key)
        .arg(value)
        .query::<u64>(&mut *con)
        .unwrap();

    Ok(())
}

fn process_items(api: &HnClient, item_ids: Vec<u32>) {
    for item_id in item_ids {
        let item_json = api.get_json(item_id).unwrap();
        match item_json.as_ref() {
            "null" => println!("{} null", item_id),
            _ => {
                let item: Item = serde_json::from_str(&item_json).unwrap();
                let item_type = item.item_type();
                match item_type.as_ref() {
                    "story" => {
                        println!("{} story", item_id);
                        let _ = write_json_to_redis(item_id.to_string(), item_json);
                    }
                    _ => {}
                }
            }
        }
    }
}

fn top_n_items(numofitems: u32, max_number: u32) -> Vec<u32> {
    let mut items = Vec::new();
    items.push(max_number);
    let mut value = max_number;

    for _ in 1..numofitems {
        value = value - 1;
        items.push(value);
    }
    items
}

fn main() {
    let api = HnClient::init().unwrap();

    // Eventually this will be replaced by the start_switch
    // Our first model will be start_switch_redis
    // In the future it could be start_switch_sled
    // Depending on where you are getting your data from

    let start_id_redis: RedisResult<Option<u32>> = get_redis_key("hn-story-start".to_string());
    let start_id = start_id_redis.unwrap().unwrap();

    // Instead of getting the start_id from redis
    // get it from here
    // let start_id = 21698868;

    // If the above 2 locations are None then grab
    // it from the hackernews api

    // let start_id = api.get_max_item_id().unwrap();

    println!("start id = {}", start_id);

    let item_ids = top_n_items(10000, start_id);
    process_items(&api, item_ids);
}
