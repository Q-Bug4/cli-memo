# Requirement
Users can 
1. **save** commands to memos from multi data source like history and files.
2. **run** memos to do something like batch rename files.
3. **check** saved memos.
4. **manage(modify, delete, ...)** memos.

## Memo

### structure
```typescript
memo = {
    name: String,
    language: String,
    content: String,
    source_type: InputSourceEnum,
    result_type: OutputResultEnum,
}
```

## Data source
1. files
2. pick from history
3. files
4. user's keyboard input

## Storage
rust sqlite
