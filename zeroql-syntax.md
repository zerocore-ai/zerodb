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

#### TYPE

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

#### WHERE CLAUSE

```surql
SELECT * FROM person WHERE age > 40
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
DEFINE MODULE test ON DB app {
    export function name(): String {
        return "Alice"
    }
}

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

#### DEFINE TYPE

```surql
DEFINE TYPE person {
    name: string,
    age: u8,
}
```

```surql
DEFINE TYPE color =
    | TYPE red
    | TYPE green
    | TYPE blue
```

```surql
DEFINE TYPE singleton
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
FOR $id IN ["alice", "bob"] {
    CREATE person SET id = $id IF NOT EXISTS
}
```

```surql
FOR $id IN ["alice", "bob"] {
    CREATE person:$id IF NOT EXISTS
}
```

```surql
FOR $id IN (SELECT id FROM person WHERE age > 40) {
    RELATE person:$id -> buys -> product:apple
}
```

<!-- --- -->

## IF

```surql
IF $age > 40 {
    CREATE person:alice SET age = 40
} ELSE IF $age > 30 {
    CREATE person:alice SET age = 30
} ELSE {
    CREATE person:alice SET age = 20
}
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
