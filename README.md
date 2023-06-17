# api
L'API du réseau social

# Sommaire
- [Lancer l'API](#lancer-lapi)
- [Utilisation](#utilisation)
    - [Liste des utilisateurs](#obtenir-la-liste-des-utilisateurs)
    - [Créer un utilisateur](#créer-un-nouvel-utilisateur)
    - [Se connecter](#se-connecter)
    - [Créer un post](#créer-un-post)

# Lancer l'API
Rien de plus simple !
```bash
cargo run
```

# Utilisation

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
- Code 403 et l'erreur sous forme de chaîne de caractères lorsque les indentifiants sont invalides ou déjà utilisés

## Se connecter
Requête : `POST /login`

Body (JSON) :
- username_or_email => Chaîne de caractères
- password => Chaîne de caractères

Renvoie :
- Code 200 et le token sous forme de cookie
- Code 403 lorsque les indentifiants sont incorrects

## Créer un post
Requête : `POST /post`

Body (JSON) :
- content => Chaîne de caractères avec une longueure maximale de 500 caractères
- images (facultatif) => tableau d'images encodée en base64 (en chaînes de caractères)

Renvoie :
- Code 201 et le message de succès
- Code 403 lors d'une erreur (non connecté, post trop long) et le message d'erreur
- Code 500 lors d'une erreur serveur et le message d'erreur