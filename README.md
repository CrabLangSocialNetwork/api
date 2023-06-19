# api
L'API du réseau social

# Sommaire
- [Lancer l'API](#lancer-lapi)
- [Utilisation](#utilisation)
    - Auth
        - [Créer un utilisateur](#créer-un-nouvel-utilisateur)
        - [Se connecter](#se-connecter)
    - Utilisateurs
        - [Liste des utilisateurs](#obtenir-la-liste-des-utilisateurs)
    - Posts
        - [Obtenir la liste des posts](#obtenir-la-liste-des-posts)
        - [Créer un post](#créer-un-post)
        
# Lancer l'API
Rien de plus simple !
```bash
cargo run
```

# Utilisation

## Auth
### Créer un nouvel utilisateur
Requête : `POST /register`

Body (JSON) :
- username => Chaîne de caractères
- password => Chaîne de caractères
- email => Chaîne de caractères
- is_male (facultatif) => booléen

Renvoie :
- Code 201 et le token sous forme de cookie
- Code 403 et l'erreur sous forme de chaîne de caractères lorsque les indentifiants sont invalides ou déjà utilisés

### Se connecter
Requête : `POST /login`

Body (JSON) :
- username_or_email => Chaîne de caractères
- password => Chaîne de caractères

Renvoie :
- Code 200 et le token sous forme de cookie
- Code 403 lorsque les indentifiants sont incorrects

## Utilisateurs
### Obtenir la liste des utilisateurs
Requête : `GET /users`

Renvoie :
- Code 200 et la liste des utilisateurs au format JSON
- Code 500 lors d'une erreur serveur

## Posts

### Obtenir la liste des posts
**Authentification facultative (par cookie de session)**
Requête : `GET /posts`

Renvoie :
- Code 200 et la liste des posts sous format JSON
- Code 500 lors d'une erreur serveur

### Créer un post
**Authentification nécessaire (par cookie de session)**
Requête : `POST /post`

Body (JSON) :
- content => Chaîne de caractères d'une longueure maximale de 500 caractères
- images (facultatif) => tableau d'images encodée en base64 (en chaînes de caractères)

Renvoie :
- Code 201 et le message de succès
- Code 403 lors d'une erreur (non connecté, post trop long) et le message d'erreur
- Code 500 lors d'une erreur serveur et le message d'erreur

### Modifier un post
**Authentification nécessaire (par cookie de session)**
Requête : `PUT /posts/:id` avec id => ID du post

Body (JSON) :
- content => Chaîne de caractères d'une longueure maximale de 500 caractères

Renvoie :
- Code 201 et le message de succès
- Code 403 lors d'une erreur et le message d'erreur