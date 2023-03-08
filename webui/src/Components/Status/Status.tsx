import { css } from "@emotion/react";
import styled from "@emotion/styled";
import { FC } from "react";
import { ServiceStatus } from "../../Types/ServiceStatus";

const Container = styled.div``;

const StatusTable = styled.div`
  margin-top: 20px;
`;

const StatusRow = styled.div`
  display: flex;
  align-items: flex-start;
  flex-direction: row;
  margin-bottom: 10px;
  flex: 1;
`;

const StatusName = styled.div`
  width: 80px;
  display: flex;
  justify-content: flex-end;
  margin-right: 20px;
  color: #b03aff;
`;

const StatusValue = styled.div`
  flex: 1;
`;

const CurrentStatus = styled.span<{ color: string }>`
  ${({ color }) =>
    css`
      color: ${color};
    `}
  margin-right: 5px;
`;

const parseStatus = (status: ServiceStatus) => {
  if (status.status.startsWith("Running")) {
    return (
      <StatusValue>
        <CurrentStatus color="#00e667">Running</CurrentStatus>{" "}
        {status.status.replace("Running", "")}
      </StatusValue>
    );
  }
  return <StatusValue>{status.status}</StatusValue>;
};

export interface StatusProps {
  statuses: ServiceStatus[];
}

const Status: FC<StatusProps> = ({ statuses }) => {
  return (
    <Container>
      <StatusTable>
        {statuses.map((status) => (
          <StatusRow>
            <StatusName>{status.name}:</StatusName>
            {status.name !== "Active" && (
              <StatusValue>{status.status}</StatusValue>
            )}
            {status.name === "Active" && parseStatus(status)}
          </StatusRow>
        ))}
      </StatusTable>
    </Container>
  );
};

Status.defaultProps = {
  statuses: [],
};

export default Status;
