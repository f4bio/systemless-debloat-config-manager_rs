# build rust/wasm
FROM rust:latest

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
WORKDIR /app
COPY . .

RUN wasm-pack build

# build node/webpack
FROM node:latest

WORKDIR /app
COPY . .
COPY --from=0 /app/pkg /app/pkg

RUN npm clean-install
RUN npm run build:prod

# put it all together
FROM nginx:alpine

LABEL maintainer="Fabio Tea <iam@f4b.io>"
LABEL version="0.0.1"

COPY ./nginx_mime.types /etc/nginx/mime.types
COPY --from=1 /app/dist /usr/share/nginx/html
