#!/bin/bash

set -e

curl --request GET \
--url http://localhost:8080/v1/users

curl --request POST \
--url 'http://localhost:8080/v1/users' \
--header 'Content-Type: application/json' \
--data '{
  "data": [
    {
      "email": "john.doe@gmail.com",
      "password": "randompassword1"
    },
    {
      "email": "luke.warm@hotmail.fr", 
      "password": "randompassword2"
    }
  ]
}'