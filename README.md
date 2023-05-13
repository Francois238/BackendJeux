# BackendJeux 
Backend en rust du snake
## Installation 
### BDD avec postgreSQL  
Creation d'une nouvelle database : `CREATE DATABASE jeux;`  
Creation d'un utilisateur dans postgreSQL : `CREATE USER joueur WITH PASSWORD 'password';` (veuillez à mettre un mot de passe fort)  
Création des droits pour cet utilisateur : `GRANT ALL PRIVILEGES ON DATABASE jeux TO joueur;`  
Modifiez la 1ère ligne du fichier .env en remplacer le mot de passe 'password' par votre mot de passe  
### Création de la BDD avec diesel 
Installer diesel : `cargo install diesel_cli`  
Entrez : `diesel migration run`
 
## Lancement du programme  
Merci de bien changer les mots de passes dans le fichier .env  
Entrez `cargo run`
