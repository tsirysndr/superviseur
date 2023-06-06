import { Button } from "baseui/button";
import { FC, useState } from "react";
import styled from "@emotion/styled";
import { StopFill } from "@styled-icons/bootstrap/StopFill";
import { Reload } from "@styled-icons/ionicons-outline/Reload";
import { Play } from "@styled-icons/fa-solid/Play";
import { Spinner } from "baseui/spinner";
import { Command } from "@styled-icons/boxicons-regular";
import { useHotkeys } from "react-hotkeys-hook";
import NewServiceModal from "../NewServiceModal";

const State = styled.div`
  margin-right: 15px;
  color: #630be2;
`;

const StateRow = styled.div`
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;
`;

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
  starting: boolean;
  stopping: boolean;
}

const Actions: FC<ActionsProps> = (props) => {
  const {
    onStart,
    onStop,
    onRestart,
    allServicesAreRunning,
    starting,
    stopping,
  } = props;
  const [showModal, setShowModal] = useState(false);
  useHotkeys("ctrl+k", () => setShowModal(true));
  return (
    <Container>
      <Button
        onClick={() => setShowModal(true)}
        startEnhancer={() => (
          <div
            style={{
              color: "#630be2",
              display: "flex",
              flexDirection: "row",
              alignItems: "center",
            }}
          >
            <Command size={15} color="#630be2" />
            <div>K</div>
          </div>
        )}
        overrides={{
          BaseButton: {
            style: {
              height: "30px",
              width: "150px",
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
        New Service
      </Button>
      {!allServicesAreRunning && !starting && !stopping && (
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
      {allServicesAreRunning && !starting && !stopping && (
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
      {starting && (
        <StateRow>
          <State>Starting</State>
          <Spinner $size={"18px"} $borderWidth="3px" $color="#630be2" />
        </StateRow>
      )}
      {stopping && (
        <StateRow>
          <State>Stopping</State>
          <Spinner $size={"18px"} $borderWidth="3px" $color="#630be2" />
        </StateRow>
      )}
      <NewServiceModal isOpen={showModal} onClose={() => setShowModal(false)} />
    </Container>
  );
};

export default Actions;
