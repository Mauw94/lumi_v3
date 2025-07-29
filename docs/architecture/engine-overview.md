## High-Level Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Source Code   │───▶│   Lexical       │───▶│   Syntax        │
│   (JavaScript)  │    │   Analysis      │    │   Analysis      │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                │                       │
                                ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Bytecode      │◀───│   Semantic      │◀───│   Abstract      │
│   Execution     │    │   Analysis      │    │   Syntax Tree   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                │
                                ▼
                       ┌─────────────────┐
                       │   Runtime       │
                       │   Environment   │
                       └─────────────────┘
```


## Core Components

### 1. Lexical Analysus (lumi_lexer)
- **Purpose**: Converts source code into tokens
- **Features**:
    - Unicode support - **Not yet implemented**
    - Precise position tracking
    - Error handling and recovery