# Set Up
Create a `diesel.toml` file in the root directory. Paste this in there:
```
# For documentation on how to configure this file,
# see diesel.rs/guides/configuring-diesel-cli

[print_schema]
file = "src/schema.rs"
```
Set up Diesel CLI. Generates schema.rs and migrations for the database.
```
diesel setup
diesel migration generate <NAME_OF_DATABASE_MODEL>
diesel migration run
```
You should now see the folder /migrations. 
Inside is a folder with the Date as its name:
``` /2022-07-24-143544_accounts ```

Define SQL tables inside up.sql and drop those tables in down.sql.
Example.
```
# up.sql
CREATE TABLE accounts (
    id serial PRIMARY KEY NOT NULL,
    key VARCHAR NOT NULL,
    owner VARCHAR NOT NULL,
);
```
```
# down.sql
DROP TABLE accounts;
```

Start the server.
``` cargo run ```

# TODO

1) Connect to database created by Solana Geyser plugin.
2) Configure Geyser plugin to track a custom program (craft_skins).
3) #[test] a trx on craft_skins program, see that Geyser recorded it.
4) Create API to query accounts from database.
5) Create simple client to interface with accounts.
