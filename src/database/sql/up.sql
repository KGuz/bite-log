CREATE TABLE bites (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    calories INTEGER NOT NULL,
    category INTEGER DEFAULT NULL,
    nutritions INTEGER DEFAULT NULL,
    date TEXT NOT NULL,
    time TEXT NOT NULL
)