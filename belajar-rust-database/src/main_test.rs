#[cfg(test)]
mod tests {
    use chrono::Local;
    use futures::TryStreamExt;
    use std::time::Duration;
    use tokio::test;

    use sqlx::{
        postgres::{PgPoolOptions, PgRow},
        query, query_as, Connection, Error, PgConnection, Pool, Postgres, Row,
    };

    use crate::model::{Brand, Category};

    #[test]
    async fn test_manual_connection() -> Result<(), Error> {
        let url = "postgres://postgres:root@localhost:5432/belajar-rust-database";
        let connection = PgConnection::connect(&url).await?;

        connection.close().await?;
        Ok(())
    }

    async fn get_pool() -> Result<Pool<Postgres>, Error> {
        let url = "postgres://postgres:root@localhost:5432/belajar-rust-database";
        PgPoolOptions::new()
            .max_connections(10)
            .min_connections(5)
            .acquire_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(60))
            .connect(url)
            .await
    }

    #[test]
    async fn test_pool_connection() -> Result<(), Error> {
        let pool = get_pool().await?;
        pool.close().await;
        Ok(())
    }

    #[test]
    async fn test_execute() -> Result<(), Error> {
        let pool = get_pool().await?;
        query("insert into category(id, name, description) values('A', 'Contoh', 'Contoh')")
            .execute(&pool)
            .await?;
        Ok(())
    }

    #[test]
    async fn test_prepare_statement() -> Result<(), Error> {
        let pool = get_pool().await?;
        query("insert into category(id, name, description) values($1, $2, $3)")
            .bind("B")
            .bind("Contoh")
            .bind("Contoh Deskripsi")
            .execute(&pool)
            .await?;
        Ok(())
    }

    #[test]
    async fn test_fetch_optional() -> Result<(), Error> {
        let pool = get_pool().await?;
        let result = query("select * from category where id = $1")
            .bind("A")
            .fetch_optional(&pool)
            .await?;

        if let Some(row) = result {
            let id: String = row.get("id");
            let name: String = row.get("name");
            let description: String = row.get("description");
            println!("id: {}, name: {}, description: {}", id, name, description);
        } else {
            println!("Data not found");
        }

        Ok(())
    }

    #[test]
    async fn test_fetch_one() -> Result<(), Error> {
        let pool = get_pool().await?;
        let result = query("select * from category where id = $1")
            .bind("A")
            .fetch_one(&pool)
            .await?;

        let id: String = result.get("id");
        let name: String = result.get("name");
        let description: String = result.get("description");
        println!("id: {}, name: {}, description: {}", id, name, description);

        Ok(())
    }

    #[test]
    async fn test_fetch_all() -> Result<(), Error> {
        let pool = get_pool().await?;
        let result = query("select * from category").fetch_all(&pool).await?;

        for row in result {
            let id: String = row.get("id");
            let name: String = row.get("name");
            let description: String = row.get("description");
            println!("id: {}, name: {}, description: {}", id, name, description);
        }

        Ok(())
    }

    #[test]
    async fn test_fetch_stream() -> Result<(), Error> {
        let pool = get_pool().await?;
        let mut result = query("select * from category").fetch(&pool);

        while let Some(row) = result.try_next().await? {
            let id: String = row.get("id");
            let name: String = row.get("name");
            let description: String = row.get("description");
            println!("id: {}, name: {}, description: {}", id, name, description);
        }

        Ok(())
    }

    #[test]
    async fn test_result_mapping() -> Result<(), Error> {
        let pool = get_pool().await?;
        let result = query("select * from category")
            .map(|row: PgRow| Category {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
            })
            .fetch_all(&pool)
            .await?;

        for category in result {
            println!("{:?}", category);
        }

        Ok(())
    }

    #[test]
    async fn test_result_mapping_auto() -> Result<(), Error> {
        let pool = get_pool().await?;
        let result: Vec<Category> = query_as("select * from category").fetch_all(&pool).await?;

        for category in result {
            println!("{:?}", category);
        }

        Ok(())
    }

    #[test]
    async fn test_insert_brand() -> Result<(), Error> {
        let pool = get_pool().await?;
        query("insert into brands(id, name, description, created_at, updated_at) values($1, $2, $3, $4, $5)")
            .bind("A")
            .bind("Contoh")
            .bind("Contoh Deskripsi")
            .bind(Local::now().naive_local())
            .bind(Local::now().naive_local())
            .execute(&pool).await?;

        Ok(())
    }

    #[test]
    async fn test_result_mapping_brand() -> Result<(), Error> {
        let pool = get_pool().await?;
        let result: Vec<Brand> = query_as("select * from brands").fetch_all(&pool).await?;

        for category in result {
            println!("{:?}", category);
        }

        Ok(())
    }

    #[test]
    async fn test_transaction() -> Result<(), Error> {
        let pool = get_pool().await?;
        let mut transaction = pool.begin().await?;

        query("insert into brands(id, name, description, created_at, updated_at) values($1, $2, $3, $4, $5)")
            .bind("B")
            .bind("Contoh")
            .bind("Contoh Deskripsi")
            .bind(Local::now().naive_local())
            .bind(Local::now().naive_local())
            .execute(&mut *transaction).await?;

        query("insert into brands(id, name, description, created_at, updated_at) values($1, $2, $3, $4, $5)")
            .bind("C")
            .bind("Contoh")
            .bind("Contoh Deskripsi")
            .bind(Local::now().naive_local())
            .bind(Local::now().naive_local())
            .execute(&mut *transaction).await?;

        query("insert into brands(id, name, description, created_at, updated_at) values($1, $2, $3, $4, $5)")
            .bind("D")
            .bind("Contoh")
            .bind("Contoh Deskripsi")
            .bind(Local::now().naive_local())
            .bind(Local::now().naive_local())
            .execute(&mut *transaction).await?;

        transaction.commit().await?;
        Ok(())
    }

    #[test]
    async fn test_auto_increment() -> Result<(), Error> {
        let pool = get_pool().await?;
        let result = query("insert into sellers(name) values($1) returning id")
            .bind("Contoh Seller")
            .fetch_one(&pool)
            .await?;

        let id: i32 = result.get("id");
        println!("id: {}", id);

        Ok(())
    }

    #[test]
    async fn test_auto_increment_with_transaction() -> Result<(), Error> {
        let pool = get_pool().await?;
        let mut transaction = pool.begin().await?;

        query("insert into sellers(name) values($1) returning id")
            .bind("Contoh seller with transaction")
            .execute(&mut *transaction)
            .await?;

        let result = query("select lastval() as id")
            .fetch_one(&mut *transaction)
            .await?;

        let id: i32 = result.get_unchecked("id");
        println!("id: {}", id);

        transaction.commit().await?;
        Ok(())
    }
}
