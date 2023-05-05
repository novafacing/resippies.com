CREATE TABLE identities (
    id TEXT PRIMARY KEY NOT NULL,
    -- Identity UUID
    username TEXT NOT NULL UNIQUE,
    -- phone TEXT, -- TODO: Add phone number
    email TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    code TEXT NOT NULL,
    -- Code is a verification code used to activate a user
    verified INTEGER DEFAULT 0 -- Whether the user has been activated
);
CREATE TABLE recipes (
    id TEXT PRIMARY KEY NOT NULL,
    author TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    visibility TEXT CHECK (visibility IN ('public', 'private')),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (author) REFERENCES identities (id)
);
-- Item is an ingredient with no defined quantity, it becomes an ingredient
-- when a quantity is attached
CREATE TABLE items (
    id TEXT PRIMARY KEY NOT NULL,
    -- Item UUID
    name TEXT NOT NULL,
    description TEXT
);
CREATE TABLE ingredients (
    id TEXT PRIMARY KEY NOT NULL,
    -- Ingredient UUID
    item TEXT NOT NULL,
    quantity REAL NOT NULL,
    unit TEXT NOT NULL,
    FOREIGN KEY (item) REFERENCES items (id)
);
CREATE TABLE steps (
    id TEXT PRIMARY KEY NOT NULL,
    -- Step UUID
    name TEXT,
    description TEXT NOT NULL
);
CREATE TABLE cookbooks (
    id TEXT PRIMARY KEY,
    -- Cookbook UUID
    author TEXT NOT NULL,
    -- Author UUID
    name TEXT,
    description TEXT,
    -- Cookbook name
    visibility TEXT CHECK (visibility IN ('public', 'private')),
    FOREIGN KEY (author) REFERENCES identities (id)
);
CREATE TABLE recipes_ingredients (
    recipe TEXT NOT NULL,
    -- Recipe UUID
    ingredient TEXT NOT NULL,
    -- Ingredient UUID
    PRIMARY KEY (recipe, ingredient),
    FOREIGN KEY (recipe) REFERENCES recipes (id),
    FOREIGN KEY (ingredient) REFERENCES ingredients (id)
);
CREATE TABLE recipes_steps (
    recipe TEXT NOT NULL,
    -- Recipe UUID
    step TEXT NOT NULL,
    -- Step UUID
    num INTEGER NOT NULL,
    -- Step number in the recipe
    PRIMARY KEY (recipe, step),
    FOREIGN KEY (recipe) REFERENCES recipes (id),
    FOREIGN KEY (step) REFERENCES steps (id)
);
CREATE TABLE cookbooks_recipes (
    cookbook TEXT NOT NULL,
    -- Cookbook UUID
    recipe TEXT NOT NULL,
    -- Recipe UUID
    PRIMARY KEY (cookbook, recipe),
    FOREIGN KEY (cookbook) REFERENCES cookbooks (id)
);
CREATE TRIGGER cookbook_recipe_visibility BEFORE
INSERT ON cookbooks_recipes FOR EACH ROW
    WHEN (
        (
            SELECT visibility
            FROM cookbooks
            WHERE id = NEW.cookbook
        ) = 'public'
        OR (
            SELECT visibility
            FROM cookbooks
            WHERE id = NEW.cookbook
        ) = 'private'
        AND (
            SELECT visibility
            FROM recipes
            WHERE id = NEW.recipe
        ) = 'private'
    ) BEGIN
SELECT RAISE(
        ABORT,
        'Recipes in public cookbooks must be public'
    )
WHERE (
        SELECT visibility
        FROM cookbooks
        WHERE id = NEW.cookbook
    ) = 'public'
    AND (
        SELECT visibility
        FROM recipes
        WHERE id = NEW.recipe
    ) = 'private';
END;
CREATE TABLE cookbooks_contributors (
    cookbook TEXT NOT NULL,
    -- Cookbook UUID
    contributor TEXT NOT NULL,
    -- Contributor UUID
    PRIMARY KEY (cookbook, contributor),
    FOREIGN KEY (cookbook) REFERENCES cookbooks (id),
    FOREIGN KEY (contributor) REFERENCES identities (id)
);
CREATE TABLE recipes_contributors (
    recipe TEXT NOT NULL,
    -- Recipe UUID
    contributor TEXT NOT NULL,
    -- Contributor UUID
    PRIMARY KEY (recipe, contributor),
    FOREIGN KEY (recipe) REFERENCES recipes (id),
    FOREIGN KEY (contributor) REFERENCES identities (id)
);