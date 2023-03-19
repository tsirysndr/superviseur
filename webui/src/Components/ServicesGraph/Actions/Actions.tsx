import { Button } from "baseui/button";
import { FC } from "react";
import styled from "@emotion/styled";
import { StopFill } from "@styled-icons/bootstrap/StopFill";
import { Reload } from "@styled-icons/ionicons-outline/Reload";
import { Play } from "@styled-icons/fa-solid/Play";

const Container = styled.div`
  position: absolute;
  right: 20px;
  top: 20px;
  z-index: 1;
`;

export interface ActionsProps {
  onStart: () => void;
  onStop: () => void;
  onRestart: () => void;
  allServicesAreRunning: boolean;
}

const Actions: FC<ActionsProps> = (props) => {
  const { onStart, onStop, onRestart, allServicesAreRunning } = props;
  return (
    <Container>
      {!allServicesAreRunning && (
        <Button
          onClick={onStart}
          startEnhancer={() => <Play size={16} color="#630be2" />}
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
                  opacity: 0.6,
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
      )}
      {allServicesAreRunning && (
        <>
          <Button
            onClick={onStop}
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
                    opacity: 0.6,
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
            onClick={onRestart}
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
                    opacity: 0.8,
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
    </Container>
  );
};

export default Actions;
