# Nexa Gateway

A modular, scalable API gateway system built with Rust, designed for AI agent orchestration with real-time communication capabilities.

## Project Overview

Nexa Gateway is a comprehensive backend system that provides:

- RESTful API endpoints for AI agent orchestration
- Real-time WebSocket communication
- Vector database integration for semantic searches
- JWT-based authentication and authorization
- Modular architecture for scalability

## Project Structure

The project is organized as a Rust workspace with multiple crates:

nexa-gateway/
├── gateway/       # Main API server using Axum
├── api/           # Secondary API server
├── service/       # Business logic services
├── auth/          # Authentication and authorization
├── agora/         # WebSocket server for real-time communication
├── vectordb/      # Vector database integration (Qdrant)
├── common/        # Shared utilities, models, and middleware
├── core/          # Core functionality and abstractions
└── config/        # Configuration files

### Crate Details

- **gateway**: The main RESTful API server built using Axum. Handles HTTP requests, routes them to appropriate services, and returns responses.
- **api**: A secondary API server, possibly for internal or specialized endpoints.
- **service**: Contains business logic services that implement the application's core functionality.
- **auth**: Handles authentication (JWT-based) and permissions with advanced security features.
- **agora**: A WebSocket server for real-time communication, topic subscriptions, and message routing.
- **vectordb**: Manages interactions with Qdrant for embeddings and semantic vector searches.
- **common**: Shared models, types, configuration handling, logging, and error handling.
- **core**: Core functionality and abstractions used across the system.

## Prerequisites

- Rust (latest stable version)
- PostgreSQL database
- Qdrant vector database (optional, for vector search functionality)

## Configuration

Configuration is managed through YAML files in the `config/` directory:

- `default.yaml`: Base configuration
- `development.yaml`: Development environment overrides
- `production.yaml`: Production environment overrides
- `test.yaml`: Test environment overrides

You can also override configuration using environment variables with the prefix `APP__` (e.g., `APP__SERVER__PORT=8080`).

Example configuration:

```yaml
environment: "development"

server:
  host: "0.0.0.0"
  port: 8080

database:
  url: "postgres://user:password@localhost/nexaagents"
  max_connections: 10

auth:
  jwt_secret: "supersecretkey"
  jwt_expiration: 24 # 24 hours

agora:
  host: "0.0.0.0"
  port: 8081

logging:
  level: "info"
```

## Building and Running

### Cargo Commands

#### Build the Project

```bash
# Build all crates in debug mode
cargo build

# Build all crates in release mode
cargo build --release

# Build a specific crate
cargo build -p nexa-gateway-gateway
```

#### Run the Gateway Server

```bash
# Run the gateway server
cargo run -p nexa-gateway-gateway

# Run with a specific configuration
APP_ENVIRONMENT=production cargo run -p nexa-gateway-gateway
```

#### Run the API Server

```bash
# Run the API server
cargo run -p nexa-gateway-api
```

#### Run Tests

```bash
# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p nexa-gateway-common
```

#### Check for Errors

```bash
# Check for compilation errors without building
cargo check
```

#### Generate Documentation

```bash
# Generate documentation
cargo doc --open
```

## API Endpoints

### Gateway API

- `GET /`: Health check endpoint
- `GET /api/agents`: List all agents
- `POST /api/agents`: Create a new agent
- `GET /api/agents/{id}`: Get agent by ID

### WebSocket API (Agora)

The Agora WebSocket server provides real-time communication capabilities:

- Connect to WebSocket: `ws://{host}:{port}`
- Subscribe to topics
- Publish messages to topics
- Receive real-time updates

## Authentication

The system uses JWT (JSON Web Token) for authentication:

1. Obtain a JWT token by authenticating with credentials
2. Include the token in the `Authorization` header for API requests
3. The token includes user roles for authorization

## Development

### Adding a New Feature

1. Identify which crate should contain the feature
2. Implement the feature following the existing patterns
3. Add tests to verify functionality
4. Update documentation as needed

### Project Conventions

- Use async/await for asynchronous operations
- Follow Rust's error handling patterns with Result types
- Use structured logging with tracing
- Document public APIs with rustdoc comments

## Deployment

### Development Environment

```bash
# Run in development mode
cargo run -p nexa-gateway-gateway
```

### Production Environment

```bash
# Build in release mode
cargo build --release

# Run the gateway binary
./target/release/nexa-gateway-gateway
```

### Docker Deployment

A Dockerfile is provided for containerized deployment:

```bash
# Build the Docker image
docker build -t nexa-gateway .

# Run the container
docker run -p 8080:8080 -p 8081:8081 nexa-gateway
```

## Contributing

1. Update the worklog.md file with your changes
2. Follow the project's code style and conventions
3. Write tests for new functionality
4. Submit a pull request with a clear description of changes

## License

This project is licensed under the MIT License - see the LICENSE file for details.
