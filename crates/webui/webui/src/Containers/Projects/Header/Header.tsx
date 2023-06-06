import styled from "@emotion/styled";
import { Button } from "baseui/button";
import { Plus } from "baseui/icon";
import { FC } from "react";
import { Command } from "@styled-icons/boxicons-regular";
import { useHotkeys } from "react-hotkeys-hook";

const Wrapper = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
`;

export type HeaderProps = {
  onCreateProject: () => void;
};

const Header: FC<HeaderProps> = ({ onCreateProject }) => {
  useHotkeys("ctrl+n", (e) => {
    e.preventDefault();
    onCreateProject();
  });
  return (
    <>
      <Wrapper>
        <h2>Projects</h2>
        <Button
          onClick={onCreateProject}
          startEnhancer={() => (
            <div
              style={{
                display: "flex",
                flexDirection: "row",
                alignItems: "center",
              }}
            >
              <Command size={15} color="#630be2" />
              <div>N</div>
            </div>
          )}
          overrides={{
            BaseButton: {
              style: {
                height: "35px",
                width: "150px",
                fontSize: "14px",
                padding: "0px",
                backgroundColor: "#fff",
                color: "#630be2",
                fontFamily: "RockfordSansMedium",
                marginRight: "10px",
                borderRadius: "3px",
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
          New Project
        </Button>
      </Wrapper>
    </>
  );
};

export default Header;
