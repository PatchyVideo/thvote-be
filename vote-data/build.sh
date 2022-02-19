#!/bin/bash
NAME=thvote-vote-data
cp ../target/x86_64-unknown-linux-musl/release/${NAME} tmp.app
docker build --no-cache -t ${NAME} .
docker save -o ${NAME}.tar ${NAME}
rm tmp.app
mv ${NAME}.tar ../