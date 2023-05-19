import React from "react";
import clsx from "clsx";
import "prismjs/themes/prism-dark.css";
import "prismjs/components/prism-hcl";
import "prismjs/components/prism-typescript";
import styles from "./styles.module.css";
import CodeExample from "./CodeExample.mdx";

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
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          <CodeExample />
        </div>
      </div>
    </section>
  );
}
