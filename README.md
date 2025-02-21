# AP

A simple web application built in Rust using [Axum](https://github.com/tokio-rs/axum) for routing and [Minijinja](https://github.com/mitsuhiko/minijinja) for templating. This project pulls content from Storyblok using a custom macro and renders dynamic pages with Jinja templates.

## Features

- **Rust Web Server**: Uses [`axum`](https://github.com/tokio-rs/axum) for building an asynchronous HTTP server.
- **Templating**: Renders HTML pages using [`minijinja`](https://github.com/mitsuhiko/minijinja) with custom layouts and page templates.
- **Dynamic Data**: Fetches data from Storyblok through a custom `get_data!` macro defined in [`src/macros.rs`](src/macros.rs).
- **Static Assets**: Serves static files such as the CSS and favicon from [`src/assets/`](src/assets/).

## Project Structure

- **src/main.rs**: Application entry point that starts the Axum server. See [src/main.rs](src/main.rs).
- **src/environment.rs**: Sets up the Jinja templating environment and loads templates. See [src/environment.rs](src/environment.rs).
- **src/pages/**: Contains page modules for different routes:
  - **Home**: [src/pages/home/mod.rs](src/pages/home/mod.rs) and its template [src/pages/home/index.jinja](src/pages/home/index.jinja).
  - **Blog**: [src/pages/blog/mod.rs](src/pages/blog/mod.rs) with the article submodule [src/pages/blog/article/mod.rs](src/pages/blog/article/mod.rs) and template [src/pages/blog/article/index.jinja](src/pages/blog/article/index.jinja).
  - **Fallback**: [src/pages/fallback/mod.rs](src/pages/fallback/mod.rs) and its template [src/pages/fallback/index.jinja](src/pages/fallback/index.jinja).
- **src/layout/**: Contains the main layout template: [src/layout/default.jinja](src/layout/default.jinja).
- **src/router/**: Implements routing logic:
  - **Page Routes**: [src/router/page_routes.rs](src/router/page_routes.rs)
  - **Static Source Routes**: [src/router/source_routes.rs](src/router/source_routes.rs)
- **assets**: Static files such as [src/assets/index.css](src/assets/index.css) and [src/assets/favicon.ico](src/assets/favicon.ico).

## Prerequisites

- **Rust**: Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed (this project uses Rust edition 2021).
- **Environment Variables**: Create a `.env` file in the project root with the following variables:

  ```env
  ST_TOKEN=your_storyblok_token
  ST_BASE_URL=https://api.storyblok.com/v2/cdn/stories
  ```

## Installation

1. **Clone the repository**:

   ```sh
   git clone https://github.com/falcosan/ap
   cd ap
   ```

2. **Install Dependencies**: The project relies on crates defined in Cargo.toml. Cargo will automatically download these dependencies during the build.

## Running the Project

For development, you can use the provided shell script:

```sh
./dev.sh
```

The server binds to `127.0.0.1:8000`. Open your browser at [http://127.0.0.1:8000](http://127.0.0.1:8000) to view the application.

## Building for Production

To build the project in release mode, run:

```sh
cargo build --release
```

## License

This project is licensed under the MIT License.

## Additional Information

### Data Macros

The `extract_components!` custom macro defined in [`src/macros.rs`](src/macros.rs), recursively extracts JSON components matching a given target name from the provided JSON data.

#### Description

This macro searches through a JSON structure (which is expected to be a `serde_json::Value`) for objects
that have a `"component"` key. When it finds such an object, it checks whether the associated value matches
the specified target name.

If the target name is `"TextContent"`, the macro converts any Markdown text found under the `"text"` key into HTML
using the `pulldown_cmark` parser. The resulting HTML is then inserted back into the JSON object in place of the original text.

All matching components (with or without HTML conversion) are collected into a `Vec<serde_json::Value>` and returned.

#### Parameters

- `data`: An expression yielding a `serde_json::Value` containing the JSON tree to search.
- `name`: An expression yielding a string slice representing the target component's name.

#### Example Usage

```rust
// Suppose you have a JSON string representing your data:

let json_str = "{
    "component": "TextContent",
    "text": "# Hello World!"
}";

// Parse the string into a serde_json::Value

let data: serde_json::Value = serde_json::from_str(json_str).unwrap();

// Call the macro to extract components of type "TextContent"

let components = extract_components!(data, "TextContent");

// components now contains the content with the Markdown in "text" transformed into HTML.

// Further processing can be performed on this Vec<serde_json::Value> as needed.
```

#### Usage Notes

- Make sure your project includes the `serde_json` and `pulldown_cmark` crates.
- Import the module that defines this macro before using it.
- The macro internally traverses both JSON arrays and objects. Non-object or non-array values are skipped.
- When working with `"TextContent"` components, the macro expects a `"text"` key to be present in the object;
  otherwise, no HTML conversion is performed.

#### Retrieving the Macro

To use `extract_components!`:

1.  Add the module (e.g., `macros.rs`) containing the macro to your project.
2.  Ensure that the module is imported where needed (using `mod macros;` or similar).
3.  Invoke the macro by passing a `serde_json::Value` and a target component name as demonstrated above.
