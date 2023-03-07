import { createLightTheme, createDarkTheme } from "baseui/themes";

export const BaseUIDarkTheme = createDarkTheme(
  {
    primaryFontFamily: "RockfordSansRegular",
  },
  {
    colors: {
      buttonPrimaryFill: "#ab28fc",
      buttonPrimaryText: "#fff",
      buttonPrimaryHover: "#ab28fcbe",
      buttonSecondaryFill: "rgba(171, 40, 252, 0.052)",
      buttonSecondaryText: "#ab28fc",
      buttonSecondaryHover: "rgba(171, 40, 252, 0.107)",
      buttonTertiaryText: "#fff",
      buttonTertiaryFill: "#000",
      buttonTertiaryHover: "#0000009700044",
    },
  }
);

export const BaseUILightTheme = createLightTheme(
  {
    primaryFontFamily: "RockfordSansRegular",
  },
  {
    colors: {
      buttonPrimaryFill: "#ab28fc",
      buttonPrimaryHover: "#ab28fcbe",
      buttonSecondaryFill: "rgba(171, 40, 252, 0.052)",
      buttonSecondaryText: "#ab28fc",
      buttonSecondaryHover: "rgba(171, 40, 252, 0.107)",
      buttonTertiaryText: "#fff",
      buttonTertiaryFill: "#000",
      buttonTertiaryHover: "#0000009700044",
    },
  }
);
