```plantuml
  classDiagram
    class User {
        +id: Uuid
        +username: String
        +email: String
        +password_hash: String
        +created_at: DateTime
        +updated_at: DateTime
    }
    class Task {
        +id: Uuid
        +title: String
        +description: String
        +status: TaskStatus
        +due_date: Option~DateTime~
        +user_id: Uuid
        +category_id: Option~Uuid~
        +created_at: DateTime
        +updated_at: DateTime
    }
    class Category {
        +id: Uuid
        +name: String
        +user_id: Uuid
        +created_at: DateTime
        +updated_at: DateTime
    }
    class TaskStatus {
        <<enumeration>>
        TODO
        IN_PROGRESS
        DONE
    }
    User "1" -- "*" Task : has
    User "1" -- "*" Category : has
    Category "0..1" -- "*" Task : contains
```