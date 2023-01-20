CREATE TABLE
    identities (
        id TEXT PRIMARY KEY NOT NULL, -- Identity UUID
        username TEXT NOT NULL UNIQUE,
        -- phone TEXT, -- TODO: Add phone number
        email TEXT NOT NULL,
        password_hash TEXT NOT NULL,
        code TEXT NOT NULL, -- Code is a verification code used to activate a user
        verified INTEGER DEFAULT 0 -- Whether the user has been activated
    );

CREATE TABLE
    recipes (
        id TEXT PRIMARY KEY NOT NULL,
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
        id TEXT PRIMARY KEY NOT NULL, -- Item UUID
        name TEXT NOT NULL,
        description TEXT
    );

CREATE TABLE
    ingredients (
        id TEXT PRIMARY KEY NOT NULL, -- Ingredient UUID
        item TEXT NOT NULL,
        quantity INTEGER NOT NULL,
        unit TEXT NOT NULL,
        --
        FOREIGN KEY (item) REFERENCES items (id),
        UNIQUE (item, quantity, unit)
    );

CREATE TABLE
    steps (
        id TEXT PRIMARY KEY NOT NULL, -- Step UUID
        name TEXT,
        description TEXT NOT NULL
    );

CREATE TABLE
    cookbooks (
        id TEXT PRIMARY KEY, -- Cookbook UUID
        author TEXT NOT NULL, -- Author UUID
        name TEXT, -- Cookbook name
        --
        FOREIGN KEY (author) REFERENCES users (id)
    );

CREATE TABLE
    recipes_ingredients (
        recipe TEXT NOT NULL, -- Recipe UUID
        ingredient TEXT NOT NULL, -- Ingredient UUID
        --
        PRIMARY KEY (recipe, ingredient),
        FOREIGN KEY (recipe) REFERENCES recipes (id),
        FOREIGN KEY (ingredient) REFERENCES ingredients (id)
    );

CREATE TABLE
    recipes_steps (
        recipe TEXT NOT NULL, -- Recipe UUID
        step TEXT NOT NULL, -- Step UUID
        num INTEGER NOT NULL UNIQUE, -- Step number in the recipe
        --
        PRIMARY KEY (recipe, step),
        FOREIGN KEY (recipe) REFERENCES recipes (id),
        FOREIGN KEY (step) REFERENCES steps (id)
    );

CREATE TABLE
    cookbooks_recipes (
        cookbook TEXT NOT NULL, -- Cookbook UUID
        recipe TEXT NOT NULL, -- Recipe UUID
        --
        PRIMARY KEY (cookbook, recipe),
        FOREIGN KEY (cookbook) REFERENCES cookbooks (id),
        FOREIGN KEY (recipe) REFERENCES recipes (id)
    );