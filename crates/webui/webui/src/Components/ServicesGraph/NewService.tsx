import { FC, useState } from "react";
import { Command } from "@styled-icons/boxicons-regular";
import { ArrowRightShort, PlusCircleFill } from "@styled-icons/bootstrap";
import styled from "@emotion/styled";
import { useHotkeys } from "react-hotkeys-hook";
import NewServiceModal from "./NewServiceModal";

const Row = styled.div`
  display: flex;
  flex-direction: row;
  align-items: center;
  margin-bottom: 10px;
  font-size: 14px;
  color: #31075598;
`;

const Container = styled.div`
  width: 200px;
  height: 100px;
  border: 2px solid #b03aff;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  background-color: #fff;
  position: absolute;
  border-radius: 10px;
  top: calc(50vh - 50px);
  left: calc(50vw - 100px);
  cursor: pointer;
  z-index: 2;
`;

const Wrapper = styled.div`
  display: flex;
  flex-direction: row;
`;

const NewService: FC = () => {
  const [showModal, setShowModal] = useState(false);
  useHotkeys("ctrl+k", () => setShowModal(true));
  return (
    <>
      <Container onClick={() => setShowModal(true)}>
        <Wrapper>
          <PlusCircleFill
            size={20}
            color="#b03aff"
            style={{ marginRight: 10, marginTop: 2 }}
          />
          <div>
            <div>Add a Service</div>
            <Row>
              <Command size={20} />
              <div>K</div>
              <ArrowRightShort
                size={20}
                style={{ marginLeft: 3, marginRight: 3 }}
              />
              <div>New Service</div>
            </Row>
          </div>
        </Wrapper>
      </Container>
      <NewServiceModal isOpen={showModal} onClose={() => setShowModal(false)} />
    </>
  );
};

export default NewService;
