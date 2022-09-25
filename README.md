# GarlandTools Rust Bindings/Wrapper

Unofficial Rust wrapper/bindings for [GarlandTools] API.  

> ⚠️ This is a public API.  
> ⚠️ Please do not spam or abuse it in any shape or form.

Special thanks to [GarlandTools] for providing this API and keeping it updated.

## Table of Contents

- [GarlandTools Rust Bindings/Wrapper](#garlandtools-rust-bindingswrapper)
  - [Table of Contents](#table-of-contents)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Credits](#credits)
  - [Versions](#versions)

## Installation

Simply add the following to your `[dependencies]` section in your `Cargo.toml`.

```bash
[dependencies]
garlandtools = "0.1.1
```

## Usage

All [GarlandTools] Endpoints are implemented in this API.  
Below is a table showing all endpoints and whether they have an _id_ and/or _all_ endpoint.  
An _id_ endpoint means there is a unique identifier that can be used to query information.
Commonly an integer. However, there is also a `Job` enum and string endpoints.  
An _all_ endpoint simply returns all data of that endpoint in a massive JSON file, no id needed but requires more filtering.

All endpoints return JSON which is parsed into **unstructured** `serde_json::Value`.
Usually either `serde_json::Value::Object` or `serde_json::Value::Array`.
This is true for all, but two endpoints which return a **binary PNG** instead (map and icons).  
A full overview is below:

| Endpoint Name | Has id endpoint | Has 'all' endpoint | Returns                            |
| ------------- | --------------- | ------------------ | ---------------------------------- |
| Achievement   | ✅               | ✅                  | JSON (`serde_json::Value::Object`) |
| Data          | ❌               | ✅                  | JSON (`serde_json::Value::Object`) |
| Endgame Gear  | ✅ (`Job`)       | ❌                  | JSON (`serde_json::Value::Object`) |
| Fate          | ✅               | ✅                  | JSON (`serde_json::Value::Object`) |
| Fishing       | ❌               | ✅                  | JSON (`serde_json::Value::Object`) |
| Icon          | ✅ (`str`)       | ❌                  | Binary PNG                         |
| Instance      | ✅               | ✅                  | JSON (`serde_json::Value::Object`) |
| Item          | ✅               | ❌                  | JSON (`serde_json::Value::Object`) |
| Leve          | ✅               | ✅                  | JSON (`serde_json::Value::Object`) |
| Leveling Gear | ✅ (`Job`)       | ❌                  | JSON (`serde_json::Value::Object`) |
| Map           | ✅ (`str`)       | ❌                  | Binary PNG                         |
| Mob           | ✅               | ✅                  | JSON (`serde_json::Value::Object`) |
| Node          | ✅               | ✅                  | JSON (`serde_json::Value::Object`) |
| NPC           | ✅               | ✅                  | JSON (`serde_json::Value::Object`) |
| Quest         | ✅               | ✅                  | JSON (`serde_json::Value::Object`) |
| Search        | ✅ (`str`)       | ❌                  | JSON (`serde_json::Value::Array`)  |
| Status        | ✅               | ✅                  | JSON (`serde_json::Value::Object`) |

To use the API, first initialize the `GarlandTools` class:

```rust
use garlandtools::GarlandTools;

let garlandtools = GarlandTools::default();
```

Optionally, you can specify a specific language:

```rust
use garlandtools::{GarlandTools, Language};

// For English language (default):
let garlandtools = GarlandTools::new(Language::English);

// For German language:
let garlandtools = GarlandTools::new(Language::German);

// For French language:
let garlandtools = GarlandTools::new(Language::French);

// For Japanese language:
let garlandtools = GarlandTools::new(Language::Japanese);
```

Each endpoint has it's own function implemented in the `GarlandTools` class.  
Simply call them and supply parameters _if needed_.
Say we want to query a specific item and we know the item id is `2`.  
All we need to do is:

```rust
// 0. Imports
use garlandtools::GarlandTools;

// 1. Initialize Wrapper/Binding
let garlandtools = GarlandTools::default();

// 2. Query item
let item_id = 2;
let item = garlandtools.item(item_id).await.unwrap();   // Unsafe! Check if the `Result` before unwrapping.

// 3. Do something with it!
println!("{:?}", item);
```

There is an additional `search` function to submit a search query.
**However, please use this endpoint only if absolutely necessary and you don't know a certain ID.**

## Credits

I want to credit [GarlandTools] and [GarlandTools NodeJS project](https://github.com/karashiiro/garlandtools-api) without which this wouldn't be possible.

[GarlandTools]: garlandtools.org/
[Requests-Cache]: https://pypi.org/project/requests-cache/

## Versions

| Version | Supported | Description                                              |
| ------- | --------- | -------------------------------------------------------- |
| v0.1.1  | ✅         | Much easier to use, lots of improvements!                |
| v0.1.0  | ✅         | Initial version. Works fine, but is harder to work with. |
