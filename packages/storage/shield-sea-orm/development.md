# Development

Run in Shield root directory:

```shell
sea-orm-cli migrate generate -d packages/storage/shield-sea-orm/src/migrations/<directory> <name>

docker compose up -d

sea-orm-cli migrate fresh -u "mysql://shield:shield@localhost:13306/shield" -d ./examples/sea-orm
sea-orm-cli migrate fresh -u "postgres://shield:shield@localhost:15432/shield" -d ./examples/sea-orm
sea-orm-cli migrate fresh -u "sqlite:///tmp/shield-sea-orm.sqlite?mode=rwc" -d ./examples/sea-orm

sea-orm-cli generate entity -u "mysql://shield:shield@localhost:13306/shield" -o packages/storage/shield-sea-orm/src/entities_template/mysql --with-serde both
sea-orm-cli generate entity -u "postgres://shield:shield@localhost:15432/shield" -o packages/storage/shield-sea-orm/src/entities_template/postgresql --with-serde both
sea-orm-cli generate entity -u "sqlite:///tmp/shield-sea-orm.sqlite?mode=rwc" -o packages/storage/shield-sea-orm/src/entities_template/sqlite --with-serde both
```
