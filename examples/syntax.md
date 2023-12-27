## Database Operations

### Create

```rs
zql! {
    db::create(@app_db, {
        namespace: "bf91cccb-81ca-4ebe-9687-a79d7d3debb2"
    })
}
```

### Destroy

```rs
zql! {
    destroy db.@app_db
}
```

---

## Table Operations

### Create

```rs
zql! {
    let persons = table::create(@persons, {
        name: string assert [is_uppercase]
        age: int,
    })
}
```

### Destroy

```rs
zql! {
    destroy table.@persons
}
```

---

## Enum Operations

### Create

```rs
zql! {
    let color = enum::create(@color, [
        @red,
        @green,
        @blue,
    ])
}
```

### Destroy

```rs
zql! {
    destroy enum.@color
}
```

---

## Document Operations

### Add

```rs
zql! {
    persons::add(@john, {
        name: "John Doe",
        age: 42,
    })
}
```

### Update

```rs
zql! {
    persons.@john.update({
        age: 43,
    })
}
```

### Remove

```rs
zql! {
    remove persons.@john
}
```

## Let Bindings

```rs
zql! {
    let red = color.@red
}
```

---

## Querying

### Select All

```rs
zql! {
    [ { name } in persons ]
}
```

### Conditional

```rs
zql! {
    [ p in persons, p.age > 18 ]
}
```

### Expanded Form

```rs
zql! {
    [ { name: p.name, age: p.age } : p in persons, p.age > 18 ]
}
```

---

## Operators

### Uniform Function Call Syntax

```rs
zql! {
    [ { name: uppercase(p.name) } : p in persons ]

    [ { name: p.name.uppercase() } : p in persons ]
}
```

### Prefix Notation

```rs
zql! {
    [ { name: uppercase(p.name) } : p in persons ]

    [ { name: uppercase p.name } : p in persons ]
}
```

### Infix Notation

```rs
zql! {
    [ { age: mod(p.age, 18) } : p in persons ]

    [ { name: p.age mod 18 } : p in persons ]
}
```

### The Dot Notation

```rs
zql! {
    [ { name: .name.uppercase() } in persons, 18..20 contains .age ]
}
```

### The Pipe Operator

```rs
zql! {
    [ { name: .name.uppercase() } in persons, 18..20 contains .age ] |> order asc |> group_by .age |> limit 10
}
```

---

## Math Operations

### Addition

```rs
zql! {
    [ { age: .age + 1 } in persons ]
}
```

---

## Graph Relations

### Establishing a Relation

```rs
zql! {
    persons.@john -> friend -> persons.@jane
}
```

### Removing a Relation

```rs
zql! {
    persons.@john -/> friend -/> persons.@jane
}
```

### Querying a Relation

```rs
zql! {
    [ { name } in persons.@john -> friend -> * ]
    [ { name } in * -> friend -> persons.@jane ]
    [ { name } in * -> friend -> * ]
    [ { name } in * -> friend ]
    [ { name } in friend -> * ]
}
```

---

## Literals

### Numeric

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

### String

```py
"Hello World"
'Hello World'
```

### Boolean

```rs
true
false
```

### Regex

```js
!/John/
```

### Stream

```rs
[1, 2, 3]
```

### Object

```rs
{
    name: "John Doe",
    age: 42,
}
```

### Tuple

```rs
(1, 2, 3)
```

### Range

```rs
1..10
1..=10
```

### Symbols

```
@james
@0xf356bc
@`😏j37386vSG)=`
```

---

## Block

```rs
zql! {
    {
        let x = 1;
        let y = 2;
        x + y
    }
}
```

---

## Pure Functions

```rs
zql! {
    function average(s: [int]) -> [int] {
        // ...
    }
}
```

---

## Closure

```rs
zql! {
    |p| p.age > 18

    |p| {
        p.age > 18
    }
}
```

---

## Match

### Numeric Matching

```rs
zql! {
    let age = [ { age } in persons ] |> first
    match age {
        18 => "adult",
        0..=17 => "child",
    }
}
```

### Stream Matching

```rs
zql! {
    function sum(s: [int]) -> [int] {
        match s {
            [] => 0,
            [x] => x,
            [x, ..xs] => x + sum(xs),
        }
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

    [ { age } in persons ] |> average
}
```
