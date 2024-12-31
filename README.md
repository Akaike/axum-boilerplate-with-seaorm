# Axum Boilerplate with SeaORM

This repository provides a basic boilerplate for building a Rust backend with Axum.

## Stack
- **Axum**: Web framework for building HTTP services.
- **SeaORM**: Async & dynamic ORM for interacting with databases.
- **Validator**: For request data validation.
- **Tower**: Middleware framework for composing asynchronous service chains.
- **Anyhow**: Simplified error handling.
- **PostgreSQL**

## Features
- **Sample Server**: A basic server setup demonstrating the integration of SeaORM with a RESTful API.
- **JWT Authentication Middleware**: Basic JWT authentication middleware to secure specific routes.
- **Cargo Generate Support**: Easy project scaffolding with interactive configuration.

## Project Structure
- `entity/`: Contains the entity definitions automatically generated by SeaORM.
- `migration/`: Contains migration scripts for database schema evolution.
- `server/`: Contains the server implementation, including routes and handlers.

## Getting Started

### Prerequisites
- Rust
- Cargo
- Docker
- Cargo Generate

### Installation

#### Using Cargo Generate
1. Install cargo-generate if you haven't already:
```bash
cargo install cargo-generate
```

2. Generate your project:
```bash
cargo generate --git https://github.com/akaike/axum-boilerplate-with-seaorm
```

This will:
- Create a new project with your chosen name
- Set up database configuration in `.env.example`
- Configure JWT authentication (optional)

3. Create your .env file from the template:
```bash
cp .env.example .env
```

4. Update the `.env` file if necessary

### Start database:
```bash
docker compose up -d
```

### Running Migrations:
```bash
cd migration
cargo run
```

### Running the Server:
```bash
cargo run
```
By default, the server runs on port 3000. To change this, adjust the configuration in /server/src/server.rs

Refer to the [SeaORM Documentation](https://www.sea-ql.org/SeaORM/docs/migration/writing-migration/) for additional details on customizing entities and other advanced features.

## API Endpoints

### Todos
- `GET /api/v1/todos/:id` - Get a todo by ID
- `POST /api/v1/todos` - Create a new todo
- `PUT /api/v1/todos/:id` - Update a todo
- `DELETE /api/v1/todos/:id` - Delete a todo

### Request/Response Examples

#### Create Todo
```bash
POST /api/v1/todos
Content-Type: application/json

{
    "title": "My new todo"
}
```

#### Update Todo
```bash
PUT /api/v1/todos/:id
Content-Type: application/json

{
    "title": "Updated todo",
    "completed": true
}
```

#### Delete Todo
```bash
DELETE /api/v1/todos/:id
```

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.