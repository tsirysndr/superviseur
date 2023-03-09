import { css } from "@emotion/react";
import styled from "@emotion/styled";
import { Button } from "baseui/button";
import { FC } from "react";
import { ServiceStatus } from "../../Types/ServiceStatus";
import { StopFill } from "@styled-icons/bootstrap/StopFill";
import { Reload } from "@styled-icons/ionicons-outline/Reload";
import { Play } from "@styled-icons/fa-solid/Play";

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
  color: #630be2;
`;

const StatusValue = styled.div`
  flex: 1;
`;

const Actions = styled.div`
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: flex-end;
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
  const status = statuses.find((status) => status.name === "Active")?.status;
  return (
    <Container>
      <Actions>
        {status?.startsWith("Running") && (
          <>
            <Button
              startEnhancer={() => <StopFill size={16} color="#630be2" />}
              overrides={{
                BaseButton: {
                  style: {
                    height: "30px",
                    width: "80px",
                    fontSize: "12px",
                    padding: "0px",
                    backgroundColor: "#fff",
                    color: "#630be2",
                    fontFamily: "RockfordSansMedium",
                    marginRight: "10px",
                    borderRadius: "2px",
                    border: "2px solid #630be2",
                    ":hover": {
                      backgroundColor: "#fff",
                    },
                  },
                },
                StartEnhancer: {
                  style: {
                    marginRight: "8px",
                  },
                },
              }}
            >
              Stop
            </Button>
            <Button
              startEnhancer={() => <Reload size={14} color="#fff" />}
              overrides={{
                BaseButton: {
                  style: {
                    height: "30px",
                    width: "80px",
                    fontSize: "12px",
                    padding: "0px",
                    fontFamily: "RockfordSansMedium",
                    backgroundColor: "#630be2",
                    color: "#fff",
                    borderRadius: "2px",
                    ":hover": {
                      backgroundColor: "#630be2",
                    },
                  },
                },
                StartEnhancer: {
                  style: {
                    marginRight: "8px",
                  },
                },
              }}
            >
              Restart
            </Button>
          </>
        )}
        {!status?.startsWith("Running") && (
          <>
            <Button
              startEnhancer={() => <Play size={14} color="#fff" />}
              overrides={{
                BaseButton: {
                  style: {
                    height: "30px",
                    width: "80px",
                    fontSize: "12px",
                    padding: "0px",
                    fontFamily: "RockfordSansMedium",
                    backgroundColor: "#630be2",
                    color: "#fff",
                    borderRadius: "2px",
                    ":hover": {
                      backgroundColor: "#630be2",
                    },
                  },
                },
                StartEnhancer: {
                  style: {
                    marginRight: "8px",
                  },
                },
              }}
            >
              Start
            </Button>
          </>
        )}
      </Actions>
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
