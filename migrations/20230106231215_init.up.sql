-- Add up migration script here
-- Add up migration script here
CREATE TABLE
    identities (
        name TEXT NOT NULL PRIMARY KEY,
        -- phone TEXT, -- Phone is TODO
        email TEXT,
        code TEXT NOT NULL, -- code is a verification code used to activate a user
        verified INTEGER DEFAULT 0
    );

CREATE TABLE
    users (
        name TEXT NOT NULL PRIMARY KEY,
        password TEXT NOT NULL,
        --
        FOREIGN KEY (name) REFERENCES identities (name)
    );

CREATE TABLE
    recipes (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        author TEXT NOT NULL,
        name TEXT NOT NULL,
        description TEXT,
        --
        FOREIGN KEY (author) REFERENCES users (id)
    );

-- Item is an ingredient with no defined quantity, it becomes an ingredient
-- when a quantity is attached
CREATE TABLE
    items (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        description TEXT
    );

CREATE TABLE
    ingredients (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        item INTEGER NOT NULL,
        quantity INTEGER NOT NULL,
        unit TEXT NOT NULL,
        --
        FOREIGN KEY (item) REFERENCES items (id),
        UNIQUE (item, quantity, unit)
    );

CREATE TABLE
    steps (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        num INTEGER NOT NULL,
        name TEXT,
        description TEXT NOT NULL
    );

CREATE TABLE
    cookbooks (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        author TEXT NOT NULL,
        name TEXT,
        --
        FOREIGN KEY (author) REFERENCES users (id)
    );

CREATE TABLE
    recipes_ingredients (
        recipe INTEGER NOT NULL,
        ingredient INTEGER NOT NULL,
        PRIMARY KEY (recipe, ingredient),
        FOREIGN KEY (recipe) REFERENCES recipes (id),
        FOREIGN KEY (ingredient) REFERENCES ingredients (id)
    );

CREATE TABLE
    recipes_steps (
        recipe INTEGER NOT NULL,
        step INTEGER NOT NULL,
        PRIMARY KEY (recipe, step),
        FOREIGN KEY (recipe) REFERENCES recipes (id),
        FOREIGN KEY (step) REFERENCES steps (id)
    );

CREATE TABLE
    cookbooks_recipes (
        cookbook INTEGER NOT NULL,
        recipe INTEGER NOT NULL,
        PRIMARY KEY (cookbook, recipe),
        FOREIGN KEY (cookbook) REFERENCES cookbooks (id),
        FOREIGN KEY (recipe) REFERENCES recipes (id)
    );