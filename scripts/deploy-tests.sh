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
           "user_uuid": "68a373e4-c8d7-4449-8e63-0f216a59fd0e",
           "name": "tag3"
      }'