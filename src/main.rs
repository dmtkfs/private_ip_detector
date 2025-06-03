use tokio_postgres::{NoTls, Error};
use ipnet::IpNet; // Needed for network checking
use std::net::IpAddr;
use std::env;
use dotenv::dotenv;  // Add this import

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let db_host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
    let db_user = env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string());
    let db_password = env::var("DB_PASSWORD").unwrap_or_else(|_| "password123".to_string());
    let db_name = env::var("DB_NAME").unwrap_or_else(|_| "postgres".to_string());

    let config = format!(
        "host={} user={} password={} dbname={}",
        db_host, db_user, db_password, db_name
    );

    let (client, connection) = match tokio_postgres::connect(&config, NoTls).await {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            return Err(e);
        }
    };

    tokio::spawn(connection);

    let rows = match client.query("SELECT src_ip, dst_ip FROM connection_log", &[]).await {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("Failed to execute query: {}", e);
            return Err(e);
        }
    };

    for row in rows {
        let src_ip: IpAddr = row.get(0);
        let dst_ip: IpAddr = row.get(1);

        println!("Checking Source IP: {}", src_ip);
        if is_private(&src_ip) {
            println!("Source IP is Private");
        } else {
            println!("Source IP is Public");
        }

        println!("Checking Destination IP: {}", dst_ip);
        if is_private(&dst_ip) {
            println!("Destination IP is Private");
        } else {
            println!("Destination IP is Public");
        }
    }

    Ok(())
}

// Function to check if an IP address is in a private range
fn is_private(ip: &IpAddr) -> bool {
    let private_ranges = [
        ("10.0.0.0", 8),     // Class A
        ("172.16.0.0", 12),   // Class B
        ("192.168.0.0", 16),  // Class C
    ];

    // Check if the IP is in any of the private ranges
    for (range, prefix) in private_ranges.iter() {
        if let Ok(network) = IpNet::new(range.parse().unwrap(), *prefix) {
            if network.contains(ip) {
                return true;
            }
        }
    }
    false
}

