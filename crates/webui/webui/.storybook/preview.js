import { Client as Styletron } from "styletron-engine-atomic";
import { Provider as StyletronProvider } from "styletron-react";
import Providers from "../src/Providers";
import { MockedProvider } from "@apollo/client/testing";
import { mocks } from "../src/Mocks";
import "@fontsource/ubuntu";
import "reactflow/dist/style.css";
import "../src/index.css";

const engine = new Styletron();

export const parameters = {
  actions: { argTypesRegex: "^on[A-Z].*" },
  controls: {
    matchers: {
      color: /(background|color)$/i,
      date: /Date$/,
    },
  },
};

export const decorators = [
  (Story) => (
    <StyletronProvider value={engine}>
      <MockedProvider mocks={mocks} addTypename={true}>
        <Providers>
          <Story />
        </Providers>
      </MockedProvider>
    </StyletronProvider>
  ),
];
