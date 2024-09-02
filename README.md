# Clean Arch Rust

This project is a study application created to explore Clean Architecture principles in Rust. It is based on the tutorial provided in this video: [Building a Clean Architecture in Rust](https://youtu.be/8GN7KBz2kAI). The project aims to demonstrate how to structure a Rust application following clean architectural practices, utilizing Actix-web for the web framework and Diesel for ORM with PostgreSQL.


## Requirements

- [Rust](https://www.rust-lang.org/)
- [Docker](https://www.docker.com/)
- [Docker Compose](https://docs.docker.com/compose/)
- [PostgreSQL](https://www.postgresql.org/) (or use Docker Compose to set it up automatically)

## Setup

1. **Clone the repository:**

    ```bash
    git clone https://github.com/loanmatteusz/clean-arch-rust.git
    cd clean-arch-rust
    ```

2. **Set up the database using Docker Compose:**

    ```bash
    docker-compose up -d
    ```

3. **Configure environment variables:**

   Create a `.env` file in the root of the project with the following content:

    ```env
    DATABASE_URL=postgres://admin:admin@localhost:5434/user_db
    ```

4. **Install project dependencies:**

    ```bash
    cargo build
    ```

5. **Run database migrations:**

    ```bash
    diesel migration run
    ```

   Make sure you have Diesel CLI installed:

    ```bash
    cargo install diesel_cli --no-default-features --features postgres
    ```

## Running the Application

1. **Start the server:**

    ```bash
    cargo run
    ```

2. **API Endpoints:**

    - `POST /api/v1/users/`: Register a new user.
    - `GET /api/v1/users/{email}`: Retrieve a user by email.

## Testing

Use tools like [Insomnia](https://insomnia.rest/) or [Postman](https://www.postman.com/) to test the endpoints.

Example JSON body for registering a new user:

```json
{
    "name": "User Test",
    "email": "usertest@example.com",
    "phone": "1234567890",
    "address": "User's Street"
}
```
