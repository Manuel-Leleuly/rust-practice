#[cfg(test)]
mod tests {
    use std::{collections::HashMap, num::NonZero, time::Duration};

    use futures::StreamExt;
    use redis::{
        aio::{MultiplexedConnection, PubSub},
        geo::{RadiusOptions, Unit},
        pipe,
        streams::{StreamReadOptions, StreamReadReply},
        AsyncCommands, Client, Commands, RedisError, Value,
    };
    use tokio::time::sleep;

    #[test]
    fn test_connection() {
        let mut client = Client::open("redis://localhost:6379").unwrap();

        let _: () = client.set("name", "Manuel").unwrap();
        let value: String = client.get("name").unwrap();

        println!("{}", value);
    }

    async fn get_client() -> Result<MultiplexedConnection, RedisError> {
        let client = Client::open("redis://localhost:6379")?;
        client.get_multiplexed_async_connection().await
    }

    #[tokio::test]
    async fn test_async_connection() -> Result<(), RedisError> {
        let mut connection = get_client().await?;
        let _: () = connection.set("name", "Manuel").await?;
        let value: String = connection.get("name").await?;

        println!("{}", value);

        Ok(())
    }

    #[tokio::test]
    async fn test_string() -> Result<(), RedisError> {
        let mut connection = get_client().await?;

        let _: () = connection.del("name").await?;

        let _: () = connection.set_ex("name", "Manuel", 2).await?;
        let value: String = connection.get("name").await?;
        println!("{}", value);

        sleep(Duration::from_secs(5)).await;

        let value: Result<String, RedisError> = connection.get("name").await;
        assert_eq!(true, value.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_list() -> Result<(), RedisError> {
        let mut connection = get_client().await?;

        let _: () = connection.del("names").await?;
        let _: () = connection.rpush("names", "Manuel").await?;
        let _: () = connection.rpush("names", "Theodore").await?;
        let _: () = connection.rpush("names", "Leleuly").await?;

        let len: i32 = connection.llen("names").await?;
        assert_eq!(3, len);

        let names: Vec<String> = connection.lrange("names", 0, -1).await?;
        assert_eq!(vec!["Manuel", "Theodore", "Leleuly"], names);

        let name: Vec<String> = connection.lpop("names", NonZero::new(1)).await?;
        assert_eq!(vec!["Manuel"], name);
        let name: Vec<String> = connection.lpop("names", NonZero::new(1)).await?;
        assert_eq!(vec!["Theodore"], name);
        let name: Vec<String> = connection.lpop("names", NonZero::new(1)).await?;
        assert_eq!(vec!["Leleuly"], name);

        Ok(())
    }

    #[tokio::test]
    async fn test_set() -> Result<(), RedisError> {
        let mut connection = get_client().await?;

        let _: () = connection.del("names").await?;
        let _: () = connection.sadd("names", "Manuel").await?;
        let _: () = connection.sadd("names", "Manuel").await?;
        let _: () = connection.sadd("names", "Theodore").await?;
        let _: () = connection.sadd("names", "Theodore").await?;
        let _: () = connection.sadd("names", "Leleuly").await?;
        let _: () = connection.sadd("names", "Leleuly").await?;

        let len: i32 = connection.scard("names").await?;
        assert_eq!(3, len);

        let names: Vec<String> = connection.smembers("names").await?;
        assert_eq!(vec!["Manuel", "Theodore", "Leleuly"], names);

        Ok(())
    }

    #[tokio::test]
    async fn test_sorted_set() -> Result<(), RedisError> {
        let mut connection = get_client().await?;

        let _: () = connection.del("names").await?;
        let _: () = connection.zadd("names", "Manuel", 100).await?;
        let _: () = connection.zadd("names", "Theodore", 85).await?;
        let _: () = connection.zadd("names", "Leleuly", 95).await?;

        let len: i32 = connection.zcard("names").await?;
        assert_eq!(3, len);

        let names: Vec<String> = connection.zrange("names", 0, -1).await?;
        assert_eq!(vec!["Theodore", "Leleuly", "Manuel"], names);

        Ok(())
    }

    #[tokio::test]
    async fn test_hash() -> Result<(), RedisError> {
        let mut connection = get_client().await?;

        let _: () = connection.del("user:1").await?;
        let _: () = connection.hset("user:1", "id", "1").await?;
        let _: () = connection.hset("user:1", "name", "Manuel").await?;
        let _: () = connection
            .hset("user:1", "email", "manuel@gmail.com")
            .await?;

        let user: HashMap<String, String> = connection.hgetall("user:1").await?;
        assert_eq!("1", user.get("id").unwrap());
        assert_eq!("Manuel", user.get("name").unwrap());
        assert_eq!("manuel@gmail.com", user.get("email").unwrap());

        Ok(())
    }

    #[tokio::test]
    async fn test_geo_point() -> Result<(), RedisError> {
        let mut connection = get_client().await?;

        let _: () = connection.del("sellers").await?;
        let _: () = connection
            .geo_add("sellers", (106.822702, -6.177590, "Toko A"))
            .await?;
        let _: () = connection
            .geo_add("sellers", (106.820889, -6.174964, "Toko B"))
            .await?;

        let distance: f64 = connection
            .geo_dist("sellers", "Toko A", "Toko B", Unit::Kilometers)
            .await?;
        assert_eq!(0.3543, distance);

        let result: Vec<String> = connection
            .geo_radius(
                "sellers",
                106.821825,
                -6.175105,
                5.0,
                Unit::Kilometers,
                RadiusOptions::default(),
            )
            .await?;
        assert_eq!(vec!["Toko A", "Toko B"], result);

        Ok(())
    }

    #[tokio::test]
    async fn test_hyper_log_log() -> Result<(), RedisError> {
        let mut connection = get_client().await?;

        let _: () = connection.del("visitors").await?;
        let _: () = connection
            .pfadd("visitors", ("manuel", "theodore", "leleuly"))
            .await?;
        let _: () = connection
            .pfadd("visitors", ("eko", "kurniawan", "khannedy"))
            .await?;
        let _: () = connection
            .pfadd("visitors", ("eko", "budi", "joko"))
            .await?;

        let total: i32 = connection.pfcount("visitors").await?;
        assert_eq!(8, total);

        Ok(())
    }

    #[tokio::test]
    async fn test_pipeline() -> Result<(), RedisError> {
        let mut connection = get_client().await?;

        pipe()
            .set_ex("name", "Manuel", 2)
            .set_ex("address", "Indonesia", 2)
            .exec_async(&mut connection)
            .await?;

        let name: String = connection.get("name").await?;
        assert_eq!("Manuel", name);

        let address: String = connection.get("address").await?;
        assert_eq!("Indonesia", address);

        Ok(())
    }

    #[tokio::test]
    async fn test_transaction() -> Result<(), RedisError> {
        let mut connection = get_client().await?;

        pipe()
            .atomic()
            .set_ex("name", "Manuel", 2)
            .set_ex("address", "Indonesia", 2)
            .exec_async(&mut connection)
            .await?;

        let name: String = connection.get("name").await?;
        assert_eq!("Manuel", name);

        let address: String = connection.get("address").await?;
        assert_eq!("Indonesia", address);

        Ok(())
    }

    #[tokio::test]
    async fn test_publish_stream() -> Result<(), RedisError> {
        let mut connection = get_client().await?;

        for i in 0..10 {
            let mut map = HashMap::<&str, String>::new();
            map.insert("name", format!("Manuel {}", i));
            map.insert("address", String::from("Indonesia"));

            let _: () = connection.xadd_map("members", "*", &map).await?;
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_create_consumer() -> Result<(), RedisError> {
        let mut connection = get_client().await?;

        let _: () = connection.xgroup_create("members", "group-1", "0").await?;
        let _: () = connection
            .xgroup_createconsumer("members", "group-1", "consumer-1")
            .await?;
        let _: () = connection
            .xgroup_createconsumer("members", "group-1", "consumer-2")
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_get_stream() -> Result<(), RedisError> {
        let mut connection = get_client().await?;

        let setting = StreamReadOptions::default()
            .group("group-1", "consumer-1")
            .count(5)
            .block(3000);
        let result: StreamReadReply = connection
            .xread_options(&["members"], &[">"], &setting)
            .await?;

        for key in result.keys {
            for item in key.ids {
                let map: HashMap<String, Value> = item.map;
                let name: String = match map.get("name").unwrap() {
                    Value::BulkString(value) => String::from_utf8(value.to_vec())?,
                    _ => String::from(""),
                };
                let address: String = match map.get("address").unwrap() {
                    Value::BulkString(value) => String::from_utf8(value.to_vec())?,
                    _ => String::from(""),
                };
                println!("{:?}", name);
                println!("{:?}", address);
            }
        }

        Ok(())
    }

    async fn get_client_pubsub() -> Result<PubSub, RedisError> {
        let client = Client::open("redis://localhost:6379/")?;
        client.get_async_pubsub().await
    }

    #[tokio::test]
    async fn test_subscribe() -> Result<(), RedisError> {
        let mut connection = get_client_pubsub().await?;

        let _: () = connection.subscribe("members").await?;
        let mut pubsub_stream = connection.on_message();

        let message: String = pubsub_stream.next().await.unwrap().get_payload()?;
        println!("{}", message);

        Ok(())
    }

    #[tokio::test]
    async fn test_publish() -> Result<(), RedisError> {
        let mut connection = get_client().await?;

        let _: () = connection.publish("members", "Hello PubSub").await?;

        Ok(())
    }
}
