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
          <div style={{ flex: 1, minWidth: 603 }}>
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
              <span className="featureText aqua-blue">multi-service</span>{" "}
              applications on{" "}
              <span className="featureText pink-magenta">isolated</span>{" "}
              environments with{" "}
              <span className="featureText pink-magenta">Nix</span> or{" "}
              <span className="featureText pink-magenta">Docker</span> using{" "}
              <span className="featureText purple-indigo">HCL</span> or{" "}
              <span className="featureText purple-indigo">any language</span>{" "}
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
              <span className="featureText aqua-blue">
                Docker, Nix, Spin, WebAssembly
              </span>{" "}
              and <span className="featureText aqua-blue">more</span>.
            </div>
          </div>
          <div style={{ flex: 1, minWidth: 630 }}>
            <SupportedPlugins />
          </div>
        </div>
      </section>
    </>
  );
}
