#!/bin/bash

concurrently "npx tailwindcss -i ./assets/main.css -o ./assets/tailwind.css --watch" "dx serve"