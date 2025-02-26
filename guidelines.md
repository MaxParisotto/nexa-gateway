# Workspace & Crate Structure

Initialize a Rust workspace with clearly defined crates:

gateway: The main RESTful API server built using Axum.

agora: A separate crate handling real-time WebSocket communication, topic subscriptions, and message routing.

vectordb: Crate managing interactions with Qdrant for embeddings and semantic vector searches.

auth: Handles authentication (JWT-based) and permissions with advanced security features.

common: Shared models, types, configuration handling, logging, and authentication middleware.

Use Tokio for asynchronous programming and concurrency.

Add structured error handling throughout the crates with thiserror or anyhow.

Include configuration management with config-rs, supporting YAML or TOML files.

Key Features to Implement:

Gateway Crate

REST API endpoints for AI agent orchestration.

Authentication middleware using JWT tokens.

Integration endpoints to communicate internally with the agora and vectordb.

Agora WebSocket Crate Requirements:

Implement WebSocket server with real-time capabilities using tokio-tungstenite or similar.

Allow dynamic subscriptions and topic-based messaging.

Include event-driven architecture for scalability and low latency.

VectorDB Crate (Qdrant integration):

Manage connection pooling and querying efficiently.

Provide functions to insert, update, query, and delete semantic embeddings.

Security:

JWT Authentication middleware implemented at the gateway layer.

Include SSL/TLS support for secure HTTPS communication.

Make sure the workspace structure is clean, modular, well-documented, and ready for expansion. Include a clear README outlining:

Project structure.

How to build, test, and run the crates locally.

How to integrate and scale in a production environment.
