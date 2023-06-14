# api
L'API du forum

# 1. Installation de SurrealDB
Suivez les indications à ce lien : https://surrealdb.com/install

# 2. Lancez SurrealDB
Pour une base de données stockée en mémoire faites :
```rust
surreal start --log debug --user root --pass root memory
```

Pour plus d'informations, vous pouvez consulter https://surrealdb.com/docs/installation

# 3. Lancer l'API
Rien de plus simple !
```bash
cargo run
```