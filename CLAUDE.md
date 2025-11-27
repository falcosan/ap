# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Running the Application
```bash
cargo run
```
The server binds to `127.0.0.1:8000` by default (configurable via `PORT` env var).

### Building for Production
```bash
cargo build --release
```
The release profile is optimized with LTO, size optimization (`opt-level = "s"`), and panic=abort.

### Updating Dependencies
```bash
./update
```
This script runs `cargo upgrade -i allow` followed by `cargo update`.

## Environment Variables

Required variables (configure in `.env`):
- `ST_TOKEN` - Storyblok API token for fetching content
- `ST_BASE_URL` - Base URL for Storyblok API (typically `https://api.storyblok.com/v2/cdn/stories`)
- `AP_DATA` - Base URL for fetching XML data (RSS, sitemap)

Optional variables:
- `PORT` - Server port (defaults to 8000)
- `AP_BASE_URL` - Application base URL (used in templates and CORS)
- `DD_BASE_URL` - Additional allowed CORS origin
- `GOOGLE_VERIFICATION` - Google verification meta tag value

## Architecture Overview

### Request Flow
1. Request enters through `main.rs` where Axum server is configured with CORS and path normalization middleware
2. Router (`src/router/mod.rs`) merges static asset routes and page routes
3. Page handlers call page modules which use macros to fetch and transform data
4. Templates are rendered using the global `ENV` instance with Minijinja
5. HTML response is returned to the client

### Key Components

**Macro System** (`src/macros.rs`):
- `export!` - Module export utility for consistent page module exports
- `get_data!` - Fetches data from Storyblok API with two modes:
  - `{ slug: "path" }` - Fetches single story by slug
  - `{ starts_with: "prefix" }` - Fetches multiple stories, filtering out parent folders
- `extract_components!` - Recursively traverses JSON to extract components by name; automatically converts Markdown to HTML for "TextContent" components using pulldown-cmark

**Template Environment** (`src/environment.rs`):
- Global singleton `ENV` initialized via `LazyLock`
- All templates embedded at compile-time via `include_str!`
- Custom filters: `date_format` (formats ISO dates), `startswith` (string prefix check)
- Global variables: `current_year`, `AP_BASE_URL`, `google_verification`
- Fallback rendering: automatically renders fallback template on errors

**Routing** (`src/router/`):
- `page_routes.rs` - Dynamic page handlers (home, blog, articles) and XML endpoints (RSS, sitemap)
- `source_routes.rs` - Static file serving (CSS, favicon, robots.txt) from `src/static/`
- Legacy language redirects (`/it/*`, `/es/*`) redirect to root paths

### Page Structure

Each page module follows this pattern:
```rust
pub fn page_name(current_path: &str) -> String {
    let context = PageContext {
        data: get_data!({ slug: "page-slug" }),
        current_path: current_path.to_string(),
    };
    ENV.render_template("template.html", context)
}
```

Pages are located in `src/pages/` with co-located Jinja templates. Each page exports its handler function via the `export!` macro in `main.rs`.

### Template System

Templates use Minijinja (Jinja2-like syntax) with:
- Base layout: `src/layout/index.jinja`
- Page templates: `src/pages/{page}/index.jinja`
- Templates inherit from `layout.html` using `{% extends "layout.html" %}`
- Access to `page` (context data) and `current_path` variables

## Adding New Pages

1. Create directory in `src/pages/{page_name}/`
2. Add `mod.rs` with handler function following the pattern above
3. Add `index.jinja` template
4. Register template in `environment.rs` templates array
5. Add route in `router/page_routes.rs`
6. Export module in `main.rs` using `export!(page_name)`

## Data Fetching Pattern

Storyblok integration uses a custom macro-based approach:
- Use `get_data!({ slug: "path" })` to fetch a single story by full slug
- Use `get_data!({ starts_with: "blog/" })` to fetch all stories in a folder (excludes folder itself)
- Extract specific components with `extract_components!(&data, "ComponentName")`
- TextContent components automatically have Markdown converted to HTML

## Static Assets

Static files are served from `src/static/` directory. To add new static assets, add a route in `source_routes.rs` using `ServeFile`.
