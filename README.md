# jetty-app-0182

A todo app built with Rust and Axum.

## Features

- Add, complete, and delete todos
- Filter by all / active / completed
- Clear all completed todos at once
- In-memory storage (resets on restart)

## Getting started

```bash
cargo run
```

Open [http://localhost:3000](http://localhost:3000) in your browser.

## API

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Health check (`{ "ok": true }`) |
| GET | `/api/todos` | List all todos |
| POST | `/api/todos` | Create todo (`{ "text": "..." }`) |
| PUT | `/api/todos/{id}` | Update todo (`{ "text"?, "completed"? }`) |
| DELETE | `/api/todos/{id}` | Delete todo |
