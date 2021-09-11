# React Rust Starter Project

React + GraphQL (Apollo) on the frontend

Rust + GraphQL + Actix on the backend


## How to run:

In a development environment, the Actix server (Rust) is run on `localhost:3000` and the frontend React application is run on `localhost:8000`. Both dev servers need to be running before you can begin development. In the React application, any call to `localhost:8000/api` is proxied over to `localhost:3000/api`, with the help of `parcel-proxy-server` (see the file `client/proxy-dev-server.js`).

How to run the Actix server in watch mode:
```
cd server/
systemfd --no-pid -s http::3000 -- cargo watch -x run
```

How to run the React application:
```
cd client/
yarn start
```
