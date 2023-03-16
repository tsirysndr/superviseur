import {
  ApolloClient,
  ApolloProvider,
  createHttpLink,
  InMemoryCache,
  split,
} from "@apollo/client";
import { WebSocketLink } from "@apollo/client/link/ws";
import { MockedProvider } from "@apollo/client/testing";
import { getMainDefinition } from "@apollo/client/utilities";
import { FC } from "react";
import { SubscriptionClient } from "subscriptions-transport-ws";
import { mocks } from "../Mocks";

const uri =
  process.env.NODE_ENV === "development"
    ? process.env.REACT_APP_API_URL || "http://localhost:5478"
    : `${origin}/graphql`;

const httpLink = createHttpLink({
  uri,
});

const wsLink = new WebSocketLink(
  new SubscriptionClient(uri.replace("http", "ws"))
);

const link = split(
  ({ query }) => {
    const definition = getMainDefinition(query);
    return (
      definition.kind === "OperationDefinition" &&
      definition.operation === "subscription"
    );
  },
  wsLink,
  httpLink
);

const client = new ApolloClient({
  link,
  cache: new InMemoryCache(),
});

export type ProviderProps = {
  children: React.ReactNode;
};

const Provider: FC<ProviderProps> = ({ children }) => {
  if (process.env.NODE_ENV === "development") {
    return (
      <MockedProvider mocks={mocks} addTypename={true}>
        {children}
      </MockedProvider>
    );
  }
  return <ApolloProvider client={client}>{children}</ApolloProvider>;
};

export default Provider;
