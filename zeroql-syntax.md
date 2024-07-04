## Database Operations

#### Create

```js
create_database! {
    name: app_db,
    namespace: "bf91cccb-81ca-4ebe-9687-a79d7d3debb2"
}
```

#### Delete

```rs
delete database:app_db
```

---

## Table Operations

#### Create Table

```js
create_table! {
    type: person,
    fields: {
        name: string @ (is_unique, is_valid_name),
        age: int,
    }
}
```

#### Nested Objects

```js
create_table! {
    type: product,
    fields: {
        name: string @ is_unique,
        price: f64,
        meta: {
            color: string,
            size: string,
        }
    }
}
```

#### Create Record

```rs
person:john.create({
    name = "John Doe",
    age = 42,
})
```

#### Update Record

```rs
person:john.update({ age = 43 })
```

#### Delete Record

```rs
delete person:john
```

#### Delete

```rs
delete table:person
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

type optional<t> = some(t) | none
```

#### Usage

```rs
let p: person = {
    name: "John Doe",
    age: 42,
}
```

---

## Traits

#### Definition

```rs
trait store<T> {
    fun create(name: string, data: T)
    fun update(name: string, data: T)
    fun delete(name: string)
}
```

#### Implementation

```rs
type memstore<T> where T: store {
    data: hashmap<string, T>
}

fun create(m: memstore<vec<u8>>, name: string, data: vec<u8>) {
    m.data.insert(name, data)
}

fun update(m: memstore<vec<u8>>, name: string, data: vec<u8>) {
    m.data.update(name, data)
}

fun delete(m: memstore<vec<u8>>, name: string) {
    m.data.remove(name)
}
```

---

## Typing

#### Static Typing

```py
fn print_name(p: person) {
    print(f"{p.name}")
}

let p = {
    name: "John Doe",
    age: 42,
}

print_name(p) # Error
```

#### Structural Typing

```py
fn print_name(p: { name: string }) {
    print(f"{p.name}")
}

let p = {
    name: "John Doe",
    age: 42,
}

print_name(p) # Okay
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
( { * } in person )
```

```rs
( { name } in person )
```

```rs
( { $0 } in person )
```

#### Guard

```rs
( p in person; p.age > 18 )
```

#### Expanded Form

```rs
( { name: p.name, age: p.age } for p in person; p.age > 18 )
```

#### Zip

```py
( { name, manager } in zip(employee, department); e.department_id == d.id ) # zip<T, U>(s1: [T], s2: [U]) -> [(T, U)]
```

```py
[ { name: e.name, manager: d.manager } for (e, d) in zip(employee, department); e.department_id == d.id ]
```

### Aggregation

```py
customer |> group 'country' # group<T, F, R, U>(s: [T], f: F) -> [{ F: U, data: [R] }] where T: {F, *R}, T[*R]: U
```

```rs
( { country, count(data) } in customer |> group 'country' )
```

```rs
( { country, count: count(data) } for { country, data } in customer |> group 'country' )
```

---

## Operators

#### Uniform Function Call Syntax

```rs
( { name: uppercase(p.name) } for p in person )

( { name: p.name.uppercase() } for p in person )
```

#### Prefix Notation

```rs
( { name: uppercase(name) } in person )

( { name: uppercase name } in person )
```

#### The Object Field

```rs
( { name: name.uppercase() } for p in person; [18..20] contains p.age )
```

#### The Pipe Operator

```rs
( { name: name.uppercase() } for p in person; [18..20] contains p.age ) |> order 'asc' |> group 'age' |> limit 10
```

---

## Transactions

```rs
transaction {
    person:john.create {
        name =  "John Doe",
        age = 42,
    }

    person:john.update {
        age: 43,
    }

    person:john.remove()
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
match array {
    @[] => 0,
    @[x] => x,
    @[x, *xs] => x + sum(xs),
}
```

---

## Math Operations

#### Addition

```rs
( { age: p.age + 1 } for p in person )
```

#### Subtraction

```rs
( { age: p.age - 1 } for p in person )
```

#### Multiplication

```rs
( { age: p.age * 2 } for p in person )
```

#### Division

```rs
( { age: p.age / 2 } for p in person )
```

#### Modulo

```rs
( { age: p.age % 2 } for p in person )
```

#### Power

```rs
( { age: p.age ^ 2 } for p in person )
```

---

## Functions

#### Definition

```js
fun average(s: [int]) -> [int] {
    # ...
}
```

```js
fun person::create(id: symbol, data: { name: string, age: int }) -> void {
    # ...
}
```

#### Usage

```rs
( { age } in person ) |> average

( p in person ) |> get_names
```

#### Default Arguments

```js
fun average(s: [int], n: int = 10) -> [int] {
    # ...
}
```

#### Result Type

```js
fun compute(a: f64, b: f64) -> result<f64, error> {
    # ...
}
```

```js
let result = try compute(1.0, 2.0)

let result = unwrap compute(1.0, 2.0)

let result = compute(1.0, 2.0) |> unwrap_or 0.
```

#### Option Type

```js
fun compute(a: f64, b: f64) -> option<f64> {
    # ...
}
```

```js
let result = unwrap compute(1.0, 2.0)

let result = compute(1.0, 2.0) |> unwrap_or 0.
```

---

## Operator Overloading

#### Plus

```js
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

#### Colon

```js
fun __colon_symbol__(p: person, s: symbol) {
    # ...
}

let p = person:john;
```

---

## Graph Relations

#### Establishing a Relation

```rs
person:john -> likes -> person:jane
```

#### Removing a Relation

```rs
person:john -!> likes -!> person:jane
```

#### Querying a Relation

```rs
( { name } in person:john -> likes -> * )
( { name } in * -> likes -> person:jane )
( { name } in * -> likes -> * )
( { name } in * -> likes )
( { name } in likes -> * )
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

#### Symbol

```py
"Hello World" # String literals are also Symbols
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

#### Stream

```rs
(:1 :2 :3)
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
1..10
1..=10
```

#### Symbols

```
@james
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
        [x, *xs] => x + sum(xs),
    }
}
```

---

## Importing

```rs
#[zql::import]
fn average(s: IntStream) -> Int {
    // ...
}

zql! {
    import average
    import person::*
    import store::{ create, update, delete }

    ( { age } in person ) |> average
}
```

---

## Exporting

```rs
export {
    average,
    store::new,
    person::*,
}
```
