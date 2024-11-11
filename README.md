# Straight-Line: Realtime Community Chat Application

Straight-Line is a modern, scalable, and high-performance realtime community chat application built with cutting-edge technologies. This project leverages **Rust** for the backend, **React** with **TypeScript** for the frontend, and is fully deployable using **AWS Serverless**. Docker and Docker Compose are used for containerization, and the project follows a **monorepo** structure with **Nx** for efficient management.

## Features

- **Docker & Docker Compose**: Simplified local development and deployment with containerized environments.
- **Rust Backend**: The backend is built using Rust, with an emphasis on high performance, safety, and concurrency.
- **Dependency Injection**: Promotes modular and testable code by decoupling dependencies.
- **Nx Monorepo**: A monorepo setup using Nx to manage both the backend and frontend, ensuring consistency and ease of development.
- **React with TypeScript Frontend**: A modern, type-safe frontend built with React and TypeScript for strong type checking and scalability.
- **Realtime Communication**: Enables real-time chat features, ideal for community-driven discussions.
- **AWS Serverless Deployment**: Fully serverless architecture using AWS services for high availability, scalability, and cost-effectiveness.

## Tech Stack

- **Backend**: Rust, Actix Web, Nx for monorepo management.
- **Frontend**: React, TypeScript, Nx for monorepo management.
- **Database**: PostgreSQL.
- **Deployment**: AWS Lambda, API Gateway.
- **Containerization**: Docker, Docker Compose.

## Getting Started

### Prerequisites

Before running the application, ensure you have the following installed:

- [Docker](https://www.docker.com/get-started)
- [Docker Compose](https://docs.docker.com/compose/install/)
- [Nx CLI](https://nx.dev/cli)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/furqonat/straight-line.git
   cd straight-line
   ```

2. Install Nx CLI globally if not installed:

```bash
npm install -g nx
```

3. Build and run the application with Docker Compose:

```bash
docker compose up --build
```
