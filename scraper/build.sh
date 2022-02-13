#!/bin/bash
NAME=thvote-scraper
docker build --no-cache -t ${NAME} .
docker save -o ${NAME}.tar ${NAME}
