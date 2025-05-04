# Full-Stack Development Template

A modern, production-ready template for full-stack applications featuring:
- React + TypeScript frontend
- Rust backend with Actix-web
- PostgreSQL database
- AWS services with LocalStack emulation
- Docker-based development environment
- End-to-end testing with Playwright

## Architecture

### Frontend
- React 18 with TypeScript
- Modern component architecture
- Playwright for E2E testing
- Hot-reloading development environment

### Backend
- Rust with Actix-web framework
- SQLx for type-safe database queries
- AWS SDK integration
- Database migrations
- Error handling middleware

### Infrastructure
- Docker Compose for local development
- PostgreSQL 16 database
- LocalStack for AWS service emulation
- Volume persistence for data

## Prerequisites

- Docker and Docker Compose
- Node.js 18+ (for local frontend development)
- Rust toolchain (for local backend development)
- Git

## Quick Start

1. Clone the repository:
```bash
git clone <repository-url>
cd <project-name>
```

2. Start the development environment:
```bash
docker-compose up -d
```

3. Access the applications:
- Frontend: http://localhost:3000
- Backend API: http://localhost:8000
- LocalStack: http://localhost:4566
- PostgreSQL: localhost:5432

## Development Guide

### Frontend Development

#### Local Development
```bash
cd frontend
npm install
npm start
```

#### Testing
```bash
cd frontend
npm test        # Run unit tests
npm run e2e     # Run Playwright tests
```

### Backend Development

#### Local Development
```bash
cd backend
cargo run
```

#### Testing
```bash
cd backend
cargo test
```

#### Database Migrations
```bash
cd backend
cargo sqlx migrate run
```

### AWS Services with LocalStack

The project uses LocalStack to emulate AWS services locally. Currently configured services:
- S3

To interact with LocalStack:
```bash
aws --endpoint-url=http://localhost:4566 s3 ls
```

### Environment Variables

#### Frontend
- `REACT_APP_API_URL`: Backend API URL (default: http://localhost:8000)

#### Backend
- `DATABASE_URL`: PostgreSQL connection string
- `AWS_ENDPOINT_URL`: LocalStack endpoint
- `AWS_ACCESS_KEY_ID`: AWS access key
- `AWS_SECRET_ACCESS_KEY`: AWS secret key
- `AWS_REGION`: AWS region

## Project Structure

```
.
├── backend/           # Rust backend
│   ├── src/          # Source code
│   ├── migrations/   # Database migrations
│   └── scripts/      # Development scripts
├── frontend/         # React frontend
│   ├── src/         # Source code
│   ├── public/      # Static assets
│   └── tests/       # Test files
├── localstack/      # LocalStack data
├── volume/          # Persistent volumes
└── docker-compose.yml
```

## Contributing

1. Create a new branch for your feature
2. Make your changes
3. Run tests
4. Submit a pull request

## Project Renaming

The project can be easily renamed using the provided script:

```bash
node scripts/rename-project.js <new-name>
```

For example, to rename the project to "myapp":
```bash
node scripts/rename-project.js myapp
```

This will:
- Update the database name to `<new-name>_db`
- Rename the main table and related database objects
- Update the backend model names
- Update the frontend application title

After running the script, you'll need to:
1. Recreate your database with the new name
2. Run the migrations again
3. Rebuild and restart your application

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## API Documentation

The backend provides the following endpoints:

- `GET /health` - Health check endpoint

## Database

The PostgreSQL database is configured with the following credentials:
- Database: example_db
- User: postgres
- Password: postgres
- Port: 5432 