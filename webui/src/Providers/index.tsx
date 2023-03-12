import { FC } from "react";
import ThemeProvider from "./ThemeProvider";
import { MockedProvider } from "@apollo/client/testing";

export type ProvidersProps = {
  children: React.ReactNode;
};

const Providers: FC<ProvidersProps> = ({ children }) => {
  return (
    <MockedProvider mocks={[]} addTypename={false}>
      <ThemeProvider>{children}</ThemeProvider>
    </MockedProvider>
  );
};

export default Providers;
