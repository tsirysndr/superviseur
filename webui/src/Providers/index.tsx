import { SnackbarProvider, PLACEMENT } from "baseui/snackbar";
import { FC } from "react";
import ThemeProvider from "./ThemeProvider";

export type ProvidersProps = {
  children: React.ReactNode;
};

const Providers: FC<ProvidersProps> = ({ children }) => {
  return (
    <SnackbarProvider placement={PLACEMENT.bottom}>
      <ThemeProvider>{children}</ThemeProvider>
    </SnackbarProvider>
  );
};

export default Providers;
