# To-Do App Tech Stack

## Backend
- Language: Rust
- Web Framework: Axum
- Database: PostgreSQL
- ORM: Sea-ORM
- Authentication: jsonwebtoken

## Frontend
- Rendering: Server-Side Rendering (SSR)
- Templating: Askama
- Styling: 
  - Tailwind CSS
  - DaisyUI (component library on top of Tailwind)

## Development & Tooling
- Asset Bundling: trunk or webpack (for Tailwind and DaisyUI)
- Database Migrations: sea-orm-cli
- Testing: tokio-test

## Deployment
- Containerization: Docker
- Orchestration: docker-compose

## Documentation
- API Docs: OpenAPI (Swagger) with utoipa