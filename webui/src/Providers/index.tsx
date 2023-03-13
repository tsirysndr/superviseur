import { FC } from "react";
import ThemeProvider from "./ThemeProvider";
import { MockedProvider } from "@apollo/client/testing";
import { mocks } from "../Mocks";

export type ProvidersProps = {
  children: React.ReactNode;
};

const Providers: FC<ProvidersProps> = ({ children }) => {
  return (
    <MockedProvider mocks={mocks} addTypename={true}>
      <ThemeProvider>{children}</ThemeProvider>
    </MockedProvider>
  );
};

export default Providers;
