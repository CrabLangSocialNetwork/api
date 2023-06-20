# api
L'API du réseau social

# Sommaire
- [Lancer l'API](#lancer-lapi)
- [Utilisation](#utilisation)
    - [Auth](#auth)
        - [Créer un utilisateur](#créer-un-nouvel-utilisateur)
        - [Se connecter](#se-connecter)
    - [Utilisateurs](#utilisateurs)
        - [Liste des utilisateurs](#obtenir-la-liste-des-utilisateurs)
        - [Modifier un utilisateur](#modifier-un-utilisateur)
    - [Posts](#posts)
        - [Obtenir la liste des posts](#obtenir-la-liste-des-posts)
        - [Obtenir la liste des posts d'un utilisateur](#obtenir-la-liste-des-posts-dun-utilisateur)
        - [Créer un post](#créer-un-post)
        - [Modifier un post](#modifier-un-post)
        - [Supprimer un post](#supprimer-un-post)
        
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
- Code 200 et la liste des utilisateurs au format JSON :
    - username => chaîne de caractères
    - is_male => booléen pouvant être null
    - created_at => Date au format Utc (temps universel coordonné) sous forme de chaîne de caractères
    - updated_at => Date au format Utc (temps universel coordonné) sous forme de chaîne de caractères
- Code 500 lors d'une erreur serveur

### Modifier un utilisateur
**Authentification nécessaire (par cookie de session)**

Requête : `PUT /@:username`

Body (JSON) :
- email (optionnel) => chaîne de caractères (email valide)
- username (optionel) => chaîne de caractères (au moins 5 caractères, commençant par une lettre, uniquement composé de lettres, nombres et underscores)
- password (optionel) => chaîne de caractères (au moins 8 caractères)

Renvoie :
- Code 200 et le message de succès
- Code 304 lorsqu'aucune modification n'est nécessaire
- Code 403 lorsque l'utilisateur n'a pas la permission de modifier cet utilisateur
- Code 404 lorsque le l'utilisateur n'est pas trouvé
- Code 500 lors d'une erreur serveur

## Posts

### Obtenir la liste des posts
**Authentification facultative (par cookie de session)**

Requête : `GET /posts`

Renvoie :
- Code 200 et la liste des posts au format JSON
    - id: chaîne de caractères,
    - content: chaîne de caractères,
    - images: Tableau d'URL d'images (chaîne de caractères),
    - author: {
        username: chaîne de caractères,
        permission_level: chaîne de caractères (Guest, User, Moderator ou Administrator)
    },
    - has_permission: booléen,
    - created_at: Date au format Utc (temps universel coordonné) sous forme de chaîne de caractères,
    - updated_at: Date au format Utc (temps universel coordonné) sous forme de chaîne de caractères
- Code 500 lors d'une erreur serveur

### Obtenir la liste des posts d'un utilisateur
**Authentification facultative (par cookie de session)**

Requête : `GET /@:username/posts`

Renvoie :
- Code 200 et la liste des posts au format JSON
    - id: chaîne de caractères,
    - content: chaîne de caractères,
    - images: Tableau d'URL d'images (chaîne de caractères),
    - author: {
        username: chaîne de caractères,
        permission_level: chaîne de caractères (Guest, User, Moderator ou Administrator)
    },
    - has_permission: booléen,
    - created_at: Date au format Utc (temps universel coordonné) sous forme de chaîne de caractères,
    - updated_at: Date au format Utc (temps universel coordonné) sous forme de chaîne de caractères
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
- images_url => Tableau d'URL d'images (peut être vide)
- new_images (facultatif) => Tableau d'images encodées en base64

Renvoie :
- Code 200 et le message de succès
- Code 403 et le message d'erreur lors d'une erreur
- Code 404 et le message d'erreur lorsque le post n'existe pas
- Code 500 lors d'une erreur serveur

### Supprimer un post
**Authentification nécessaire (par cookie de session)**

Requête : `DELETE /posts/:id` avec id => ID du post

Body (JSON) :
- content => Chaîne de caractères d'une longueure maximale de 500 caractères
- images_url => Tableau d'URL d'images (peut être vide)
- new_images (facultatif) => Tableau d'images encodées en base64

Renvoie :
- Code 200 et le message de succès
- Code 403 et le message d'erreur lors d'une erreur
- Code 404 et le message d'erreur lorsque le post n'existe pas
- Code 500 lors d'une erreur serveur