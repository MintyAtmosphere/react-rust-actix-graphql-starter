import React from "react";
import axios from "axios";
import logo from "./logo.svg";
import { ApolloClient, InMemoryCache, ApolloProvider } from "@apollo/client";
import "./App.css";
import Header from "./Header";

import { APOLLO_CLIENT_URI, MY_API } from "./util/constants";

const apolloClient = new ApolloClient({
  uri: APOLLO_CLIENT_URI,
  cache: new InMemoryCache(),
});

class App extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      isReadingFromRust: "failed",
    };
  }

  componentDidMount() {
    axios.get(MY_API).then((response) => {
      // handle success
      const {
        data: { status },
      } = response;
      console.log(status);
      this.setState({
        isReadingFromRust: status,
      });
    });
  }

  render() {
    return (
      <ApolloProvider client={apolloClient}>
        <Header />
        Hello {this.props.name}! This is React Frontend
        <h1>test</h1>
        <h3>Status reading from Rust server: {this.state.isReadingFromRust}</h3>
      </ApolloProvider>
    );
  }
}

export default App;
