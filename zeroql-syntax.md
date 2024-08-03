## CREATE

```surql
CREATE person:alice SET age = 40, company = "Gigamono", weight = 70.5
```

```surql
CREATE person:alice SET {
    age: 40,
    company: "Gigamono",
    weight: 70.5,
}
```

```surql
CREATE person SET (name, age, company, weight) VALUES \
    ("alice", 40, "Gigamono", 70.5),
    ("bob", 30, "Gigamono", 80.5)
```

```surql
CREATE person:* -- Creates a new record with a random ID.
```

```surql
CREATE person -- Creates a new record with a random ID.
```

<!-- --- -->

## RELATE

```surql
RELATE person:alice -> buys -> product:apple
```

```surql
RELATE person:* -> buys -> product:* WHERE person.id = "alice" AND product.id = "apple"
```

```surql
RELATE [person:alice, person:bob] -> buys -> product:apple
```

```surql
RELATE person:alice -> buys -> product:apple SET expiry = time::now() + duration::weeks(2)
```

```surql
RELATE person:alice -> buys -> product:apple SET {
    expiry: time::now() + duration::weeks(2)
}
```

<!-- --- -->

## VARIABLES

```surql
LET $age = 40

CREATE person:alice SET age = $age

SET $age = 20

UPDATE person:alice SET age = $age
```

```surql
LET $token TYPE [u8; 10] = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09"
UPDATE person:alice SET token = $token
```

<!-- --- -->

## TYPES

#### UNSIGNED INTEGERS

```surql
LET $flag TYPE u8 = 0b0000_0001

LET $flag TYPE u16 = 0o0000_0001

LET $flag TYPE u32 = 0x0000_0001

LET $flag TYPE u64 = 1
```

#### SIGNED INTEGERS

```surql
LET $balance TYPE i8 = -40
```

#### FLOATS

```surql
LET $weight TYPE f32 = 70.5

LET $cost TYPE f64 = .75
```

#### BOOLEAN

```surql
LET $is_active TYPE bool = true
```

#### STRING

```surql
LET $name TYPE string = "alice"
```

#### BYTE STRING

```surql
LET $token TYPE [u8; 10] = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09"
```

#### TUPLE

```surql
LET $person TYPE (string, u8) = ("alice", 40)
```

#### REGEX

```surql
LET $name TYPE regex = //a.*//
```

#### OBJECT

```surql
LET $alice TYPE person = {
    name: "alice",
    age: 40,
}
```

#### ARRAY

```surql
LET $ids TYPE [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
```

#### LIST

```surql
LET $cats TYPE [string] = [
    "chubby",
    "fluffy",
    "furry",
    "shaggy",
    "sleek",
    "wooly",
]
```

#### OPTION

```surql
LET $age TYPE u8? = 40
```

<!-- --- -->

## SELECT

#### SELECT ALL

```surql
SELECT * FROM person
```

```surql
person:*
```

```surql
person
```

#### SELECT ONE

```surql
SELECT * FROM person:alice
```

```surql
person:alice
```

#### FIELDS & INDICES

```surql
SELECT main_address.coords FROM person
```

```surql
SELECT addresses[0].coords FROM person
```

#### FOLDING TABLES

```surql
SELECT FOLD distinct(age) FROM person
```

#### SUBQUERIES

```surql
SELECT FOLD sum(age) FROM (SELECT FOLD distinct(age) FROM person)
```

```surql
SELECT (SELECT coords FROM addresses) AS coords FROM person
```

#### RECORD LINKING

```surql
CREATE person:alice SET name = "alice"
CREATE car:tesla SET owner = person:alice

SELECT owner.person.* FROM car:tesla
```

#### FROM RELATION

```surql
SELECT * FROM person -> reacted_to -> post WHERE type = 'celebrate'
```

```surql
SELECT p.name FROM person:tobie -> likes -> (person AS p)
```

#### OMIT FIELDS

```surql
SELECT * OMIT age FROM person
```

#### WHERE CLAUSE

```surql
SELECT * FROM person WHERE age > 40
```

```surql
SELECT name FROM person WHERE age > 40 AND name ~ //^[aA].*//
```

#### WITH INDEX

```surql
SELECT * FROM person WITH INDICES idx_name
```

#### WITH NO INDEX

```surql
SELECT * FROM person WITH NO INDEX
```

#### GROUP BY

```surql
SELECT * FROM product GROUP BY family
```

#### ORDER BY

```surql
SELECT * FROM person ORDER BY age DESC
```

#### START AT

```surql
SELECT * FROM person ORDER BY age DESC START AT 10
```

#### LIMIT TO

```surql
SELECT * FROM person START AT 10 LIMIT TO 100
```

<!-- --- -->

## GRAPH QUERIES

#### SIMPLE QUERIES

```surql
person -> likes -> product -- any person that likes any product
```

```surql
person:* -> likes -> product:*
```

#### REFLECTION

```surql
person:alice -> likes <- person -- any person alice likes that likes alice back
```

#### CONJUNCTION

```surql
person:alice -> likes OR plays -> person
```

```surql
person:alice -> likes AND plays -> person
```

```surql
person:alice -> NOT likes -> person
```

```surql
[person, animal] -> owns AND plays -> [game, toy]
```

#### TABLE RELATION

```surql
RELATE car ->> is_a ->> thing

RELATE driver ->> is_a ->> person

RELATE driver:james -> owns -> car:tesla

-- any [record] that is a person that owns any [record] that is a thing
(* -> is_a -> person) -> owns -> (* -> is_a -> thing)
```

### LEVELS

```surql
RELATE animal ->> is_a ->> thing
RELATE bird ->> is_a ->> animal
```

```surql
animal -> is_a -> thing -- Level 0
```

```surql
bird -> is_a -> animal -> is_a -> thing -- Level 1
```

```surql
bird -> is_a[1] -> thing -- Level 1
-- bird -> is_a -> * -> is_a -> thing
```

```surql
bird -> is_a[0..2] -> thing -- Level 0 to 1
-- bird -> is_a -> thing
-- bird -> is_a -> * -> is_a -> thing
```

**And in the future**

```surql
bird -> is_a[..] -> thing -- Level 0 to infinity
-- bird -> is_a -> thing
-- bird -> is_a -> * -> is_a -> thing
-- ...
```

<!-- --- -->

## UPDATE

```surql
UPDATE person:alice SET age = 30
```

```surql
UPDATE person:alice SET {
    age: 30
}
```

```surql
UPDATE person WHERE age > 40 SET age = 30
```

```surql
UPDATE person WHERE * SET age = none
```

<!-- --- -->

## DELETE

```surql
DELETE person:alice
```

```surql
DELETE person:alice -> buys -> * WHERE price > 10
```

```surql
DELETE person -- Deletes all records in a table but not the table itself.
```

<!-- --- -->

## OPERATIONS

#### ADDITION & SUBSTRACTION

```surql
UPDATE person:alice SET age += 1
```

```surql
UPDATE person:alice SET age -= 1
```

<!-- --- -->

## USE

```surql
USE NS staging DB app
```

```surql
USE NS staging DB app
```

```surql
USE NS staging
```

```surql
USE DB app
```

<!-- --- -->

## DEFINES

#### DEFINE NAMESPACE

```surql
DEFINE NAMESPACE staging
```

```surql
DEFINE NAMESPACE IF NOT EXISTS staging
```

```surql
DEFINE NS IF NOT EXISTS staging
```

#### DEFINE DATABASE

```surql
DEFINE DATABASE app ON staging
```

```surql
DEFINE DATABASE IF NOT EXISTS app
```

```surql
DEFINE DB app ON NS staging
```

#### DEFINE TABLE

```surql
DEFINE TABLE person FIELDS \
    name TYPE string,
    age TYPE u8
```

```surql
DEFINE TABLE IF NOT EXISTS product ON app FIELDS \
    name TYPE string,
    brand TYPE string VALUE "Gigamono",
    weight_lbs TYPE f64 VALUE 0.5
```

#### DEFINE EDGE

```surql
DEFINE EDGE buys FIELDS \
    expiry TYPE time
```

```surql
DEFINE EDGE IF NOT EXISTS buys ON app FIELDS \
    expiry TYPE time
```

#### DEFINE TYPE

```surql
DEFINE TYPE person FIELDS \
    name TYPE string,
    age TYPE u8
```

```surql
DEFINE TYPE person ON DB app IF NOT EXISTS FIELDS \
    name TYPE string,
    age TYPE u8
```

#### DEFINE ENUM

```surql
DEFINE ENUM color VARIANTS \
    red,
    green,
    blue
```

```surql
DEFINE ENUM IF NOT EXISTS color ON app VARIANTS \
    red,
    green,
    blue
```

#### DEFINE INDEX

```surql
DEFINE INDEX idx_name ON TABLE person FIELDS name
```

```surql
DEFINE INDEX idx_name ON TABLE person FIELDS name UNIQUE
```

```surql
DEFINE INDEX idx_name ON TABLE person FIELDS name WITH index::hnsw(m = 16, ef = 64)
```

#### DEFINE MODULE

```surql
DEFINE MODULE test ON DB app WITH
    export function name(): String {
        return "Alice"
    }
END

UPDATE person:alice SET name = test::name()
```

#### DEFINE PARAM

```surql
DEFINE PARAM endpoint VALUE "https://api.example.com" ON DATABASE app

http::get($endpoint)
```

```surql
DEFINE PARAM branch IF NOT EXISTS VALUE "main"
```

```surql
DEFINE PARAM age TYPE u8 IF NOT EXISTS
```

<!-- --- -->

## REMOVES

#### REMOVE NAMESPACE

```surql
REMOVE NAMESPACE staging
```

```surql
REMOVE NAMESPACE staging IF EXISTS
```

#### REMOVE DATABASE

```surql
REMOVE DATABASE app
```

```surql
REMOVE DATABASE IF EXISTS app
```

#### REMOVE TABLE

```surql
REMOVE TABLE person
```

```surql
REMOVE TABLE IF EXISTS person
```

#### REMOVE EDGE

```surql
REMOVE EDGE buys ON DB app
```

```surql
REMOVE EDGE buys IF EXISTS ON DB app
```

#### REMOVE TYPE

```surql
REMOVE TYPE person
```

```surql
REMOVE TYPE person IF EXISTS ON DB app
```

```surql
REMOVE TYPE person ON DB app
```

#### REMOVE ENUM

```surql
REMOVE ENUM color ON DB app
```

```surql
REMOVE ENUM color IF EXISTS ON DB app
```

#### REMOVE INDEX

```surql
REMOVE INDEX idx_name ON TABLE person ON DB app
```

```surql
REMOVE INDEX idx_name IF EXISTS ON TABLE person
```

#### REMOVE MODULE

```surql
REMOVE MODULE test
```

```surql
REMOVE MODULE test IF EXISTS
```

#### REMOVE PARAM

```surql
REMOVE PARAM $endpoint
```

```surql
REMOVE PARAM $endpoint IF EXISTS
```

<!-- --- -->

## DESCRIBES

#### DESCRIBE NAMESPACE

```surql
DESCRIBE NAMESPACE staging
```

```surql
DESCRIBE NAMESPACE staging IF EXISTS
```

#### DESCRIBE DATABASE

```surql
DESCRIBE DATABASE app
```

```surql
DESCRIBE DATABASE app IF EXISTS ON NS staging
```

#### DESCRIBE TABLE

```surql
DESCRIBE TABLE person
```

```surql
DESCRIBE TABLE person IF EXISTS ON DB app
```

#### DESCRIBE EDGE

```surql
DESCRIBE EDGE buys
```

```surql
DESCRIBE EDGE buys IF EXISTS ON DB app
```

#### DESCRIBE TYPE

```surql
DESCRIBE TYPE person
```

```surql
DESCRIBE TYPE person IF EXISTS ON DB app
```

#### DESCRIBE ENUM

```surql
DESCRIBE ENUM name
```

```surql
DESCRIBE ENUM name IF EXISTS ON DB app
```

#### DESCRIBE INDEX

```surql
DESCRIBE INDEX idx_name
```

```surql
DESCRIBE INDEX idx_name IF EXISTS ON DB app ON TABLE person
```

#### DESCRIBE MODULE

```surql
DESCRIBE MODULE test
```

```surql
DESCRIBE MODULE test IF EXISTS ON DB app
```

#### DESCRIBE PARAM

```surql
DESCRIBE PARAM endpoint
```

```surql
DESCRIBE PARAM endpoint IF EXISTS
```

<!-- --- -->

## FOR

```surql
FOR $id IN ["alice", "bob"] DO
    CREATE person SET id = $id IF NOT EXISTS
END
```

```surql
FOR $id IN ["alice", "bob"] DO
    CREATE person:$id IF NOT EXISTS
END
```

```surql
FOR $id IN (SELECT id FROM person WHERE age > 40) DO
    RELATE person:$id -> buys -> product:apple
END
```

<!-- --- -->

## IF

```surql
IF $age > 40 THEN
    CREATE person:alice SET age = 40
ELSE IF $age > 30 THEN
    CREATE person:alice SET age = 30
ELSE
    CREATE person:alice SET age = 20
END
```

<!-- --- -->

## TRANSACTIONS

```surql
BEGIN TRANSACTION

CREATE account:alice SET balance = 1000
CREATE account:bob SET balance = 1000
UPDATE account:alice SET balance += 500
UPDATE account:bob SET balance -= 500

COMMIT TRANSACTION
```

<!-- --- -->
