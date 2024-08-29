## To-Do App Architecture
1. Layer Overview

Presentation Layer: Axum handlers and Askama templates
Application Layer: Business logic and use cases
Domain Layer: Core business entities and rules
Infrastructure Layer: Database interactions (Sea-ORM) and external services

migrations/
tests/
2. Component Details
2.1 Presentation Layer

Routes: Define API endpoints
Handlers: Handle HTTP requests, invoke services, return responses
Templates: Askama HTML templates for server-side rendering

2.2 Application Layer

Services: Implement business logic, orchestrate operations

2.3 Domain Layer

Models: Define core business entities and their behaviors

2.4 Infrastructure Layer

Repositories: Implement data access using Sea-ORM
Context: Manage application context, including database connections and transactions

3. Data Flow

HTTP Request → Routes → Handlers
Handlers → Services
Services → Repositories / Models
Repositories ↔ Context ↔ Database
Services → Handlers
Handlers → Templates (for HTML responses)
HTTP Response

4. Key Components

Authentication Middleware
Error Handling
Logging
Configuration Management