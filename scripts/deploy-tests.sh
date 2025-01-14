#!/bin/bash

set -e

# Fetch the user table
curl --request GET \
     --url http://localhost:8080/v1/users


# Register a new user
curl --request POST \
     --url 'http://localhost:8080/v1/auth/register' \
     --header 'Content-Type: application/json' \
     --data '{
       "data": 
         {
           "email": "john.doe@gmail.com",
           "password": "Randompassword1!"
         }
      }'