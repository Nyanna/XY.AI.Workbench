# cgen - OpenAPI 3.1 to Code Generator

A type-safe code generator that transforms OpenAPI 3.1 YAML specifications into well-structured, production-ready source code. The target language is selected via `--language`, which picks the corresponding `templates/<language>` directory.

## Overview

**cgen** is a code generation tool designed to bridge the gap between API specifications and code implementation. It reads OpenAPI 3.1 schemas and generates corresponding models, clients, and supporting code with full type safety and zero boilerplate.

### Key Features

- 🎯 **Type-Safe**: Generates fully typed code with strong compile-time safety guarantees
- 📋 **OpenAPI 3.1 Support**: Comprehensive support for OpenAPI 3.1 YAML specifications
- 🏗️ **Modular Architecture**: Clean separation of concerns through a multi-stage pipeline
- 🔄 **Lossless Processing**: Preserves semantic information through schema ingestion, modeling, and naming stages

## Concept

The generator operates through a well-defined, multi-stage pipeline:

```
OpenAPI YAML Schema
       ↓
   [Ingest] ─────────────── Parse and validate OpenAPI specification
       ↓
   [Model] ─────────────── Build internal representation of API structure
       ↓
  [Identity] ─────────────── Compute semantic identities and resolve references
       ↓
  [Optimize] ────────────── Optimize identity and eliminate redundancy
       ↓
  [Naming] ─────────────── Assign compatible names and package hierarchy
       ↓
   [Emit] ─────────────── Generate and write source files
       ↓
   Source Code
```

### Pipeline Stages

1. **Ingest**: Reads and parses the OpenAPI 3.1 YAML schema, validating its structure
2. **Model**: Constructs an internal domain model representing the API's types and operations
3. **Identity**: Computes semantic identities and resolves schema references and definitions
4. **Optimize**: Deduplicates and optimizes the identified model for efficient code generation
5. **Naming**: Maps internal identifiers to compliant names and determines package structure
6. **Emit**: Generates and writes the final source files to the output directory

## Usage

### Command-Line Interface

```bash
cgen --schema <path-to-openapi.yaml> --out <output-directory> [--base-package <package.name>] [--language java|php|python|rust|typescript]
```

### Arguments

- `--schema` (required): Path to the OpenAPI 3.1 YAML schema file
- `--out` (required): Output directory where generated Java sources will be written
- `--base-package` (optional): Root package for generated code
- `--language` (optional): Target language, `java`, `php`, `python`, `rust` or `typescript` (default: `java`); selects the `templates/<language>` directory

### TypeScript target

`--language typescript` generates `.ts` files that need no runtime dependency: the client uses the global `fetch`, the server `node:http`.

- **Runtime**: Node >= 22.18 (or >= 23.6) runs the output directly via type stripping; the generated code only uses erasable syntax (no `enum`, no parameter properties), so any other TypeScript toolchain works as well.
- **Modules**: ESM only, relative imports carry the `.ts` extension. The output directory must lie within a package with `"type": "module"`. For `tsc` use `module`/`moduleResolution: nodenext` with `allowImportingTsExtensions`.
- **Client**: `XxxClientImpl(baseUrl, fetchImpl = fetch)`; override `customizeRequest(request)` to set e.g. `Authorization`. All operation methods return a `Promise`.
- **Server**: extend the abstract `XxxServer`, implement the operation methods (same signatures as the client interface) and call `start(port)`, or use `handle(req, res)` as a request listener of your own `node:http` server.
- **Models**: proxies over the parsed JSON value with `getX()`/`setX()` returning `T | null`; enums are `const` objects plus a union type of the same name.
- Parameter names that are TypeScript-only reserved words (e.g. `delete`, `function`, `in`) are not escaped.

## Project Structure

```
codegen/
├── src/
│   └── xy/cgen/
│       ├── __init__.py           # Package initialization
│       ├── __main__.py           # Module entry point
│       ├── cli.py                # Command-line argument parsing
│       ├── config.py             # Configuration management
│       ├── pipeline.py           # Pipeline orchestration
│       ├── naming/               # Naming and package mapping
│       │   ├── identifiers.py    # identifier generation
│       │   ├── names.py          # Name resolution
│       │   ├── packages.py       # Package hierarchy mapping
│       │   ├── paths.py          # Path-based naming
│       │   └── traverse.py       # Model traversal utilities
│       ├── typemap/              # Type mapping configuration
│       ├── emit/                 # Code emission
│       │   ├── model_emit.py     # Model class generation
│       │   ├── client_emit.py    # API client generation
│       │   ├── model_context.py  # Model context management
│       │   ├── client_context.py # Client context management
│       │   ├── io_context.py     # I/O context and file writing
│       │   └── writer.py         # File writing utilities
│       ├── model/                # Internal domain modeling
│       ├── identity/             # Semantic identity computation
│       ├── ingest/               # OpenAPI schema ingestion
│       └── naming/               # naming conventions
├── pyproject.toml               # Project metadata and dependencies
├── .gitignore                   # Git configuration
└── README.md                    # This file
```

## Architecture

### Internal Data Flow

The generator maintains a clean separation between:

- **Parsing**: OpenAPI specification → raw AST
- **Modeling**: Raw AST → strongly-typed domain model
- **Semantic Analysis**: Domain model → identified/optimized representations
- **Naming**: Semantic model → Compatible names and package structure
- **Emission**: Named model → Source files

Each stage is independent and can be evolved, tested, or replaced without affecting others.

## Development

### Running from Source

```bash
export PYTHONPATH=src
python3 -m xy.cgen --schema path/to/schema.yaml --out output --base-package com.example
```