CREATE TABLE IF NOT EXISTS bites (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    calories INTEGER NOT NULL,
    category INTEGER DEFAULT NULL,
    nutritions BLOB DEFAULT NULL,
    date TEXT NOT NULL,
    time TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS profiles (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    height INTEGER NOT NULL,
    weight INTEGER NOT NULL,
    activity INTEGER NOT NULL,
);