import { FC } from "react";
import styled from "@emotion/styled";
import { css } from "@emotion/react";

const Container = styled.div`
  background-color: #000;
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow-y: auto;
`;

const LineNumber = styled.span<{ displayLineNumber?: boolean }>`
  ${(props) =>
    props.displayLineNumber
      ? css`
          color: #ffffffa3;
        `
      : css`
          color: #000;
        `}
  font-family: Ubuntu;
  font-size: 12px;
  padding: 2px;
  padding-right: 10px;
`;

const Line = styled.span`
  color: #fff;
  font-family: Ubuntu;
  font-size: 12px;
  padding: 2px;
  padding-left: 10px;
  display: flex;
  flex-direction: row;
  align-items: center;
  height: 20px;
  &:hover {
    background-color: #191919;
  }
`;

interface LogProps {
  lines: string[];
  displayLineNumbers?: boolean;
}

const Log: FC<LogProps> = ({ displayLineNumbers, lines }) => {
  return (
    <Container>
      {lines.map((line, i) => (
        <Line key={i}>
          <LineNumber displayLineNumber={displayLineNumbers}>
            {i + 1}
          </LineNumber>
          <div>{line}</div>
        </Line>
      ))}
    </Container>
  );
};

Log.defaultProps = {
  displayLineNumbers: true,
};

export default Log;
