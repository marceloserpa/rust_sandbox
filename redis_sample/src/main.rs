extern crate redis;
use redis::Commands;

fn fetch_an_integer() -> redis::RedisResult<isize> {
    let client = redis::Client::open("redis://172.17.0.2/")?;
    let mut con = client.get_connection()?;

    con.set("my_key", 42)?;
    con.set("my_key1", 43)?;

    let val: isize = con.get("my_key")?;

    Ok(val)
}


fn main() {
    match fetch_an_integer() {
        Err(_err) => {
            println!("Could not execute example:");
        }
        Ok(_my_number) => {
            println!("My Number {}", _my_number.to_string());
        }
    }
}