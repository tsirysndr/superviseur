import React from "react";
import clsx from "clsx";
import Link from "@docusaurus/Link";
import useDocusaurusContext from "@docusaurus/useDocusaurusContext";
import Layout from "@theme/Layout";
import HomepageFeatures from "@site/src/components/HomepageFeatures";
import Astronauts from "@site/static/img/astronauts.png";
import Waves from "@site/static/img/waves.svg";

import styles from "./index.module.css";

function HomepageHeader() {
  const { siteConfig } = useDocusaurusContext();
  return (
    <header className={clsx("hero hero--primary", styles.heroBanner)}>
      <div>
        <div
          style={{
            display: "flex",
            flexDirection: "row",
          }}
        >
          <div
            style={{
              display: "flex",
              flex: 1,
              height: "70vh",
              alignItems: "center",
            }}
          >
            <div
              style={{
                color: "#000",
                fontSize: "2rem",
                fontFamily: "RockfordSans Regular",
                padding: "0 4rem",
                textAlign: "left",
              }}
            >
              Compose your{" "}
              <span
                style={{
                  fontFamily: "RockfordSans Bold",
                  color: "#ffad62",
                }}
              >
                containerized
              </span>{" "}
              and{" "}
              <span
                style={{
                  fontFamily: "RockfordSans Bold",
                  color: "#ffad62",
                }}
              >
                non-containerized
              </span>{" "}
              <span
                style={{
                  fontFamily: "RockfordSans Bold",
                  color: "#3bcbce",
                }}
              >
                services
              </span>{" "}
              for localdev and deployments in{" "}
              <span
                style={{
                  fontFamily: "RockfordSans Bold",
                  color: "#f780fb",
                }}
              >
                HCL
              </span>{" "}
              or{" "}
              <span
                style={{
                  fontFamily: "RockfordSans Bold",
                  color: "#f780fb",
                }}
              >
                any language
              </span>{" "}
              with an{" "}
              <span
                style={{
                  fontFamily: "RockfordSans Bold",
                  color: "#f780fb",
                }}
              >
                SDK
              </span>
            </div>
          </div>
          <div style={{ flex: 1 }}>
            <img className="astronauts" src={Astronauts} alt="Astronauts" />
          </div>
        </div>
        <div className="home-waves">
          <Waves />
        </div>
      </div>
    </header>
  );
}

export default function Home(): JSX.Element {
  const { siteConfig } = useDocusaurusContext();
  return (
    <Layout
      title={`Superviseur`}
      description="Description will go into a meta tag in <head />"
    >
      <HomepageHeader />
      <main>
        <HomepageFeatures />
      </main>
    </Layout>
  );
}
