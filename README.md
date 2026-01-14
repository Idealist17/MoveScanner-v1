# Sui Move to Neo4j Parser

This tool parses Sui Move projects (bytecode) and imports the structure (Modules, Structs, Functions, Calls) into a Neo4j Graph Database.


## Features

- **Bytecode Analysis**: Parses compiled Move bytecode (`.mv` files).
- **Knowledge Graph**: Exports a graph representation including:
  - **Modules**: Definitions and hierarchy.
  - **Structs**: Fields and Abilities (key, store, drop, copy).
  - **Functions**: Visibility, signatures, and call graph.
  - **Relationships**: Defines, Calls, etc.
- **Neo4j Import**: Automation script to load the graph directly into Neo4j using the Bolt protocol.
- **Project Narratives**: Supports multi-tenancy via project namespaces.

## Prerequisites

- **Rust**: For building the parser core.
- **Python 3.12+**: For the automation script.
- **uv**: Recommended for Python dependency management.
- **Sui CLI**: Required to build Move projects.
- **Neo4j**: Running instance (v4.4+ or v5).

## Installation

1.  **Build the Parser**:
    ```bash
    cargo build --release
    ```

2.  **Install Python Dependencies**:
    ```bash
    uv sync
    # Or manually involves:
    # pip install neo4j
    ```

## Usage

Use the provided Python script to build your Move project, scan it, and import to Neo4j.

```bash
uv run import_to_neo4j.py <PROJECT_PATH> \
    --project-name <UNIQUE_NAME> \
    --neo4j-uri bolt://localhost:7687 \
    --neo4j-user neo4j \
    --neo4j-pass <PASSWORD>
```

### Arguments

- `PROJECT_PATH`: Path to the Sui Move project root (containing `Move.toml`).
- `--project-name`: Unique identifier for this project in the graph (used for namespacing).
- `--output-dir`: Directory to store intermediate JSON artifacts.

### Example

```bash
uv run import_to_neo4j.py ./tests/test_project \
    --project-name MyDeFiProtocol \
    --neo4j-pass mysecret
```

## Architecture

1.  **Rust Core**: Scans bytecode and generates `output_graph.json`.
2.  **Python Script**: 
    - Builds the Move project via `sui move build`.
    - Invokes the Rust Core.
    - Reads `output_graph.json`.
    - Performs Cypher queries to `MERGE` nodes and relationships into Neo4j.
