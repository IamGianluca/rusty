## General Design

The service is built using a layered architecture. 

Layered architecture is a software architecture design pattern in which an application or system is organized into distinct layers or tiers, each responsible for a specific set of functionalities. This approach promotes modularity, separation of concerns, and maintainability. Here's an overview of the layers used in this application:

1. **Application Layer (Business Logic Layer)**:
   - This layer contains the application's business logic and core functionality.
   - It processes requests from clients, performs necessary operations, and coordinates with the data layer.
   - Business rules, workflows, and application-specific logic reside here.

2. **Service Layer**:
   - It provides services that can be reused across different parts of the application, such as authentication, validation, or external integrations.

3. **Domain Layer**:
   - This layer encapsulates the core domain objects and their behaviors.
   - It defines the data model and business entities, as well as the operations that can be performed on them.
   - Domain logic and domain-specific rules are implemented here.

4. **Data Access Layer (Persistence Layer)**:
   - This layer is responsible for data storage and retrieval, such as accessing databases, file systems, or external APIs.
   - It abstracts the data storage details from the rest of the application.
   - Data access and data manipulation code, like SQL queries or object-relational mapping (ORM), are typically located here.
