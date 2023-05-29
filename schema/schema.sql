CREATE TABLE user (
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
CREATE TABLE recipe (
    id TEXT PRIMARY KEY NOT NULL,
    author TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    visibility TEXT NOT NULL CHECK (visibility IN ('public', 'private')),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    -- Delete recipe if author is deleted
    FOREIGN KEY (author) REFERENCES user (id) ON DELETE CASCADE
);
CREATE TABLE ingredient (
    id TEXT PRIMARY KEY NOT NULL,
    recipe TEXT NOT NULL,
    -- Ingredient UUID
    name TEXT NOT NULL,
    DESCRIPTION TEXT NOT NULL,
    quantity REAL NOT NULL,
    unit TEXT NOT NULL,
    FOREIGN KEY (recipe) REFERENCES recipe (id) ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE TABLE step (
    id TEXT PRIMARY KEY NOT NULL,
    recipe TEXT NOT NULL,
    -- Step UUID
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    FOREIGN KEY (recipe) REFERENCES recipe (id) ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE TABLE cookbook (
    id TEXT PRIMARY KEY NOT NULL,
    -- Cookbook UUID
    author TEXT NOT NULL,
    -- Author UUID
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    -- Cookbook name
    visibility TEXT NOT NULL CHECK (visibility IN ('public', 'private')),
    -- Delete cookbook if author is deleted
    FOREIGN KEY (author) REFERENCES user (id) ON DELETE CASCADE
);
CREATE TABLE cookbook_recipe (
    cookbook TEXT NOT NULL,
    -- Cookbook UUID
    recipe TEXT NOT NULL,
    -- Recipe UUID
    PRIMARY KEY (cookbook, recipe),
    -- Delete join table entry if cookbook or recipe is deleted
    FOREIGN KEY (cookbook) REFERENCES cookbook (id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (recipe) REFERENCES recipe (id) ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE TRIGGER cookbook_recipe_visibility BEFORE
INSERT ON cookbook_recipe FOR EACH ROW
    WHEN (
        (
            SELECT visibility
            FROM cookbook
            WHERE id = NEW.cookbook
        ) = 'public'
        OR (
            SELECT visibility
            FROM cookbook
            WHERE id = NEW.cookbook
        ) = 'private'
        AND (
            SELECT visibility
            FROM recipe
            WHERE id = NEW.recipe
        ) = 'private'
    ) BEGIN
SELECT RAISE(
        ABORT,
        'recipe in public cookbook must be public'
    )
WHERE (
        SELECT visibility
        FROM cookbook
        WHERE id = NEW.cookbook
    ) = 'public'
    AND (
        SELECT visibility
        FROM recipe
        WHERE id = NEW.recipe
    ) = 'private';
END;
CREATE TABLE cookbook_contributor (
    cookbook TEXT NOT NULL,
    -- Cookbook UUID
    contributor TEXT NOT NULL,
    -- Contributor UUID
    PRIMARY KEY (cookbook, contributor),
    -- Delete join table entry if cookbook or contributor is deleted
    FOREIGN KEY (cookbook) REFERENCES cookbook (id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (contributor) REFERENCES user (id) ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE TABLE recipe_contributor (
    recipe TEXT NOT NULL,
    -- Recipe UUID
    contributor TEXT NOT NULL,
    -- Contributor UUID
    PRIMARY KEY (recipe, contributor),
    -- Delete join table entry if recipe or contributor is deleted
    FOREIGN KEY (recipe) REFERENCES recipe (id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (contributor) REFERENCES user (id) ON DELETE CASCADE ON UPDATE CASCADE
);