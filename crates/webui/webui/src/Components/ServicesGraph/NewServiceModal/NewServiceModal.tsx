import styled from "@emotion/styled";
import { Input } from "baseui/input";
import { Modal, ModalBody } from "baseui/modal";
import { FC, useState } from "react";
import { Stack, ChevronRight } from "@styled-icons/octicons";
import { Terminal } from "@styled-icons/remix-fill";
import { Database2 } from "@styled-icons/remix-line";
import Templates from "./Templates";
import Databases from "./Databases";

const Action = styled.div`
  height: 50px;
  display: flex;
  align-items: center;
  padding-left: 20px;
  padding-right: 20px;
  cursor: pointer;
  font-size: 15px;
  &:hover {
    color: #630be2;
    background-color: #fbfbfb;
  }
`;

const CardContent = styled.div`
  border-top: 1px solid #eaeaea;
  min-height: 50px;
  max-height: 300px;
  overflow-y: auto;
  overflow-x: hidden;
`;

export type NewServiceModalProps = {
  isOpen: boolean;
  onClose: () => void;
  templates: any[];
  databases: any[];
};

const NewServiceModal: FC<NewServiceModalProps> = ({
  isOpen,
  onClose,
  templates,
  databases,
}) => {
  const [tab, setTab] = useState<"templates" | "databases" | null>(null);
  const _onClose = () => {
    onClose();
    setTab(null);
  };

  return (
    <Modal
      isOpen={isOpen}
      onClose={_onClose}
      overrides={{
        Root: {
          style: {
            zIndex: 3,
          },
        },
      }}
    >
      <ModalBody>
        <Input
          placeholder="Create a new service"
          overrides={{
            Root: {
              style: () => ({
                border: "none",
                borderRadius: "0px",
              }),
            },
            InputContainer: {
              style: () => ({
                backgroundColor: "#fff",
              }),
            },
          }}
        />
        <CardContent>
          {tab === null && (
            <>
              <Action onClick={() => setTab("templates")}>
                <Stack color="#ff0a80" size={20} style={{ marginRight: 15 }} />
                <div>Template</div>
                <ChevronRight
                  size={20}
                  style={{ marginLeft: "auto" }}
                  color="#000"
                />
              </Action>
              <Action onClick={() => setTab("databases")}>
                <Database2
                  color="#ff0a80"
                  size={20}
                  style={{ marginRight: 15 }}
                />
                <div>Database</div>
                <ChevronRight
                  size={20}
                  style={{ marginLeft: "auto" }}
                  color="#000"
                />
              </Action>
              <Action>
                <Terminal
                  color="#ff0a80"
                  size={20}
                  style={{ marginRight: 15 }}
                />
                <div>Empty Service</div>
              </Action>
            </>
          )}
          {tab === "templates" && <Templates data={templates} />}
          {tab === "databases" && <Databases data={databases} />}
        </CardContent>
      </ModalBody>
    </Modal>
  );
};

export default NewServiceModal;
