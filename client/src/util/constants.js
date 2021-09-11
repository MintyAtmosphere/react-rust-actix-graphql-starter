export const APOLLO_CLIENT_URI = process.env.NODE_ENV === "production" ? "/graphql" : "http://localhost:3000/graphql";

export const MY_API = process.env.NODE_ENV === "production" ? "/api" : "http://localhost:3000/api";