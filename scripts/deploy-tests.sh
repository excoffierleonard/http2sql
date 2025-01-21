#!/bin/bash

set -e

# Register a new user
curl --request POST \
     --url 'http://localhost:8080/v1/auth/sign-up' \
     --header 'Content-Type: application/json' \
     --data '{
           "email": "luke.warm@hotmail.fr",
           "password": "Randompassword2!"
      }'

# Login the user
curl --request POST \
     --url 'http://localhost:8080/v1/auth/sign-in' \
     --header 'Content-Type: application/json' \
     --data '{
           "email": "luke.warm@hotmail.fr",
           "password": "Randompassword2!"
      }'

# Fetch the user table
curl --request GET \
     --url http://localhost:8080/v1/users

# Create a tag
curl --request POST \
     --url 'http://localhost:8080/v1/tags' \
     --header 'Content-Type: application/json' \
     --data '{
           "api_key": "ak_prod_kOYoM5SeT+M3LqWdClwWZO0/E9Fogg63wGUxTuolMNQ=",
           "name": "tag3"
      }'