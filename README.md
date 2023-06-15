# api
L'API du réseau social

# Sommaire
- [Lancer l'API](#1-lancer-lapi)
- [Utilisation](#2-utilisation)
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

Renvoie :
    - Code 200 et la liste des utilisateurs au format JSON
    - Code 500 lors d'une erreur serveur

## Créer un nouvel utilisateur
Requête : `POST /register`

Body (JSON) :
- username => Chaîne de caractères
- password => Chaîne de caractères
- email => Chaîne de caractères
- is_male (facultatif) => booléen

Renvoie :
    - Code 201 et le token sous forme de cookie
    - Code 403 lorsque les indentifiants sont invalides ou déjà utilisés

## Se connecter
Requête : `POST /login`

Body (JSON) :
- username_or_email => Chaîne de caractères
- password => Chaîne de caractères

Renvoie :
    - Code 200 et le token sous forme de cookie
    - Code 403 lorsque les indentifiants sont incorrects