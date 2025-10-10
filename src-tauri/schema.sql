BEGIN;
    CREATE TABLE IF NOT EXISTS patients (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        surname TEXT NOT NULL,
        national_id varchar(20) UNIQUE NOT NULL,
        phone varchar(25) NOT NULL,
        medicare TEXT,
        medicare_number varchar(50),
        sex TEXT CHECK(sex IN ('M', 'F', '-')) NOT NULL,
        gender TEXT,
        description TEXT,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    );

    CREATE TABLE IF NOT EXISTS events (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        title varchar(256) NOT NULL,
        description TEXT,
        datetime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    );

COMMIT;
