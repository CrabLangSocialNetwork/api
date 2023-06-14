# api
L'API du réseau social

# Sommaire
- [Lancer l'API](#2-lancer-lapi)
- [Utilisation](#3-utilisation)
    - [Liste des utilisateurs](#obtenir-la-liste-des-utilisateurs)
    - [Créer un utilisateur](#créer-un-nouvel-utilisateur)
    - [Se connecter](#se-connecter)

# 1. Lancer l'API
Rien de plus simple !
```bash
cargo run
```

# 2. Utilisation

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