# ferrum-vault
A local password manager written in Rust

## What can be stored:

- id
- name
- website / app
- username / email
- password
- notes


## API Endpoints

```text

# Account
POST /api/account
POST /api/account/register  {name, email, password}

# Authentication
POST /api/auth/2fa  {prelim_token}
POST /api/auth/signin  {email, password, remember}
POST /api/auth/challenge  {session_token}
POST /api/auth/logout  {session_token}

# Vault
/api/vault
/api/vault/passwords  {session_token, challenge_token}
/

/api/authenticator

```
