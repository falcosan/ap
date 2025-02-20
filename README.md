# AP

A simple web application built in Rust using [Axum](https://github.com/tokio-rs/axum) and [Minijinja](https://github.com/mitsuhiko/minijinja) for templating. This project pulls content from Storyblok via its API and renders pages using custom Jinja templates.

## Features

- **Routing**: Handled by [`axum`](https://github.com/tokio-rs/axum) with separate page and source routes.
- **Templating**: Uses [`minijinja`](https://github.com/mitsuhiko/minijinja) to render HTML pages.
- **Dynamic Content**: Fetches content from Storyblok using the custom `get_data!` macro.

## Project Structure

- **src/main.rs**: Application entry point that starts the Axum server. (src/main.rs)
- **src/environment.rs**: Sets up the Jinja environment and loads templates. (src/environment.rs)
- **src/pages**: Contains the page modules:
  - Home: (src/pages/home/mod.rs) and (src/pages/home/index.jinja)
  - Blog: (src/pages/blog/mod.rs) and its article submodule (src/pages/blog/article/mod.rs)
  - Fallback: (src/pages/fallback/mod.rs)
- **src/layout/default.jinja**: The main layout template. (src/layout/default.jinja)
- **src/router**: Defines the route structure.
  - Page routes (src/router/page_routes.rs)
  - Source routes (src/router/source_routes.rs)
- **assets**: Contains static assets like CSS. (src/assets/index.css)

## Installation

1. **Clone the repository**:

   ```sh
   git clone https://github.com/falcosan/ap
   cd ap
   ```

2. **Set up environment variables**: Create a .env file in the project root. For example:

   ```env
   ST_TOKEN=your_storyblok_token
   ST_BASE_URL=https://api.storyblok.com/v2/cdn/stories
   ```

## Running the Project

To run the project in development mode, you can use the dev.sh script:

```sh
./dev.sh
```

The server binds to `127.0.0.1:8888`. Open your browser at [http://127.0.0.1:8888](http://127.0.0.1:8888).

## Building for Production

Build the project in release mode:

```sh
cargo build --release
```

## Additional Information

- **Environment Setup**: The application uses [`dotenv`](https://github.com/dotenv-rs/dotenv) to load environment variables. See details in environment.rs.
- **Routing Logic**: The router is built in mod.rs, routing both static assets and page routes.
- **Templating & API Data**: Templates are rendered with data fetched using the `get_data!` macro.
