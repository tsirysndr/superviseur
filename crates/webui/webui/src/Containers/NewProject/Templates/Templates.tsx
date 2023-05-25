import { FC } from "react";
import PostgreSQL from "../../../Images/postgresql.svg";
import Redis from "../../../Images/redis.svg";
import MongoDB from "../../../Images/mongodb.svg";
import MySQL from "../../../Images/mysql.svg";
import Fresh from "../../../Images/fresh.svg";
import NodeJS from "../../../Images/nodejs.svg";
import Go from "../../../Images/go.svg";
import { Stack } from "@styled-icons/octicons";
import Hono from "../../../Images/hono.png";
import Bun from "../../../Images/bun.svg";
import Deno from "../../../Images/deno.svg";
import Wasm from "../../../Images/wasm.svg";
import styled from "@emotion/styled";

const Action = styled.div`
  height: 50px;
  width: 100%;
  display: flex;
  align-items: center;
  padding-left: 20px;
  cursor: pointer;
  &:hover {
    color: #630be2;
    background-color: #fbfbfb;
  }
`;

const Logo = styled.img`
  height: 18px;
  margin-right: 15px;
`;

const Tag = styled.span`
  background-color: #650be214;
  color: #630be2;
  border-radius: 4px;
  font-size: 12px;
  padding: 2px 8px;
  margin-left: 12px;
`;

const Templates: FC = () => {
  return (
    <>
      <Action>
        <Stack color="#ff0a80" size={20} style={{ marginRight: 15 }} />
        <div>Empty Project</div>
      </Action>
      <Action>
        <Logo src={Fresh} />
        <div>Deno Fresh</div>
        <Tag>flox</Tag>
      </Action>
      <Action>
        <Logo src={Hono} />
        <div>Hono</div>
        <Tag>flox</Tag>
      </Action>
      <Action>
        <Logo src={Bun} />
        <div>Bun Server</div>
        <Tag>nix</Tag>
      </Action>
      <Action>
        <Logo src={NodeJS} />
        <div>NodeJS, Redis</div>
        <Tag>devenv</Tag>
      </Action>
      <Action>
        <Logo src={Go} />
        <div>Go, MySQL</div>
        <Tag>devbox</Tag>
      </Action>
      <Action>
        <Logo src={Wasm} />
        <div>Spin HTTP Server</div>
        <Tag>nix</Tag>
      </Action>
      <Action>
        <Logo src={Deno} />
        <div>Deno</div>
        <Tag>devbox</Tag>
      </Action>
      <Action>
        <Logo src={PostgreSQL} />
        <div>Provision PostgreSQL</div>
        <Tag>docker</Tag>
      </Action>
      <Action>
        <Logo src={Redis} />
        <div>Provision Redis</div>
        <Tag>docker</Tag>
      </Action>
      <Action>
        <Logo src={MongoDB} />
        <div>Provision MongoDB</div>
        <Tag>docker</Tag>
      </Action>
      <Action>
        <Logo src={MySQL} />
        <div>Provision MySQL</div>
        <Tag>docker</Tag>
      </Action>
    </>
  );
};

export default Templates;
