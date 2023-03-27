PRAGMA ignore_check_constraints = 1;
INSERT INTO identities
VALUES (
        '018719d1-9343-738a-b5db-8136bb2fd4f3',
        'demodata',
        'demodata@resippies.com',
        '$6$rounds=10000$IHfw8SPEGD8J8XCh$u4CnVE9zeNVMiv.e.V.YRyByZbOOCFJXrigcbktlBS32dKxh7WhW0deQWcXiy8Ci.H156qcybFeijyImSDKxq0',
        '01871cb3-9593-7f75-be8a-dab878a99f91',
        0
    );
INSERT INTO cookbooks
VALUES (
        '018719d8-e2ec-725b-b455-30ae2d1a3faf',
        '018719d1-9343-738a-b5db-8136bb2fd4f3',
        'Demo Cookbook',
        'This is a demo cookbook.',
        'public'
    );
INSERT INTO cookbooks_contributors
VALUES (
        '018719d8-e2ec-725b-b455-30ae2d1a3faf',
        '018719d1-9343-738a-b5db-8136bb2fd4f3'
    );
INSERT INTO recipes (id, author, name, description, visibility)
VALUES (
        '018719d6-d1b7-7cf1-95db-a741160e6305',
        '018719d1-9343-738a-b5db-8136bb2fd4f3',
        'Grammys Pancakes',
        "If there's one thing that brings back happy memories of lazy weekend mornings with my grandmother, it's her famous pancake recipe. She used to say that the secret to making the perfect pancakes was in the batter, which had to be just the right consistency - not too thick, not too thin - and allowed to rest for a few minutes before cooking. Once the batter was ready, she would heat up a griddle and drop spoonfuls of it onto the hot surface, watching as the pancakes began to bubble and rise. The result was a stack of fluffy, golden pancakes that were simply irresistible. I've since learned that my grandmother's pancake recipe is actually an old family recipe that has been passed down through the generations. It's a simple recipe that uses basic ingredients like flour, sugar, milk, and eggs, but the magic is in the technique. With a little bit of patience and practice, anyone can make these delicious pancakes and enjoy a taste of family history with every bite.",
        'public'
    );
INSERT INTO recipes_contributors
VALUES (
        '018719d6-d1b7-7cf1-95db-a741160e6305',
        '018719d1-9343-738a-b5db-8136bb2fd4f3'
    );
INSERT INTO items
VALUES (
        '018719d8-dc24-749d-8950-8bb8aea0bb71',
        'Flour',
        'All purpose or bleached or unbleached.'
    ),
    (
        '018719d8-dc80-787d-aa1e-f7d0d4c6e54a',
        'Eggs',
        'Large brown'
    ),
    (
        '018719d8-dcdb-7f61-ae5d-ed11761ded18',
        'Milk',
        'Whole or 2%'
    ),
    (
        '018719d8-dd36-7fda-bdfa-39a8e60599f5',
        'Sugar',
        'Granulated white or unbleached'
    ),
    (
        '018719d8-dd92-7e21-8125-a8c5b9e2998e',
        'Baking Powder',
        'Clapper girl or generic'
    ),
    (
        '018719d8-dded-7515-89e4-46779e3d2d7f',
        'Salt',
        ''
    ),
    (
        '018719d8-de4a-7306-a20d-79da8771f1f7',
        'Butter',
        'I prefer Kerrygold, but any butter will do.'
    );
INSERT INTO ingredients
VALUES (
        '018719d8-dea5-7e59-8016-46f5451bb508',
        '018719d8-dc24-749d-8950-8bb8aea0bb71',
        1.5,
        'c'
    ),
    (
        '018719d8-deff-7aba-ba31-aee5cf29a3c9',
        '018719d8-dc80-787d-aa1e-f7d0d4c6e54a',
        1,
        'ct'
    ),
    (
        '018719d8-df5b-7890-9385-b94f130acbe9',
        '018719d8-dcdb-7f61-ae5d-ed11761ded18',
        1.25,
        'c'
    ),
    (
        '018719d8-dfb7-7e32-9111-8049b9de50c8',
        '018719d8-dd36-7fda-bdfa-39a8e60599f5',
        1,
        'tbsp'
    ),
    (
        '018719d8-e012-7045-a460-1d8437a01242',
        '018719d8-dd92-7e21-8125-a8c5b9e2998e',
        3.5,
        'tsp'
    ),
    (
        '018719d8-e06c-7047-b70d-eb3cc72ce8e1',
        '018719d8-dded-7515-89e4-46779e3d2d7f',
        0.25,
        'tsp'
    ),
    (
        '018719d8-e0c9-7e2e-ab0f-837ce783e62f',
        '018719d8-de4a-7306-a20d-79da8771f1f7',
        3,
        'tbsp'
    );
INSERT INTO recipes_ingredients
VALUES (
        '018719d6-d1b7-7cf1-95db-a741160e6305',
        '018719d8-dea5-7e59-8016-46f5451bb508'
    ),
    (
        '018719d6-d1b7-7cf1-95db-a741160e6305',
        '018719d8-deff-7aba-ba31-aee5cf29a3c9'
    ),
    (
        '018719d6-d1b7-7cf1-95db-a741160e6305',
        '018719d8-df5b-7890-9385-b94f130acbe9'
    ),
    (
        '018719d6-d1b7-7cf1-95db-a741160e6305',
        '018719d8-dfb7-7e32-9111-8049b9de50c8'
    ),
    (
        '018719d6-d1b7-7cf1-95db-a741160e6305',
        '018719d8-e012-7045-a460-1d8437a01242'
    ),
    (
        '018719d6-d1b7-7cf1-95db-a741160e6305',
        '018719d8-e06c-7047-b70d-eb3cc72ce8e1'
    ),
    (
        '018719d6-d1b7-7cf1-95db-a741160e6305',
        '018719d8-e0c9-7e2e-ab0f-837ce783e62f'
    );
INSERT INTO steps
VALUES (
        '018719d8-e0c9-7e2e-ab0f-837ce783e62f',
        'Dry Ingredients',
        'Sift the dry ingredients together'
    ),
    (
        '018719d8-e17f-78de-a440-91c66c1fd07d',
        'Wet Ingredients',
        'Make a well, then add the wet ingredients. Stir to combine.'
    ),
    (
        '018719d8-e1db-7451-96f2-67a49f44e6cc',
        'Scoop',
        'Scoop the batter onto a hot griddle or pan.'
    ),
    (
        '018719d8-e235-7147-94c8-20ea4650c122',
        'Cook',
        'Cook for 2-3 minutes, then flip.'
    ),
    (
        '018719d8-e28f-7bfa-9b5f-8fa393e6bd65',
        'Finish Cooking',
        'Continue cooking until brown on both sides.'
    );
INSERT INTO recipes_steps
VALUES (
        '018719d6-d1b7-7cf1-95db-a741160e6305',
        '018719d8-e0c9-7e2e-ab0f-837ce783e62f',
        1
    ),
    (
        '018719d6-d1b7-7cf1-95db-a741160e6305',
        '018719d8-e17f-78de-a440-91c66c1fd07d',
        2
    ),
    (
        '018719d6-d1b7-7cf1-95db-a741160e6305',
        '018719d8-e1db-7451-96f2-67a49f44e6cc',
        3
    ),
    (
        '018719d6-d1b7-7cf1-95db-a741160e6305',
        '018719d8-e235-7147-94c8-20ea4650c122',
        4
    ),
    (
        '018719d6-d1b7-7cf1-95db-a741160e6305',
        '018719d8-e28f-7bfa-9b5f-8fa393e6bd65',
        5
    );
INSERT INTO cookbooks_recipes
VALUES (
        '018719d8-e2ec-725b-b455-30ae2d1a3faf',
        '018719d6-d1b7-7cf1-95db-a741160e6305'
    );
PRAGMA ignore_check_constraints = 1;