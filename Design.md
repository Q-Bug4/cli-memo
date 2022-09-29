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
    content: String,
    language: Language,
    source_type: InputSourceEnum,
    result_type: OutputResultEnum,
}
```

## Data source
1. files
2. filesystem
3. pick from history
4. text

## Storage
rust sqlite
