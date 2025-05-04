# Game Project

A full-stack application with React TypeScript frontend, Rust backend, and PostgreSQL database.

## Prerequisites

- Docker
- Docker Compose
- Node.js (for local development)
- Rust (for local development)

## Project Structure

```
.
├── frontend/         # React TypeScript frontend
├── backend/          # Rust backend
├── docker-compose.yml
├── .env
└── README.md
```

## Getting Started

1. Clone the repository
2. Copy `.env.example` to `.env` and adjust the values if needed
3. Start the services:

```bash
docker-compose up --build
```

The services will be available at:
- Frontend: http://localhost:3000
- Backend: http://localhost:8000
- Database: localhost:5432

## Development

### Frontend Development

```bash
cd frontend
npm install
npm start
```

### Backend Development

```bash
cd backend
cargo run
```

## API Documentation

The backend provides the following endpoints:

- `GET /health` - Health check endpoint

## Database

The PostgreSQL database is configured with the following credentials:
- Database: game_db
- User: postgres
- Password: postgres
- Port: 5432 