import { BaseProvider } from "baseui";
import { BaseUILightTheme } from "../Theme";
import { FC } from "react";

export type ThemeProviderProps = {
  children: React.ReactNode;
};

const ThemeProvider: FC<ThemeProviderProps> = ({ children }) => {
  return <BaseProvider theme={BaseUILightTheme}>{children}</BaseProvider>;
};

export default ThemeProvider;
