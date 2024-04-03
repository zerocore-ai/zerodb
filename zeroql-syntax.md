## Database Operations

#### Create

```js
database::create(#app_db, {
    ns: "bf91cccb-81ca-4ebe-9687-a79d7d3debb2"
})
```

#### Delete

```rs
delete database.#app_db
```

---

## Table Operations

#### Create Table

```js
table::create(#person, {
    name: [string, unique, is_valid_name],
    age: int,
})
```

#### Create Record

```rs
person::create(#john, {
    name: "John Doe",
    age: 42,
})
```

#### Update Record

```rs
person.#john.update({
    age: 43,
})
```

#### Delete Record

```rs
delete person.#john
```

#### Delete

```rs
delete table.#person
```

---

## Types

#### Definition

```rs
type person {
    name: string,
    age: int,
}

type average = (s: [int]) -> int

type color = 'red' | 'green' | 'blue'
```

#### Usage

```rs
let p: person = {
    name: "John Doe",
    age: 42,
}
```

---

## Let Bindings

```rs
let age = 50
let age: int = 100
let name: string = "John Doe"
```

---

## Querying

#### Select

```rs
[ { ... } in person ]
```

```rs
[ { name } in person ]
```

#### Guard

```rs
[ p in person, p.age > 18 ]
```

#### Expanded Form

```rs
[ { name: p.name, age: p.age } : p in person, p.age > 18 ]
```

---

## Operators

#### Uniform Function Call Syntax

```rs
[ { name: uppercase(p.name) } : p in person ]

[ { name: p.name.uppercase() } : p in person ]
```

#### Prefix Notation

```rs
[ { name: uppercase(p.name) } : p in person ]

[ { name: uppercase p.name } : p in person ]
```

#### Infix Notation

```rs
[ { age: mod(p.age, 18) } : p in person ]

[ { name: p.age mod 18 } : p in person ]
```

#### The Dot Notation

```rs
[ { name: .name.uppercase() } in person, [18..20] contains .age ]
```

#### The Pipe Operator

```rs
[ { name: .name.uppercase() } in person, [18..20] contains .age ] |> order 'asc' |> group_by 'age' |> limit 10
```

---

## Transactions

```rs
transaction {
    person::create(#john, {
        name: "John Doe",
        age: 42,
    })

    person.#john.update({
        age: 43,
    })

    remove person.#john
}
```

---

## If Statement

```rs
if age > 18 {
    "adult"
} else {
    "child"
}
```

---

## For Statement

```rs
let odd_ages = []
for p in person {
    if p.age > 100 {
        break
    }

    if p.age % 2 == 0 {
        continue
    }

    odd_ages.push(p.age)
}
```

---

## While Statement

```rs
while age < 18 {
    # ...
}
```

---

## Match Statement

```rs
match age {
    18 => "adult",
    0..=17 => "child",
}
```

```rs
match list {
    [] => 0,
    [x] => x,
    [x, ...xs] => x + sum(xs),
}
```

---

## Math Operations

#### Addition

```rs
[ { age: .age + 1 } in person ]
```

#### Subtraction

```rs
[ { age: .age - 1 } in person ]
```

#### Multiplication

```rs
[ { age: .age * 2 } in person ]
```

#### Division

```rs
[ { age: .age / 2 } in person ]
```

#### Modulo

```rs
[ { age: .age % 2 } in person ]
```

#### Power

```rs
[ { age: .age ^ 2 } in person ]
```

---

## Functions

#### Definition

```js
fun average(s: [int]) -> [int] {
    // ...
}
```

#### Usage

```rs
[ { age } in person ] |> average

[ p in person ] |> get_names
```

#### Default Arguments

```js
fun average(s: [int], n: int = 10) -> [int] {
    # ...
}
```

#### Result Type

```js
fun get_names(p: [person]) -> [string]! {
    # ...
}
```

---

## Operator Overloading

#### Plus

```js
let point = types::create(#point, {
    x: int,
    y: int,
})

fun __plus__(p1: point, p2: point) -> point {
    {
        x: p1.x + p2.x,
        y: p1.y + p2.y,
    }
}

let p1: point = {
    x: 1,
    y: 2,
}

let p2: point = {
    x: 3,
    y: 4,
}

let p3 = p1 + p2
```

#### Dot

```js
fun __dot_symbol__(p: person, s: symbol) {
    # ...
}

let p = person.#john;
```

---

## Graph Relations

#### Establishing a Relation

```rs
person.#john -> friend -> person.#jane
```

#### Removing a Relation

```rs
person.#john -!> friend -!> person.#jane
```

#### Querying a Relation

```rs
[ { name } in person.#john -> friend -> * ]
[ { name } in * -> friend -> person.#jane ]
[ { name } in * -> friend -> * ]
[ { name } in * -> friend ]
[ { name } in friend -> * ]
```

---

## Literals

#### Numeric

```py
123
+123
-123
123_u32
123.0
123.0e-2
123.0_f64
123.0e-2_f64
123.
.123
0b11001
0b11001_u32
0xabcf
0xabcf_f64
0o1274
0o1274_u16
```

#### String

```py
"Hello World"
'Hello World'
```

#### Boolean

```rs
true
false
```

#### Regex

```js
//John//
```

#### List

```rs
[1, 2, 3]
```

#### Object

```rs
{
    name: "John Doe",
    age: 42,
}
```

#### Tuple

```rs
(1, 2, 3)
```

#### Range

```rs
[1..10]
[1..=10]
```

#### Symbols

```
#james
#0xf356bc
#`😏j37386vSG)=`
```

---

## Block

```rs
{
    let x = 1;
    let y = 2;
    x + y
}
```

---

## Closure

```rs
|p| p.age > 18

|p| {
    p.age > 18
}
```

---

## Match

#### Numeric Matching

```rs
let age = [ { age } in person ] |> first
match age {
    18 => "adult",
    0..=17 => "child",
}
```

#### List Matching

```js
fun sum(s: [int]) -> [int] {
    match s {
        [] => 0,
        [x] => x,
        [x, ...xs] => x + sum(xs),
    }
}
```

---

## Importing

```rs
#[zql::import]
fn average(s: IntList) -> Int {
    // ...
}

zql! {
    import average

    [ { age } in person ] |> average
}
```
