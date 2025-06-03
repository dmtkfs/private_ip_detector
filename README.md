# Private IP Detector

### Description

**Private IP Detector** is a simple Rust application that connects to a PostgreSQL database and checks if the source and destination IP addresses from the `connection_log` table are private or public. It helps in quickly identifying private IPs (like those from local networks) and distinguishing them from public ones.

This tool is intended to be used by network administrators, system engineers, or anyone needing to identify IP addresses that are private or public within their database logs.

### Features:

* Connects to PostgreSQL using `tokio-postgres`.
* Queries `connection_log` table for source and destination IPs.
* Checks whether IPs belong to private address ranges:

  * **10.0.0.0/8** (Class A)
  * **172.16.0.0/12** (Class B)
  * **192.168.0.0/16** (Class C)
* Output whether each IP (source and destination) is private or public.

---

### Installation

#### 1. Clone the repository

```bash
git clone https://github.com/yourusername/private_ip_detector.git
cd private_ip_detector
```

#### 2. Set up environment variables

Create a `.env` file in the root of your project with the following content:

```
DB_HOST=localhost
DB_USER=postgres
DB_PASSWORD=password
DB_NAME=postgres
```

*Adjust the values based on your environment, especially the `DB_PASSWORD`, `DB_HOST` and `DB_NAME`.*

#### 3. Install dependencies

Make sure you have [Rust](https://www.rust-lang.org/learn/get-started) installed. Then run:

```bash
cargo build
```

This will install all required dependencies and build the project.

---

### Usage

To run the program, simply use the following command:

```bash
cargo run
```

This will connect to your PostgreSQL database, run the query on the `connection_log` table, and display whether each source and destination IP is private or public.

### Example Output

```plaintext
Checking Source IP: 192.168.1.1
Source IP is Private
Checking Destination IP: 8.8.8.8
Destination IP is Public
```

---

### Testing

You can test the core functionality using the provided tests in `tests.rs`. To run the tests:

```bash
cargo test
```

#### Example test cases:

* **Private IPs**:

  * `192.168.1.1` → Private
  * `10.0.0.1` → Private
  * `172.16.0.1` → Private

* **Public IP**:

  * `8.8.8.8` → Public

---

### Integration with PostgreSQL

To use this in a live system, ensure your database has the following table:

```sql
CREATE TABLE connection_log (
    id serial PRIMARY KEY,
    src_ip inet NOT NULL,
    dst_ip inet NOT NULL,
    ts timestamp with time zone
);
```

You can insert sample data as follows:

```sql
INSERT INTO connection_log (src_ip, dst_ip, ts)
VALUES
    ('192.168.1.1', '8.8.8.8', now());
```

The program will automatically query this table to check IPs for whether they are private or public.

---

### License

This project is licensed under the MIT License – see the [LICENSE](./LICENSE) file for details.

---

### Additional Notes

* You can customize the IP ranges in the `is_private()` function.
* The program relies on the `dotenv` crate to manage database credentials, which makes it easy to keep sensitive data out of the code.
