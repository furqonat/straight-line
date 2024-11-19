# Straight-Line: Realtime Community Chat Application

Straight-Line like any other realtime chat application. This project leverages **Rust** for the backend, **React** with **TypeScript** for the frontend. Docker and Docker Compose are used for containerization, and the project follows a **monorepo** structure with **Nx** for efficient management.

## Tech Stack

- **Backend**: Rust, Actix Web, Nx for monorepo management.
- **Frontend**: React, TypeScript, Nx for monorepo management.
- **Database**: PostgreSQL.
- **Containerization**: Docker, Docker Compose.

## Getting Started

### Prerequisites

Before running the application, ensure you have the following installed:

- [Docker](https://www.docker.com/get-started)
- [Docker Compose](https://docs.docker.com/compose/install/)
- [Node.js](https://nodejs.org)
- [Pnpm](https://pnpm.io/installation)

### Installation

1. Clone the repository:

```bash
git clone https://github.com/furqonat/straight-line.git
cd straight-line
```

2. Install the nodejs package:

```bash
pnpm install
```

3. Build and run the application with Docker Compose:

```bash
docker compose up
```
