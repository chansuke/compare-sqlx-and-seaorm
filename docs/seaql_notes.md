# seaQL メモ
## 作業メモ
- `.env` or `.envrc` を用意し `DATABASE_URL` の値を設定

- `.sql` を用意しマイグレーションを走らせる → `direnv allow .` して `sqlx migrate run`
-> 成功

- 中身を見てみる

```
                        Table "public.activities"
   Column    |           Type           | Collation | Nullable | Default 
-------------+--------------------------+-----------+----------+---------
 uuid        | character varying(50)    |           | not null | 
 name        | text                     |           | not null | 
 category_id | text                     |           | not null | 
 created_at  | timestamp with time zone |           | not null | 
 updated_at  | timestamp with time zone |           | not null | 
 deleted_at  | timestamp with time zone |           | not null | 
Indexes:
    "activities_pkey" PRIMARY KEY, btree (uuid)
```

- https://github.com/SeaQL/sea-orm/blob/689e0075b78455dcf43d4476ef6a0f3a9813599a/ARCHITECTURE.md

- `sea-orm-cli` を叩いてみる
  - `$ sea-orm-cli -h`
  - `$ sea-orm-cli generate -h`
  - `$ sea-orm-cli generate entity -h`

- `--compact-format(default:true)`
  - https://www.sea-ql.org/SeaORM/docs/generate-entity/entity-structure/

- `--expanded-format`
  - https://www.sea-ql.org/SeaORM/docs/generate-entity/expanded-entity-structure/

- 新しくマイグレーションを追加しentityを生成すると ``

## コマンド
### マイグレーションファイル追加
`$ sqlx migrate add <name>`

###
`$ sqlx migrate run`

### entity生成
`$ sea-orm-cli generate entity -u $DATABASE_URL -o sea_ql/src/entity_expanded/ --expanded-format`
