# Development

```
sea-orm-cli migrate generate -d packages/storage/shield-seaorm/src/migrations <name>

sea-orm-cli migrate fresh -u "sqlite:///tmp/shield-seaorm.sqlite?mode=rwc" -d ./examples/seaorm

sea-orm-cli generate entity -u "sqlite:///tmp/shield-seaorm.sqlite?mode=rwc" -o packages/storage/shield-seaorm/src/entities_template --with-serde both
```
