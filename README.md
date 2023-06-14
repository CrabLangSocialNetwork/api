# api
L'API du réseau social

# Sommaire
- [Installation et lancement de SurrealDB](#1-installation-et-lancement-de-surrealdb)
    - [Installation](#a-installation)
    - [Lancement](#b-lancement)
- [Lancer l'API](#2-lancer-lapi)
- [Utilisation](#3-utilisation)
    - [Liste des utilisateurs](#obtenir-la-liste-des-utilisateurs)
    - [Créer un utilisateur](#créer-un-nouvel-utilisateur)
    - [Se connecter](#se-connecter)

# 1. Installation et lancement de SurrealDB
## a. Installation
Suivez les indications à ce lien : https://surrealdb.com/install

## b. Lancement
Pour une base de données stockée en mémoire faites :
```bash
surreal start --log debug --user root --pass root memory
```

Pour plus d'informations, vous pouvez consulter https://surrealdb.com/docs/installation

# 2. Lancer l'API
Rien de plus simple !
```bash
cargo run
```

# 3. Utilisation

## Obtenir la liste des utilisateurs
Requête : `GET /users`

## Créer un nouvel utilisateur
Requête : `POST /register`

Body (JSON) :
- username => Chaîne de caractères
- password => Chaîne de caractères
- email => Chaîne de caractères
- is_male (facultatif) => booléen

## Se connecter
Requête : `POST /login`

Body (JSON) :
- username_or_email => Chaîne de caractères
- password => Chaîne de caractères