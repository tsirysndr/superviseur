import { FC } from "react";
import styled from "@emotion/styled";
import { Stack } from "@styled-icons/octicons";

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

export type MessagingProps = {
  data: any[];
};

const Messaging: FC<MessagingProps> = ({ data }) => {
  return (
    <>
      {data.map((template) => (
        <Action key={template.id} onClick={() => {}}>
          {template.icon && <Logo src={template.icon} />}
          {!template.icon && (
            <Stack color="#ff0a80" size={20} style={{ marginRight: 15 }} />
          )}
          <div>{template.name}</div>
        </Action>
      ))}
    </>
  );
};

export default Messaging;
