import React from "react";
import clsx from "clsx";
import "prismjs/themes/prism-dark.css";
import "prismjs/components/prism-hcl";
import "prismjs/components/prism-typescript";
import styles from "./styles.module.css";
import CodeExample from "./CodeExample.mdx";
import SupportedPlugins from "./SupportedPlugins.mdx";

function Feature({ title, Svg, description }: FeatureItem) {
  return (
    <div className={clsx("col col--4")}>
      <div className="text--center">
        <Svg className={styles.featureSvg} role="img" />
      </div>
      <div className="text--center padding-horiz--md">
        <h3>{title}</h3>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): JSX.Element {
  return (
    <>
      <section className={styles.features}>
        <div
          className="container"
          style={{ display: "flex", flexDirection: "row" }}
        >
          <div style={{ flex: 1 }}>
            <CodeExample />
          </div>
          <div
            style={{
              display: "flex",
              alignItems: "start",
              flex: 1,
              paddingLeft: 40,
              marginTop: 100,
            }}
          >
            <div style={{ fontSize: "22px" }}>
              Define and run{" "}
              <span
                style={{ color: "#00ffed", fontFamily: "RockfordSans Bold" }}
              >
                multi-service
              </span>{" "}
              applications on{" "}
              <span
                style={{ color: "#f800ff", fontFamily: "RockfordSans Bold" }}
              >
                isolated
              </span>{" "}
              environments with{" "}
              <span
                style={{ color: "#f800ff", fontFamily: "RockfordSans Bold" }}
              >
                Nix
              </span>{" "}
              or{" "}
              <span
                style={{ color: "#f800ff", fontFamily: "RockfordSans Bold" }}
              >
                Docker
              </span>{" "}
              using{" "}
              <span
                style={{ color: "#8900ff", fontFamily: "RockfordSans Bold" }}
              >
                HCL
              </span>{" "}
              or{" "}
              <span
                style={{ color: "#8900ff", fontFamily: "RockfordSans Bold" }}
              >
                any language
              </span>{" "}
              you already know.
            </div>
          </div>
        </div>
      </section>
      <section className={styles.features}>
        <div
          className="container"
          style={{ display: "flex", flexDirection: "row" }}
        >
          <div
            style={{
              display: "flex",
              alignItems: "start",
              flex: 1,
              paddingRight: 100,
              marginTop: 100,
            }}
          >
            <div style={{ fontSize: "22px" }}>
              Support many different environments and runtimes, including{" "}
              <span style={{ color: "#3fe", fontFamily: "RockfordSans Bold" }}>
                Docker, Nix, Spin, WebAssembly
              </span>{" "}
              and{" "}
              <span style={{ color: "#3fe", fontFamily: "RockfordSans Bold" }}>
                more
              </span>
              .
            </div>
          </div>
          <div style={{ flex: 1 }}>
            <SupportedPlugins />
          </div>
        </div>
      </section>
    </>
  );
}
