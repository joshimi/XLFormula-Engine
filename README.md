# XLFormula Engine

XLFormula Engine is a Rust crate for parsing and evaluating Excel formulas.

It works with f64 by default (the `f64` feature) or with `f32` by enabling the `f32` feature and disabling default features.

## Features

It supports:

* Any numbers, negative and positive, as float or integer
* Arithmetic operations +, -, /, *, ^
* Logical operations AND(), OR(), NOT(), XOR()
* Comparison operations =, >, >=, <, <=, <>
* String operation & (concatenation)
* Build-in variables TRUE, FALSE
* Excel functions ABS(), SUM(), PRODUCT(), AVERAGE(), RIGHT(), LEFT(), IF(), ISBLANK()
* Operations on lists of values (one dimensional range)
* Add or subtract dates and excel funtion DAYS()
* Custom functions with number arguments
* Handle blank/null values in calculation
* Handle empty/missing parameters of function calls as blank values

## Installation

Add the corresponding entry to your Cargo.toml dependency list:

```toml
[dependencies]
xlformula_engine = "0.3.0"
```

Or use the following for using `f32`:

```toml
[dependencies]
xlformula_engine = { version = "0.3.0", default-features = false, features = ["f32"] }
```

## Examples

Here are simple examples of parsing an Excel formula string and evaluating to a result:

```rust
extern crate xlformula_engine;
use xlformula_engine::calculate;
use xlformula_engine::parse_formula;
use xlformula_engine::NoReference;
use xlformula_engine::NoCustomFunction;

fn main() {
let formula = parse_formula::parse_string_to_formula(&"=1+2", None::<NoCustomFunction>);
let result = calculate::calculate_formula(formula, None::<NoReference>);
println!("Result is {}", calculate::result_to_string(result));

let formula = parse_formula::parse_string_to_formula(&"=(1*(2+3))*2", None::<NoCustomFunction>);
let result = calculate::calculate_formula(formula, None::<NoReference>);
println!("Result is {}", calculate::result_to_string(result));

let formula = parse_formula::parse_string_to_formula(&"=1+3/0", None::<NoCustomFunction>); // error (#DIV/0!)
let result = calculate::calculate_formula(formula, None::<NoReference>);
println!("Result is {}", calculate::result_to_string(result));
}
```

The last string is evaluated to #DIV/0!.

Concatenating strings:

```rust
extern crate xlformula_engine;
use xlformula_engine::calculate;
use xlformula_engine::parse_formula;
use xlformula_engine::NoReference;
use xlformula_engine::NoCustomFunction;

fn main() {
let formula = parse_formula::parse_string_to_formula(&"=\"Hello \" & \" World!\"", None::<NoCustomFunction>);
let result = calculate::calculate_formula(formula, None::<NoReference>);
println!("Result is {}", calculate::result_to_string(result));

let formula = parse_formula::parse_string_to_formula(&"=1 + \"Hello\"", None::<NoCustomFunction>); // error (#CAST!)
let result = calculate::calculate_formula(formula, None::<NoReference>);
println!("Result is {}", calculate::result_to_string(result));
}
```

Concatenating number and string results in a #CAST! error.

Constants ( i.e. a string without '=' ):

```rust
extern crate xlformula_engine;
use xlformula_engine::calculate;
use xlformula_engine::parse_formula;
use xlformula_engine::NoReference;
use xlformula_engine::NoCustomFunction;

fn main() {
let formula = parse_formula::parse_string_to_formula(&"1.2", None::<NoCustomFunction>);
let result = calculate::calculate_formula(formula, None::<NoReference>);
println!("Result is {}", calculate::result_to_string(result));

let formula = parse_formula::parse_string_to_formula(&"Hello World", None::<NoCustomFunction>);
let result = calculate::calculate_formula(formula, None::<NoReference>);
println!("Result is {}", calculate::result_to_string(result));
}
```

Excel functions:

```rust
extern crate xlformula_engine;
use xlformula_engine::calculate;
use xlformula_engine::parse_formula;
use xlformula_engine::NoReference;
use xlformula_engine::NoCustomFunction;

fn main() {
let formula = parse_formula::parse_string_to_formula(&"=ABS(-1)", None::<NoCustomFunction>);
let result = calculate::calculate_formula(formula, None::<NoReference>);
println!("Result is {}", calculate::result_to_string(result));

let formula = parse_formula::parse_string_to_formula(&"=SUM(1,2,\"3\")", None::<NoCustomFunction>);
let result = calculate::calculate_formula(formula, None::<NoReference>);
println!("Result is {}", calculate::result_to_string(result));

let formula = parse_formula::parse_string_to_formula(&"=PRODUCT(ABS(1),2*1, 3,4*1)", None::<NoCustomFunction>);
let result = calculate::calculate_formula(formula, None::<NoReference>);
println!("Result is {}", calculate::result_to_string(result));

let formula = parse_formula::parse_string_to_formula(&"=RIGHT(\"apple\", 3)", None::<NoCustomFunction>);
let result = calculate::calculate_formula(formula, None::<NoReference>);
println!("Result is {}", calculate::result_to_string(result));

let formula = parse_formula::parse_string_to_formula(&"=LEFT(\"apple\", 3)", None::<NoCustomFunction>);
let result = calculate::calculate_formula(formula, None::<NoReference>);
println!("Result is {}", calculate::result_to_string(result));

let formula = parse_formula::parse_string_to_formula(&"=LEFT(\"apple\")", None::<NoCustomFunction>);
let result = calculate::calculate_formula(formula, None::<NoReference>);
println!("Result is {}", calculate::result_to_string(result));

let formula = parse_formula::parse_string_to_formula(&"=IF(TRUE,1,0)", None::<NoCustomFunction>);
let result = calculate::calculate_formula(formula, None::<NoReference>);
println!("Result is {}", calculate::result_to_string(result));
}
```

Logical expressions:

```rust
extern crate xlformula_engine;
use xlformula_engine::calculate;
use xlformula_engine::parse_formula;
use xlformula_engine::NoReference;
use xlformula_engine::NoCustomFunction;

fn main() {
let formula = parse_formula::parse_string_to_formula(&"=2>=1", None::<NoCustomFunction>);
let result = calculate::calculate_formula(formula, None::<NoReference>);
println!("Result is {}", calculate::result_to_string(result));

let formula = parse_formula::parse_string_to_formula(&"=OR(1>1,1<>1)", None::<NoCustomFunction>);
let result = calculate::calculate_formula(formula, None::<NoReference>);
println!("Result is {}", calculate::result_to_string(result));

let formula = parse_formula::parse_string_to_formula(&"=AND(\"test\",\"True\", 1, true) ", None::<NoCustomFunction>);
let result = calculate::calculate_formula(formula, None::<NoReference>);
println!("Result is {}", calculate::result_to_string(result));
}
```

References:

```rust
extern crate xlformula_engine;
use xlformula_engine::calculate;
use xlformula_engine::parse_formula;
use xlformula_engine::types;
use xlformula_engine::NoReference;
use xlformula_engine::NoCustomFunction;

fn main() {
let data_function = |s: String| match s.as_str() {
"A" => types::Value::Text("=1+B".to_string()),
"B" => types::Value::Number(3.0),
_ => types::Value::Error(types::Error::Value),
};
let formula = parse_formula::parse_string_to_formula(&"=A+B", None::<NoCustomFunction>);
let result = calculate::calculate_formula(formula, Some(&data_function));
println!("Result is {}", calculate::result_to_string(result));
}
```

List:

```rust
extern crate xlformula_engine;
use xlformula_engine::calculate;
use xlformula_engine::parse_formula;
use xlformula_engine::NoReference;
use xlformula_engine::NoCustomFunction;

fn main() {
let formula = parse_formula::parse_string_to_formula(&"={1,2,3}+{1,2,3}", None::<NoCustomFunction>);
let result = calculate::calculate_formula(formula, None::<NoReference>);
println!("Result is {}", calculate::result_to_string(result));    

let formula = parse_formula::parse_string_to_formula(&"=XOR({0,0,0})", None::<NoCustomFunction>);
let result = calculate::calculate_formula(formula, None::<NoReference>);
println!("Result is {}", calculate::result_to_string(result));

let formula = parse_formula::parse_string_to_formula(&"=AVERAGE({1,2,3},1,2,3)", None::<NoCustomFunction>);
let result = calculate::calculate_formula(formula, None::<NoReference>);
println!("Result is {}", calculate::result_to_string(result));
}
```

Date:

```rust
extern crate xlformula_engine;
use xlformula_engine::calculate;
use xlformula_engine::parse_formula;
use xlformula_engine::types;
use chrono::format::ParseError;
use chrono::{DateTime, FixedOffset};
use xlformula_engine::NoReference;
use xlformula_engine::NoCustomFunction;


fn main() -> Result<(), ParseError> {
let start: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2019-03-01T02:00:00.000Z")?;
let end: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2019-08-30T02:00:00.000Z")?;
let data_function = |s: String| match s.as_str() {
"start" => types::Value::Date(start),
"end" => types::Value::Date(end),
_ => types::Value::Error(types::Error::Value),
};

let formula = parse_formula::parse_string_to_formula(&"=DAYS(end, start)", None::<NoCustomFunction>);
let result = calculate::calculate_formula(formula, Some(&data_function));
println!("Result is {}", calculate::result_to_string(result));

let formula = parse_formula::parse_string_to_formula(&"=start+1", None::<NoCustomFunction>);
let result = calculate::calculate_formula(formula, Some(&data_function));
println!("Result is {}", calculate::result_to_string(result));

let formula = parse_formula::parse_string_to_formula(&"=end-3", None::<NoCustomFunction>);
let result = calculate::calculate_formula(formula, Some(&data_function));
println!("Result is {}", calculate::result_to_string(result));
Ok(())
}
```

Custom Function:

```rust
extern crate xlformula_engine;
use xlformula_engine::calculate;
use xlformula_engine::parse_formula;
use xlformula_engine::types;
use xlformula_engine::types::XlNum;
use xlformula_engine::NoReference;

fn main() {
let custom_functions = |s: String, params: Vec<XlNum>| match s.as_str() {
"Increase" => types::Value::Number(params[0] + 1.0),
"SimpleSum" => types::Value::Number(params[0] + params[1]),
"EqualFive" => types::Value::Number(5.0),
_ => types::Value::Error(types::Error::Value),
};

let formula =
parse_formula::parse_string_to_formula(&"=Increase(1)+1", Some(&custom_functions));
let result = calculate::calculate_formula(formula, None::<NoReference>);
println!("Result is {}", calculate::result_to_string(result));

let formula =
parse_formula::parse_string_to_formula(&"=EqualFive()+1", Some(&custom_functions));
let result = calculate::calculate_formula(formula, None::<NoReference>);
println!("Result is {}", calculate::result_to_string(result));

let formula =
parse_formula::parse_string_to_formula(&"=SimpleSum(1,2)", Some(&custom_functions));
let result = calculate::calculate_formula(formula, None::<NoReference>);
println!("Result is {}", calculate::result_to_string(result));
}
```

Handle blank in calculation:

```rust
extern crate xlformula_engine;
use xlformula_engine::calculate;
use xlformula_engine::parse_formula;
use xlformula_engine::types;
use chrono::format::ParseError;
use chrono::{DateTime, FixedOffset};
use xlformula_engine::NoReference;
use xlformula_engine::NoCustomFunction;

fn main() -> {
    let data_function = |s: String| match s.as_str() {
        "B" => types::Value::Blank,
        _ => types::Value::Error(types::Error::Value),
    };

    let custom_functions = |s: String, params: Vec<XlNum>| match s.as_str() {
        "BLANK" => types::Value::Blank,
        _ => types::Value::Error(types::Error::Value),
    };

    let formula = parse_formula::parse_string_to_formula(&"=SUM(B, 1)", None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, Some(&data_function));
    println!("Result is {}", calculate::result_to_string(result));

    let formula =
        parse_formula::parse_string_to_formula(&"=SUM(BLANK(), 1)", Some(&custom_functions));
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));

    //takes list as input
    let formula = parse_formula::parse_string_to_formula(&"=SUM({B, 1})", None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, Some(&data_function));
    println!("Result is {}", calculate::result_to_string(result));
    let formula = parse_formula::parse_string_to_formula(&"=XOR({F,B,T,B,F,{F,B,T,B,F}})", None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, Some(&data_function));
    println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=SUM(1, 2, , 3)", None::<NoCustomFunction>);
    let result = calculate::calculate_formula(formula, None::<NoReference>);
    println!("Result is {}", calculate::result_to_string(result));
}
```

## License

Licensed under MIT License (see the [LICENSE](https://github.com/jiradaherbst/XLFormula-Engine/blob/master/LICENSE) file for the full text).

## Contact

Please feel free to contact us at jirada.herbst@data2impact.com.
