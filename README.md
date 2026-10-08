# AP

A simple web application built in Rust using [Axum](https://github.com/tokio-rs/axum) for routing and [Minijinja](https://github.com/mitsuhiko/minijinja) for templating. This project pulls content from the Storyblok CDN API and renders dynamic pages with Jinja templates.

## Features

- **Rust Web Server**: Uses [`axum`](https://github.com/tokio-rs/axum) for building an asynchronous HTTP server.
- **Templating**: Renders HTML pages using [`minijinja`](https://github.com/mitsuhiko/minijinja) with custom layouts and page templates.
- **Dynamic Data**: Fetches data from Storyblok through the helpers in [`src/http.rs`](src/http.rs).
- **Static Assets**: Serves static files such as the CSS and favicon from [`src/static/`](src/static/).

## Project Structure

- **src/main.rs**: Application entry point that starts the Axum server. See [src/main.rs](src/main.rs).
- **src/environment.rs**: Sets up the Jinja templating environment and loads templates. See [src/environment.rs](src/environment.rs).
- **src/pages/**: Contains page modules for different routes:
  - **Home**: [src/pages/home/mod.rs](src/pages/home/mod.rs) and its template [src/pages/home/index.jinja](src/pages/home/index.jinja).
  - **Blog**: [src/pages/blog/mod.rs](src/pages/blog/mod.rs) with the article submodule [src/pages/blog/article/mod.rs](src/pages/blog/article/mod.rs) and template [src/pages/blog/article/index.jinja](src/pages/blog/article/index.jinja).
  - **Fallback**: [src/pages/fallback/mod.rs](src/pages/fallback/mod.rs) and its template [src/pages/fallback/index.jinja](src/pages/fallback/index.jinja).
- **src/layout/**: Contains the main layout template: [src/layout/index.jinja](src/layout/index.jinja).
- **src/router/**: Implements routing logic:
  - **Page Routes**: [src/router/page_routes.rs](src/router/page_routes.rs)
  - **Static Source Routes**: [src/router/source_routes.rs](src/router/source_routes.rs)
- **static**: Static files such as [src/static/index.css](src/static/index.css) and [src/static/favicon.ico](src/static/favicon.ico).

## Prerequisites

- **Rust**: Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed (this project uses Rust edition 2024).
- **Environment Variables**: Create a `.env` file in the project root with the following variables:

  ```env
  AP_BASE_URL=https://your-site.com/
  AP_DATA=https://your-data-host.com/
  ST_TOKEN=your_storyblok_token
  ST_BASE_URL=https://api.storyblok.com/v2/cdn/stories
  GOOGLE_VERIFICATION=optional_site_verification_token
  ```

  `PORT` (default `8000`) and `RUST_LOG` (default `info`) are optional.

## Installation

1. **Clone the repository**:

   ```sh
   git clone https://github.com/falcosan/ap
   cd ap
   ```

2. **Install Dependencies**: The project relies on crates defined in Cargo.toml. Cargo will automatically download these dependencies during the build.

## Running the Project

```sh
cargo run
```

The server binds to `0.0.0.0:8000`. Open your browser at [http://localhost:8000](http://localhost:8000) to view the application.

## Building for Production

To build the project in release mode, run:

```sh
cargo build --release
```

## License

This project is licensed under the MIT License.
