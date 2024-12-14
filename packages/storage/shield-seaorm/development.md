# Development

Run in Shield root directory:

```shell
sea-orm-cli migrate generate -d packages/storage/shield-seaorm/src/migrations/<directory> <name>

docker compose up -d

sea-orm-cli migrate fresh -u "mysql://shield:shield@localhost:13306/shield" -d ./examples/seaorm
sea-orm-cli migrate fresh -u "postgres://shield:shield@localhost:15432/shield" -d ./examples/seaorm
sea-orm-cli migrate fresh -u "sqlite:///tmp/shield-seaorm.sqlite?mode=rwc" -d ./examples/seaorm

sea-orm-cli generate entity -u "mysql://shield:shield@localhost:13306/shield" -o packages/storage/shield-seaorm/src/entities_template/mysql --with-serde both
sea-orm-cli generate entity -u "postgres://shield:shield@localhost:15432/shield" -o packages/storage/shield-seaorm/src/entities_template/postgresql --with-serde both
sea-orm-cli generate entity -u "sqlite:///tmp/shield-seaorm.sqlite?mode=rwc" -o packages/storage/shield-seaorm/src/entities_template/sqlite --with-serde both
```
