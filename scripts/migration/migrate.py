import os
import sqlite3
import sys


def migrate(original_db_path, data_dir):
    if not os.path.exists(original_db_path):
        print(f"❌ Error: Source database '{original_db_path}' not found.")
        return

    os.makedirs(data_dir, exist_ok=True)
    
    auth_db_path = os.path.join(data_dir, "auth.db")
    firewall_db_path = os.path.join(data_dir, "firewall.db")

    print(f"🔄 Migrating {original_db_path}...")
    print(f"📂 Target Directory: {data_dir}")

    # Connect to databases
    src_conn = sqlite3.connect(original_db_path)
    auth_conn = sqlite3.connect(auth_db_path)
    fire_conn = sqlite3.connect(firewall_db_path)

    try:
        # --- Auth Service Migration (users) ---
        print("👤 Migrating users to auth.db...")
        src_conn.row_factory = sqlite3.Row
        users = src_conn.execute("SELECT * FROM users").fetchall()
        
        auth_conn.execute("CREATE TABLE IF NOT EXISTS users (username TEXT PRIMARY KEY, password_hash TEXT NOT NULL, is_admin BOOLEAN NOT NULL DEFAULT 0)")
        for user in users:
            auth_conn.execute(
                "INSERT OR REPLACE INTO users (username, password_hash, is_admin) VALUES (?, ?, ?)",
                (user['username'], user['password_hash'], user['is_admin'])
            )
        auth_conn.commit()

        # --- Firewall Service Migration ---
        print("🖥️  Migrating servers to firewall.db...")
        fire_conn.execute("""
            CREATE TABLE IF NOT EXISTS servers (
                servername TEXT PRIMARY KEY,
                port INTEGER NOT NULL,
                api_startup_method TEXT,
                api_startup_link TEXT,
                api_startup_token TEXT
            )
        """)
        servers = src_conn.execute("SELECT * FROM servers").fetchall()
        for s in servers:
            s_dict = dict(s)
            fire_conn.execute(
                "INSERT OR REPLACE INTO servers (servername, port, api_startup_method, api_startup_link, api_startup_token) VALUES (?, ?, ?, ?, ?)",
                (s_dict['servername'], s_dict['port'], s_dict.get('api_startup_method'), s_dict.get('api_startup_link'),
                 s_dict.get('api_startup_token'))
            )

        print("📜 Migrating whitelist to firewall.db...")
        fire_conn.execute("""
            CREATE TABLE IF NOT EXISTS whitelist (
                servername TEXT NOT NULL,
                username TEXT NOT NULL,
                ip_address TEXT NOT NULL,
                expiration INTEGER NOT NULL,
                PRIMARY KEY (servername, username, ip_address),
                FOREIGN KEY (servername) REFERENCES servers(servername) ON DELETE CASCADE
            )
        """)
        whitelist = src_conn.execute("SELECT * FROM whitelist").fetchall()
        for w in whitelist:
            fire_conn.execute(
                "INSERT OR REPLACE INTO whitelist (servername, username, ip_address, expiration) VALUES (?, ?, ?, ?)",
                (w['servername'], w['username'], w['ip_address'], w['expiration'])
            )

        print("🔗 Migrating user_server_map to firewall.db...")
        fire_conn.execute("""
            CREATE TABLE IF NOT EXISTS user_server_map (
                username TEXT NOT NULL,
                servername TEXT NOT NULL,
                PRIMARY KEY (username, servername),
                FOREIGN KEY (servername) REFERENCES servers(servername) ON DELETE CASCADE
            )
        """)
        try:
            mappings = src_conn.execute("SELECT * FROM user_server_map").fetchall()
            for m in mappings:
                fire_conn.execute(
                    "INSERT OR REPLACE INTO user_server_map (username, servername) VALUES (?, ?)",
                    (m['username'], m['servername'])
                )
        except sqlite3.OperationalError:
            print("⚠️  No user_server_map table found in source, skipping...")

        fire_conn.commit()
        print("✅ Migration completed successfully.")

    except Exception as e:
        print(f"❌ Error during migration: {e}")
    finally:
        src_conn.close()
        auth_conn.close()
        fire_conn.close()

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: python3 migrate.py <source_db_path> <target_data_dir>")
    else:
        migrate(sys.argv[1], sys.argv[2])
