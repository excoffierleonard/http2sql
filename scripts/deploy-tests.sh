#!/bin/bash

set -e

# Register a new user
curl --request POST \
     --url 'http://localhost:8080/v1/auth/register' \
     --header 'Content-Type: application/json' \
     --data '{
           "email": "john.doe@gmail.com",
           "password": "Randompassword1!"
      }'

# Login the user
curl --request POST \
     --url 'http://localhost:8080/v1/auth/login' \
     --header 'Content-Type: application/json' \
     --data '{
           "email": "john.doe@gmail.com",
           "password": "Randompassword1!"
      }'

# Fetch the user table
curl --request GET \
     --url http://localhost:8080/v1/users